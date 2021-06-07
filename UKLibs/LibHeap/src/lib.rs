/*! # Heap Management Library
 *
 * Implements a `no_std` heap management library that could be used both in
 * Kernel and user spaces.
 *
 * The crate implements a concurrency safe `Heap` for the userspace called
 * `OsLockedHeap` that is simply an `Heap` instance wrapped in a
 * `LibApi::objs::impls::Mutex`
 */

#![no_std]
#![feature(const_fn_trait_bound,
           const_fn_fn_ptr_basics,
           fn_traits,
           unboxed_closures,
           once_cell)]

use core::{
    alloc::Layout,
    cmp::max,
    ptr::NonNull
};

use linked_list_allocator::hole::HoleList;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use crate::{
    consts::SLAB_THRESHOLD,
    slab::Slab
};

pub mod consts;
pub mod locked;
pub mod slab;

/**
 * Callback used by the `Heap` to obtain more memory when runs out.
 *
 * The function must return the starting address of the new virtual area
 * allocated and the aligned size of his minimum allocation block
 */
pub type HeapMemorySupplier = fn(requested_size: usize) -> Option<(usize, usize)>;

/**
 * Lists the currently available allocators of the `Heap` manager
 */
#[repr(u8)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum Allocator {
    Slab64Bytes,
    Slab128Bytes,
    Slab256Bytes,
    Slab512Bytes,
    Slab1024Bytes,
    Slab2048Bytes,
    Slab4096Bytes,
    Slab8192Bytes,
    LinkedList
}

impl Allocator {
    const VARIANTS: [Allocator; 9] = [Self::Slab64Bytes,
                                      Self::Slab128Bytes,
                                      Self::Slab256Bytes,
                                      Self::Slab512Bytes,
                                      Self::Slab1024Bytes,
                                      Self::Slab2048Bytes,
                                      Self::Slab4096Bytes,
                                      Self::Slab8192Bytes,
                                      Self::LinkedList];

    /**
     * Returns the best Allocator variant to serve the given request.
     *
     * The function fallbacks to `Allocator::LinkedList` when a
     * `Allocator::SlabXX` is chosen but the difference between the
     * `layout.size()` and the block size of the `Slab` exceeds the
     * `SLAB_THRESHOLD`
     */
    pub fn for_layout(layout: Layout) -> Allocator {
        let mut chosen = Allocator::LinkedList;
        for allocator in Self::VARIANTS.iter() {
            let alloc_size = allocator.max_alloc_size();

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
                chosen = allocator.clone();
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

    /**
     * Returns the minimum allocation block size for the selected variant
     */
    pub fn min_alloc_size(&self) -> usize {
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

    /**
     * Returns the maximum allocation block size for the selected variant
     */
    pub fn max_alloc_size(&self) -> usize {
        match self {
            Allocator::LinkedList => usize::MAX,
            _ => self.min_alloc_size()
        }
    }

    /**
     * Returns the minimum amount of memory that each allocator wants as
     * extension
     */
    pub fn min_managed_size(&self) -> usize {
        if *self != Self::LinkedList {
            self.min_alloc_size() * 4
        } else {
            /* for the linked list request at least 16KB */
            16 * 1024
        }
    }

    /**
     * Calculates the minimum memory consumption requested by an
     * `heap::Heap` instance to become functional
     */
    pub fn cumulative_min_managed_size() -> usize {
        let mut size = 0;
        for allocator in Self::VARIANTS.iter() {
            size += allocator.min_managed_size();
        }
        size
    }

    /**
     * Returns an `Iterator` to the variants
     */
    pub fn iter() -> impl Iterator<Item = &'static Self> {
        Self::VARIANTS.iter()
    }
}

/**
 * Multi strategy heap manager capable of use as `global_allocator` in
 * single threaded environments.
 *
 * For thread-safe implementation use `OsLockedHeap`.
 *
 * Internally two main allocation strategies are used:
 * * `Slab` - fixed size block allocation, used for little allocation
 *   requests (under 8KiB) and whenever the threshold doesn't exceed
 *   `SLAB_THRESHOLD`.
 * * `LinkedList` - classic UNIX chunk allocation, used for allocation
 *   requests above the 8KiB and when slab allocation exceed the threshold.
 *
 * Slab allocator comes from the internal `heap::slab` module, linked list
 * allocator comes from Philipp Oppermann's `linked_list_allocator` crate
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
    /**
     * Constructs a new `Heap` which immediately uses the given
     * `HeapMemorySupplier` to become operative
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
        let mut allocators = Allocator::iter();

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

    /**
     * Dispatches the given memory region to the given `Allocator`
     */
    pub unsafe fn add_region(&mut self, addr: usize, size: usize, allocator: Allocator) {
        /* check whether the caller wants to add memory to a slab allocator, in that
         * case check whether the given size is aligned with the Allocator's minimum
         * allocation size, otherwise give the wasted region to the linked_list
         * allocator that not necessarily needs a big alignment, just a minimum size
         */
        let mut size = size;
        if allocator != Allocator::LinkedList {
            let exceeding = size % allocator.min_alloc_size();

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

    /**
     * Allocates new memory that fits the given `Layout` request.
     *
     * Returns a `Result` variant with a `NonNull<u8>` pointer when `Ok` or
     * `Err` when the used allocator runs out of memory.
     *
     * The allocation may be served by one of the slab sub-allocators, which
     * performs an allocation in `O(1)`, or by the
     * `Allocator::LinkedList`, which performs allocation in `O(n)`.
     *
     * To know which allocator will be used call `Allocator::for_layout()`
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

    /**
     * Makes the given memory available again for further allocations.
     *
     * The request, as for allocation, may be served by one of the slab
     * sub-allocators, which performs the operation in `O(1)`, or by the
     * `Allocator::LinkedList`, which performs the de-allocation in
     * `O(n)`.
     *
     * To know which allocator will be used call `Allocator::for_layout()`
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

    /**
     * Returns the size of the current managed area in bytes
     */
    pub fn managed_mem(&self) -> usize {
        self.m_managed_mem
    }

    /**
     * Returns the currently allocated size in bytes
     */
    pub fn allocated_mem(&self) -> usize {
        self.m_allocated_mem
    }

    /**
     * Returns the available memory amount
     */
    pub fn free_memory(&self) -> usize {
        self.m_managed_mem - self.m_allocated_mem
    }

    /**
     * Returns the `Slab` that allocates the next `Allocator` sizes
     */
    unsafe fn construct_slab(addr: &mut usize, allocator: &Allocator) -> Slab {
        let slab =
            Slab::new(*addr, allocator.min_managed_size(), allocator.min_alloc_size());
        *addr += allocator.min_managed_size();
        slab
    }
}
