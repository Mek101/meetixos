/*! # Heap Management Library
 *
 * Implements a `no_std` heap manager that could be used in kernel and
 * userland
 */

#![no_std]
#![feature(once_cell)]

use core::{
    alloc::Layout,
    cmp::max,
    ptr::NonNull
};

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use crate::{
    linked_list::LinkedList,
    slab::Slab
};

pub mod linked_list;
pub mod locked;
pub mod slab;

/**
 * Maximum amount of bytes that can be wasted using slab allocation,
 * exceeded the value the allocation request rollbacks to linked list
 * allocation
 */
pub const SLAB_THRESHOLD: usize = 512;

/**
 * Callback used by the `Heap` to obtain more memory when runs out.
 *
 * The function must return the starting address of the new virtual area
 * allocated and the up-aligned size of his minimum allocation block
 */
pub type HeapMemorySupplier = fn(requested_size: usize) -> Option<(NonNull<u8>, usize)>;

/**
 * Multi strategy heap manager capable of use as `global_allocator` in
 * single threaded environments.
 *
 * Internally two main allocation strategies are used:
 * * `Slab` - fixed size block allocation, used for little allocation
 *   requests (under 8KiB) and whenever the threshold doesn't exceed
 *   `SLAB_THRESHOLD`.
 * * `LinkedList` - classic UNIX chunk allocation, used for allocation
 *   requests above the 8KiB and when slab allocation exceed the threshold.
 */
pub struct Heap {
    m_slab_64: Slab<64>,
    m_slab_128: Slab<128>,
    m_slab_256: Slab<256>,
    m_slab_512: Slab<512>,
    m_slab_1024: Slab<1024>,
    m_slab_2048: Slab<2048>,
    m_slab_4096: Slab<4096>,
    m_slab_8192: Slab<8192>,
    m_linked_list: LinkedList,
    m_mem_supplier: HeapMemorySupplier,
    m_allocated_mem: usize,
    m_supplier_managed_mem: usize
}

impl Heap {
    /**
     * Initial amount of memory (in bytes) requested to the
     * `HeapMemorySupplier` by the `Heap::new()`
     */
    const INITIAL_REQUESTED_MEM_AMOUNT: usize = Slab::<64>::PREFERRED_EXTEND_SIZE
                                                + Slab::<128>::PREFERRED_EXTEND_SIZE
                                                + Slab::<256>::PREFERRED_EXTEND_SIZE
                                                + Slab::<512>::PREFERRED_EXTEND_SIZE
                                                + Slab::<1024>::PREFERRED_EXTEND_SIZE
                                                + Slab::<2048>::PREFERRED_EXTEND_SIZE
                                                + Slab::<4096>::PREFERRED_EXTEND_SIZE
                                                + Slab::<8192>::PREFERRED_EXTEND_SIZE
                                                + LinkedList::PREFERRED_EXTEND_SIZE;

    /**
     * Constructs an `Heap` which uses the given `HeapMemorySupplier`
     */
    pub unsafe fn new(mem_supplier: HeapMemorySupplier) -> Option<Self> {
        /* obtain from the mem_supplier the initial memory to become operative */
        let (mut next_start_area_addr, up_aligned_area_size) =
            mem_supplier(Self::INITIAL_REQUESTED_MEM_AMOUNT)?;

        let original_start_area_addr = next_start_area_addr;

        /* construct the slab_64 allocator */
        let slab_64 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr += Slab::<64>::PREFERRED_EXTEND_SIZE;

        /* construct the slab_128 allocator */
        let slab_128 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr += Slab::<128>::PREFERRED_EXTEND_SIZE;

        /* construct the slab_256 allocator */
        let slab_256 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr += Slab::<256>::PREFERRED_EXTEND_SIZE;

        /* construct the slab_512 allocator */
        let slab_512 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr += Slab::<512>::PREFERRED_EXTEND_SIZE;

        /* construct the slab_1024 allocator */
        let slab_1024 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr += Slab::<1024>::PREFERRED_EXTEND_SIZE;

        /* construct the slab_2048 allocator */
        let slab_2048 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr += Slab::<2048>::PREFERRED_EXTEND_SIZE;

        /* construct the slab_4096 allocator */
        let slab_4096 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr += Slab::<4096>::PREFERRED_EXTEND_SIZE;

        /* construct the slab_8192 allocator */
        let slab_8192 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr += Slab::<8192>::PREFERRED_EXTEND_SIZE;

        /* construct the linked_list allocator */
        let linked_list =
            LinkedList::new(next_start_area_addr,
                            up_aligned_area_size
                            - (next_start_area_addr - original_start_area_addr) as usize);

        Some(Self { m_slab_64: slab_64,
                    m_slab_128: slab_128,
                    m_slab_256: slab_256,
                    m_slab_512: slab_512,
                    m_slab_1024: slab_1024,
                    m_slab_2048: slab_2048,
                    m_slab_4096: slab_4096,
                    m_slab_8192: slab_8192,
                    m_linked_list: linked_list,
                    m_mem_supplier: mem_supplier,
                    m_allocated_mem: 0,
                    m_supplier_managed_mem: up_aligned_area_size })
    }

