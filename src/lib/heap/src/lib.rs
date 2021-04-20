/*! # Heap Manager
 *
 * Implements a [`no_std`] heap manager library that could be used both in
 * kernel and user spaces.
 *
 * The crate implements a concurrency safe [`Heap`] for the userspace called
 * [`LockedHeap`] that is simply an [`Heap`] instance wrapped in a [`Mutex`]
 *
 * [`no_std`]: https://doc.rust-lang.org/1.7.0/book/no-stdlib.html
 * [`LockedHeap`]: locked/struct.LockedHeap.html
 * [`Heap`]: struct.Heap.html
 * [`Mutex`]: /api/objs/impls/type.Mutex.html
 */

#![no_std]
#![feature(const_fn)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(once_cell)]

#[macro_use]
extern crate macros;

use core::{alloc::Layout, cmp::max, ptr::NonNull};

use linked_list_allocator::hole::HoleList;

use crate::{consts::SLAB_THRESHOLD, slab::Slab};

pub mod consts;
pub mod locked;
pub mod slab;

/** # Low Level Memory Supplier
 *
 * Represents the callback used by the [`Heap`] to obtain more memory from
 * the operating system (if the [`Heap`] is used in userspace) or from the
 * memory manager (if it is used into the kernel).
 *
 * The function must return the starting address of the new virtual area
 * allocated and the aligned size of his minimum allocation block
 *
 * [`Heap`]: /heap/struct.Heap.html
 */
pub type HeapMemorySupplier = fn(requested_size: usize) -> Option<(usize, usize)>;

c_handy_enum! {
    /** # Sub Allocators Identifier
     *
     * Lists the currently available allocators of the [`Heap`] manager.
     *
     * [`Heap`]: /heap/struct.Heap.html
     */
    pub enum Allocator: u8 {
        Slab64Bytes   = 0,
        Slab128Bytes  = 1,
        Slab256Bytes  = 2,
        Slab512Bytes  = 3,
        Slab1024Bytes = 4,
        Slab2048Bytes = 5,
        Slab4096Bytes = 6,
        Slab8192Bytes = 7,
        LinkedList    = 8,
    }
}

impl Allocator {
    /** # Constructs an `Allocator` from a `Layout`
     *
     * Returns the best Allocator variant to serve the given request.
     *
     * The associated function takes care that when is chosen a [`Slab`]
     * allocator the difference between the [`layout.size()`] and the block
     * size of the [`Slab`] doesn't exceed the [`SLAB_THRESHOLD`]
     *
     * [`layout.size()`]: https://doc.rust-lang.org/std/alloc/struct.Layout.html#method.size
     * [`Slab`]: /heap/slab/struct.Slab.html
     * [`SLAB_THRESHOLD`]: /heap/consts/constant.SLAB_THRESHOLD.html
     */
    pub fn for_layout(layout: Layout) -> Allocator {
        let mut chosen = Allocator::LinkedList;
        for allocator in Self::iter_all() {
            let alloc_size = allocator.max_block_size();

            /* allocators that doesn't have the requested size and the requested
             * alignment are discarded before the check of the threshold
             */
            if alloc_size < layout.size() || alloc_size < layout.align() {
                continue;
            }

            /* check now whether the max allocation size of the current slab allocator
             * is less than the SLAB_THRESHOLD
             */
            if alloc_size - layout.size() < SLAB_THRESHOLD {
                chosen = allocator
            } else {
                /* break if the previous condition is not respected, because the next
                 * slab allocator (if not directly the linked_list) doubles the size of
                 * the current allocator, so there are no chances that the condition is
                 * respected.
                 *
                 * Breaking makes the chosen allocator the linked_list
                 */
                break;
            }
        }
        chosen
    }

    /** Returns the minimum allocation block size for the selected variant
     */
    pub fn min_block_size(&self) -> usize {
        match self {
            Self::Slab64Bytes => 64,
            Self::Slab128Bytes => 128,
            Self::Slab256Bytes => 256,
            Self::Slab512Bytes => 512,
            Self::Slab1024Bytes => 1024,
            Self::Slab2048Bytes => 2048,
            Self::Slab4096Bytes => 4096,
            Self::Slab8192Bytes => 8192,
            Self::LinkedList => HoleList::min_size()
        }
    }