    /**
     * Allocates new memory that fits the given `Layout` request.
     *
     * Returns a `NonNull<u8>` pointer when `Some`; `None` when the used
     * allocator runs out of memory and the `HeapMemorySupplier` doesn't
     * return valid memory
     */
    pub fn allocate(&mut self, layout: Layout) -> Option<NonNull<u8>> {
        let sub_heap_allocator = self.allocator_for_layout(&layout);

        /* try to allocate to serve the given layout, if the operation fails try to
         * refill the memory pools using the memory supplier.
         * If the refill fails too return None
         */
        if let Some(alloc_nn_ptr) = sub_heap_allocator.allocate(layout) {
            self.m_allocated_mem += layout.size();
            Some(alloc_nn_ptr)
        } else if self.refill_mem_pool(sub_heap_allocator, &layout) {
            self.allocate(layout)
        } else {
            None
        }
    }

    /**
     * Makes the given memory available again for further allocations
     */
    pub unsafe fn deallocate(&mut self, nn_ptr: NonNull<u8>, layout: Layout) {
        self.allocator_for_layout(&layout).deallocate(nn_ptr, layout);
        self.m_allocated_mem -= layout.size();
    }

    /**
     * Returns the size of the current managed area in bytes
     */
    pub fn supplier_managed_mem(&self) -> usize {
        self.m_supplier_managed_mem
    }

    /**
     * Returns the amount of currently allocated memory in bytes
     */
    pub fn allocated_mem(&self) -> usize {
        self.m_allocated_mem
    }

    /**
     * Returns the amount of currently available memory
     */
    pub fn free_memory(&self) -> usize {
        self.m_supplier_managed_mem - self.m_allocated_mem
    }

    /**
     * Returns the best `SubHeapAllocator` to serve the given `Layout`
     * request.
     *
     * Already catches the slab-threshold and rollbacks the returned
     * `SubHeapAllocator` to the linked-list
     */
    fn allocator_for_layout(&mut self, layout: &Layout) -> &mut dyn SubHeapAllocator {
        /* select the SubHeapAllocator from the given layout size */
        let mut selected_sub_heap_allocator =
            if layout.size() <= 64 && layout.align() <= 64 {
                &self.m_slab_64 as &mut dyn SubHeapAllocator
            } else if layout.size() <= 128 && layout.align() <= 128 {
                &self.m_slab_128 as &mut dyn SubHeapAllocator
            } else if layout.size() <= 256 && layout.align() <= 256 {
                &self.m_slab_256 as &mut dyn SubHeapAllocator
            } else if layout.size() <= 512 && layout.align() <= 512 {
                &self.m_slab_512 as &mut dyn SubHeapAllocator
            } else if layout.size() <= 1024 && layout.align() <= 1024 {
                &self.m_slab_1024 as &mut dyn SubHeapAllocator
            } else if layout.size() <= 2048 && layout.align() <= 2048 {
                &self.m_slab_2048 as &mut dyn SubHeapAllocator
            } else if layout.size() <= 4096 && layout.align() <= 4096 {
                &self.m_slab_4096 as &mut dyn SubHeapAllocator
            } else if layout.size() <= 8192 && layout.align() <= 8192 {
                &self.m_slab_8192 as &mut dyn SubHeapAllocator
            } else {
                &self.m_linked_list as &mut dyn SubHeapAllocator
            };

        /* check for threshold to avoid memory waste */
        if let Some(block_size) = selected_sub_heap_allocator.block_size() {
            /* rollback to linked-list if the selected slab waste too much memory */
            if block_size - layout.size() > SLAB_THRESHOLD {
                selected_sub_heap_allocator =
                    &self.m_linked_list as &mut dyn SubHeapAllocator;
            }
        }
        selected_sub_heap_allocator
    }

    /**
     * Refills the memory pool for the given `SubHeapAllocator`
     */
    fn refill_mem_pool(&mut self,
                       sub_heap_allocator: &mut dyn SubHeapAllocator,
                       layout: &Layout)
                       -> bool {
        /* request to the supplier the maximum amount of memory */
        let mem_amount_to_request =
            max(sub_heap_allocator.preferred_extend_size(), layout.size());

        /* call the memory supplier (which could be the kernel or the something else */
        if let Some((start_area_addr, up_aligned_area_size)) =
            (self.m_mem_supplier)(mem_amount_to_request)
        {
            /* update the managed memory amount */
            self.m_supplier_managed_mem += up_aligned_area_size;

            unsafe {
                /* add the region to the allocator and catch the eventual exceed */
                if let Some((exceeding_area_start_addr, exceeding_area_size)) =
                    sub_heap_allocator.add_region(start_area_addr, up_aligned_area_size)
                {
                    /* put the exceeding area into the linked-list pool */
                    self.m_linked_list
                        .add_region(exceeding_area_start_addr, exceeding_area_size);
                }
            }
            true
        } else {
            false
        }
    }
}

/**
 * Base `Heap` allocator interface
 */
pub trait SubHeapAllocator {
    /**
     * Preferred amount of memory for `add_region()`
     */
    const PREFERRED_EXTEND_SIZE: usize;

    /**
     * Allocate a new chunk of memory to server the given layout
     */
    fn allocate(&mut self, layout: Layout) -> Option<NonNull<u8>>;

    /**
     * Deallocates the given chunk of memory
     */
    unsafe fn deallocate(&mut self, nn_ptr: NonNull<u8>, layout: Layout);

    /**
     * Puts the given region to the memory pool of the allocator
     */
    unsafe fn add_region(&mut self,
                         start_area_ptr: NonNull<u8>,
                         area_size: usize)
                         -> Option<(NonNull<u8>, usize)>;

    /**
     * Returns the block size for slabs
     */
    fn block_size(&self) -> Option<usize>;

    /**
     * Wrapper to call `PREFERRED_EXTEND_SIZE` with `self`
     */
    fn preferred_extend_size(&self) -> usize {
        Self::PREFERRED_EXTEND_SIZE
    }
}