    /** Returns the maximum allocation block size for the selected variant
     */
    pub fn max_block_size(&self) -> usize {
        match self {
            Allocator::LinkedList => usize::MAX,
            _ => self.min_block_size()
        }
    }

    /** Returns the minimum amount of memory that each allocator wants as
     * extension
     */
    pub fn min_managed_size(&self) -> usize {
        if *self != Self::LinkedList {
            self.min_block_size() * 2
        } else {
            /* for the linked list request at least 16KB */
            16 * 1024
        }
    }

    /** # Cumulative minimum managed memory
     *
     * Calculates the minimum memory consumption requested by an [`Heap`]
     * instance to become functional
     *
     * [`Heap`]: /heap/struct.Heap.html
     */
    pub fn cumulative_min_managed_size() -> usize {
        let mut size = 0;
        for allocator in Self::iter_all() {
            size += allocator.min_managed_size();
        }
        size
    }
}

/** # Heap Manager
 *
 * Defines a multi strategy heap manager that could be used as
 * [`global_allocator`] in single threaded environments.
 *
 * For Thread safe implementation look [here]
 *
 * Internally two main allocation strategies area used:
 * * `Slab` - fixed size block allocation, used for little allocation
 *   requests (under 8KiB) and whenever the threshold doesn't exceed
 *   [`SLAB_THRESHOLD`].
 * * `LinkedList` - classic UNIX chunk allocation, used for allocation
 *   requests above the 8KiB and when slab allocation exceed the threshold.
 *
 * Slab allocator comes from the internal [`slab`] module, linked list
 * allocator comes from Philipp Oppermann's [`linked_list_allocator`] crate
 *
 * [`global_allocator`]: https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/global-allocators.html
 * [here]: /heap/locked/raw/struct.RawLazyLockedHeap.html
 * [`SLAB_THRESHOLD`]: /heap/consts/constant.SLAB_THRESHOLD.html
 * [`slab`]: /heap/slab/struct.Slab.html
 * [`linked_list_allocator`]: https://docs.rs/linked_list_allocator/0.8.6/linked_list_allocator/
 */
pub struct Heap {
    m_slab_64: Slab,
    m_slab_128: Slab,
    m_slab_256: Slab,
    m_slab_512: Slab,
    m_slab_1024: Slab,
    m_slab_2048: Slab,
    m_slab_4096: Slab,
    m_slab_8192: Slab,
    m_linked_list: HoleList,
    m_mem_supplier: HeapMemorySupplier,
    m_allocated_mem: usize,
    m_managed_mem: usize
}

impl Heap {
    /** # Constructs a new `Heap`
     *
     * It will manage the memory returned by the [`HeapMemoryGiver`]
     * callback given.
     *
     * [`HeapMemoryGiver`]: /heap/type.HeapMemoryGiver.html
     */
    pub unsafe fn new(mem_supplier: HeapMemorySupplier) -> Self {
        /* immediately ask to the given supplier the minimum amount of memory to make
         * operative the instance in creation. To the supplier is requested
         * the cumulative minimum size, that is the sum of all the min_alloc_size()
         * returns
         */
        let (mut addr, aligned_size) =
            mem_supplier(Allocator::cumulative_min_managed_size()).unwrap();

        /* construct an Allocator iterator to construct the slabs in sequence */
        let mut allocators = Allocator::iter_all();

        Self { m_slab_64: Self::construct_slab(&mut addr, allocators.next().unwrap()),
               m_slab_128: Self::construct_slab(&mut addr, allocators.next().unwrap()),
               m_slab_256: Self::construct_slab(&mut addr, allocators.next().unwrap()),
               m_slab_512: Self::construct_slab(&mut addr, allocators.next().unwrap()),
               m_slab_1024: Self::construct_slab(&mut addr, allocators.next().unwrap()),
               m_slab_2048: Self::construct_slab(&mut addr, allocators.next().unwrap()),
               m_slab_4096: Self::construct_slab(&mut addr, allocators.next().unwrap()),
               m_slab_8192: Self::construct_slab(&mut addr, allocators.next().unwrap()),
               m_linked_list: HoleList::new(addr,
                                            Allocator::LinkedList.min_managed_size()),
               m_mem_supplier: mem_supplier,
               m_allocated_mem: 0,
               m_managed_mem: aligned_size }
    }

    /** # Adds a new memory region
     *
     * The given memory region is dispatched to the given [`Allocator`].
     *
     * * `mem` must be aligned to [`ADDRESS_ALIGNMENT`]
     * * `size` must be at least [`SUB_MIN_SIZE`]
     *
     * [`Allocator`]: /heap/enum.Allocator.html
     * [`ADDRESS_ALIGNMENT`]: /heap/consts/constant.ADDRESS_ALIGNMENT.html
     * [`SUB_MIN_SIZE`]: /heap/consts/constant.SUB_MIN_SIZE.html
     */
    pub unsafe fn add_region(&mut self, addr: usize, size: usize, allocator: Allocator) {
        /* check whether the caller wants to add memory to a slab allocator, in that
         * case check whether the given size is aligned with the Allocator's minimum
         * allocation size, otherwise give the wasted region to the linked_list
         * allocator that not necessarily needs a big alignment, just a minimum size
         */
        let mut size = size;
        if allocator != Allocator::LinkedList {
            let exceeding = size % allocator.min_block_size();

            /* if the remaining subregion is at least the minimum required size of the
             * linked list give it to him, otherwise leave the remaining space
             * unmanaged
             */
            if exceeding != 0 && exceeding >= HoleList::min_size() {
                size = size - exceeding;
                self.add_region(addr + size, exceeding, Allocator::LinkedList);
            }
        }

        /* dispatch the memory to the requested allocator */
        match allocator {
            Allocator::Slab64Bytes => self.m_slab_64.extend(addr, size),
            Allocator::Slab128Bytes => self.m_slab_128.extend(addr, size),
            Allocator::Slab256Bytes => self.m_slab_256.extend(addr, size),
            Allocator::Slab512Bytes => self.m_slab_512.extend(addr, size),
            Allocator::Slab1024Bytes => self.m_slab_1024.extend(addr, size),
            Allocator::Slab2048Bytes => self.m_slab_2048.extend(addr, size),
            Allocator::Slab4096Bytes => self.m_slab_4096.extend(addr, size),
            Allocator::Slab8192Bytes => self.m_slab_8192.extend(addr, size),
            Allocator::LinkedList => {
                let ptr = NonNull::new_unchecked(addr as *mut u8);
                let layout = Layout::from_size_align_unchecked(size, 1);
                self.m_linked_list.deallocate(ptr, layout);
            }
        }

        /* increase the total heap managed memory counter */
        self.m_managed_mem += size;
    }

    /** # Requests new memory
     *
     * Allocates new memory that fits the given [`Layout`] request.
     *
     * Returns a [`Result`] variant with a [`NonNull<u8>`] when [`Ok`] or
     * [`Err`] when the used allocator runs out of memory.
     *
     * The allocation may be served by one of the slab sub-allocators, which
     * performs an allocation in `O(1)`, or by the
     * [`Allocator::LinkedList`], which performs allocation in `O(n)`.
     *
     * To know which allocator will be used call [`layout_to_allocator()`].
     *
     * [`Layout`]: https://doc.rust-lang.org/std/alloc/struct.Layout.html
     * [`Result`]: https://doc.rust-lang.org/std/result/
     * [`Ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
     * [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
     * [`NonNull<u8>`]: https://doc.rust-lang.org/std/ptr/struct.NonNull.html
     * [`Allocator::LinkedList`]:
     * /heap/enum.Allocator.html#variant.LinkedList
     * [`layout_to_allocator()`]:
     * /heap/struct.Heap.html#method.layout_to_allocator
     */
    pub fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        let allocator = Allocator::for_layout(layout);
        let res = match allocator {
            Allocator::Slab64Bytes => self.m_slab_64.alloc_block(),
            Allocator::Slab128Bytes => self.m_slab_128.alloc_block(),
            Allocator::Slab256Bytes => self.m_slab_256.alloc_block(),
            Allocator::Slab512Bytes => self.m_slab_512.alloc_block(),
            Allocator::Slab1024Bytes => self.m_slab_1024.alloc_block(),
            Allocator::Slab2048Bytes => self.m_slab_2048.alloc_block(),
            Allocator::Slab4096Bytes => self.m_slab_4096.alloc_block(),
            Allocator::Slab8192Bytes => self.m_slab_8192.alloc_block(),
            Allocator::LinkedList => {
                self.m_linked_list.allocate_first_fit(layout).map(|pair| pair.0)
            },
        };

        /* increase the currently allocated memory counter if okay */
        if res.is_ok() {
            self.m_allocated_mem += layout.size();
            res
        } else {
            /* if the allocation failed means that the allocator used have exhausted
             * the memory, so use the supplier to obtain more from the underling system
             */
            let size = max(allocator.min_managed_size(), layout.size());
            if let Some(suppl_res) = (self.m_mem_supplier)(size) {
                /* add the returned region to the allocator designed */
                unsafe {
                    self.add_region(suppl_res.0, suppl_res.1, allocator);
                }

                /* retry the allocation */
                self.allocate(layout)
            } else {
                res
            }
        }
    }

    /** # Deallocates memory
     *
     * Makes the given memory available again for further allocations.
     *
     * The request, as for allocation, may be served by one of the slab
     * sub-allocators, which performs the operation in `O(1)`, or by the
     * [`Allocator::LinkedList`], which performs the de-allocation in
     * `O(n)`.
     *
     * To know which allocator will be used call [`layout_to_allocator()`].
     *
     * [`Allocator::LinkedList`]:
     * /heap/enum.Allocator.html#variant.LinkedList
     * [`layout_to_allocator()`]:
     * /heap/struct.Heap.html#method.layout_to_allocator
     */
    pub unsafe fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout) {
        match Allocator::for_layout(layout) {
            Allocator::Slab64Bytes => self.m_slab_64.dealloc_block(ptr),
            Allocator::Slab128Bytes => self.m_slab_128.dealloc_block(ptr),
            Allocator::Slab256Bytes => self.m_slab_256.dealloc_block(ptr),
            Allocator::Slab512Bytes => self.m_slab_512.dealloc_block(ptr),
            Allocator::Slab1024Bytes => self.m_slab_1024.dealloc_block(ptr),
            Allocator::Slab2048Bytes => self.m_slab_2048.dealloc_block(ptr),
            Allocator::Slab4096Bytes => self.m_slab_4096.dealloc_block(ptr),
            Allocator::Slab8192Bytes => self.m_slab_8192.dealloc_block(ptr),
            Allocator::LinkedList => {
                self.m_linked_list.deallocate(ptr, layout);
            }
        }

        /* decrease the currently allocated counter */
        self.m_allocated_mem -= layout.size();
    }

    /** Returns the size of the current managed area in bytes
     */
    pub fn managed_mem(&self) -> usize {
        self.m_managed_mem
    }

    /** Returns the currently allocated size in bytes
     */
    pub fn allocated_mem(&self) -> usize {
        self.m_allocated_mem
    }

    /** Returns the available memory amount
     */
    pub fn free_memory(&self) -> usize {
        self.m_managed_mem - self.m_allocated_mem
    }

    /** # Constructs the next `Slab`
     *
     * Returns the [`Slab`] that allocates the next [`Allocator`] sizes
     *
     * [`Slab`]: /heap/slab/struct.Slab.html
     * [`Allocator`]: /heap/enum.Allocator.html
     */
    unsafe fn construct_slab(addr: &mut usize, allocator: Allocator) -> Slab {
        let slab =
            Slab::new(*addr, allocator.min_managed_size(), allocator.min_block_size());
        *addr += allocator.min_managed_size();
        slab
    }
}
