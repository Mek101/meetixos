/*! # Heap Management Library
 *
 * Implements an heap manager that could be used in kernel and userland
 */

#![no_std]
#![feature(once_cell,
           unboxed_closures,
           fn_traits,
           const_mut_refs,
           const_fn_trait_bound,
           const_fn_fn_ptr_basics)]

use core::{
    alloc::Layout,
    cmp::max,
    ptr::NonNull
};

use crate::{
    linked_list::LinkedList,
    slab::Slab
};

pub mod lazy_locked_heap;
pub mod linked_list;
pub mod slab;

/**
 * Maximum amount of bytes that can be wasted using slab allocation,
 * exceeded the value the allocation request rollbacks to linked list
 * allocation, which allocates the perfect right amount of memory (but
 * wasting more time)
 */
pub const SLAB_THRESHOLD: usize = 384;

/**
 * Callback used by the `Heap` to obtain more memory when it runs out.
 *
 * The function must return the starting address of the new virtual area
 * allocated and the up-aligned size of his minimum allocation block
 */
pub type HeapMemorySupplier = fn(requested_size: usize) -> Option<(NonNull<u8>, usize)>;

/**
 * Multi strategy heap manager.
 *
 * Internally two main allocation strategies are used:
 * * `Slab` - fixed size block allocation, used for little allocation
 *   requests (under 8KiB) and whenever the threshold doesn't exceed
 *   `SLAB_THRESHOLD`.
 * * `LinkedList` - classic UNIX first-fit-chunk allocation, used for
 *   allocation requests above the 8KiB or when slab allocation exceed the
 *   `SLAB_THRESHOLD`
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
    m_in_use_mem: usize,
    m_mem_from_supplier: usize
}

impl Heap /* Constants */ {
    /**
     * Initial amount of memory requested to the `HeapMemorySupplier` by the
     * `Heap::new()`
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
}

impl Heap /* Constructors */ {
    /**
     * Constructs an `Heap` which relies on the given `HeapMemorySupplier`
     */
    pub unsafe fn new(mem_supplier: HeapMemorySupplier) -> Option<Self> {
        /* obtain from the mem_supplier the initial memory to become operative */
        let (next_start_area_addr, up_aligned_area_size) =
            mem_supplier(Self::INITIAL_REQUESTED_MEM_AMOUNT)?;

        let mut next_start_area_addr = next_start_area_addr.as_ptr();
        let original_start_area_addr = next_start_area_addr;

        /* construct the slab_64 allocator */
        let slab_64 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr =
            next_start_area_addr.add(Slab::<64>::PREFERRED_EXTEND_SIZE);

        /* construct the slab_128 allocator */
        let slab_128 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr =
            next_start_area_addr.add(Slab::<128>::PREFERRED_EXTEND_SIZE);

        /* construct the slab_256 allocator */
        let slab_256 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr =
            next_start_area_addr.add(Slab::<256>::PREFERRED_EXTEND_SIZE);

        /* construct the slab_512 allocator */
        let slab_512 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr =
            next_start_area_addr.add(Slab::<512>::PREFERRED_EXTEND_SIZE);

        /* construct the slab_1024 allocator */
        let slab_1024 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr =
            next_start_area_addr.add(Slab::<1024>::PREFERRED_EXTEND_SIZE);

        /* construct the slab_2048 allocator */
        let slab_2048 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr =
            next_start_area_addr.add(Slab::<2048>::PREFERRED_EXTEND_SIZE);

        /* construct the slab_4096 allocator */
        let slab_4096 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr =
            next_start_area_addr.add(Slab::<4096>::PREFERRED_EXTEND_SIZE);

        /* construct the slab_8192 allocator */
        let slab_8192 = Slab::with_preferred_size(next_start_area_addr);
        next_start_area_addr =
            next_start_area_addr.add(Slab::<8192>::PREFERRED_EXTEND_SIZE);

        /* construct the linked_list allocator */
        let linked_list = {
            let linked_list_size = {
                let requested_mem_used_amount =
                    next_start_area_addr as usize - original_start_area_addr as usize;

                up_aligned_area_size - requested_mem_used_amount
            };

            LinkedList::new(next_start_area_addr, linked_list_size)
        };

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
                    m_in_use_mem: 0,
                    m_mem_from_supplier: up_aligned_area_size })
    }
}

impl Heap /* Methods */ {
    /**
     * Allocates new memory that fits the given `Layout` request.
     *
     * Returns `None` when the used allocator runs out of memory and the
     * `HeapMemorySupplier` returns `None` as well
     */
    pub fn allocate(&mut self, layout: Layout) -> Option<NonNull<u8>> {
        /* obtain the allocator selected for the layout given */
        let allocator_selector = AllocSelector::for_layout(&layout);

        /* perform the allocation now */
        let alloc_result = match allocator_selector {
            AllocSelector::Slab64 => self.m_slab_64.allocate(),
            AllocSelector::Slab128 => self.m_slab_128.allocate(),
            AllocSelector::Slab256 => self.m_slab_256.allocate(),
            AllocSelector::Slab512 => self.m_slab_512.allocate(),
            AllocSelector::Slab1024 => self.m_slab_1024.allocate(),
            AllocSelector::Slab2048 => self.m_slab_2048.allocate(),
            AllocSelector::Slab4096 => self.m_slab_4096.allocate(),
            AllocSelector::Slab8192 => self.m_slab_8192.allocate(),
            AllocSelector::LinkedList => self.m_linked_list.allocate(layout)
        };

        /* if the alloc operation fails try to refill the memory pools using the
         * memory supplier. If the refill fails too return None
         */
        if let Some(alloc_nn_ptr) = alloc_result {
            self.m_in_use_mem += layout.size();
            Some(alloc_nn_ptr)
        } else if self.refill_mem_pool(allocator_selector, &layout) {
            self.allocate(layout) /* retry the allocation */
        } else {
            None
        }
    }

    /**
     * Makes the given memory available again for further allocations
     */
    pub unsafe fn deallocate(&mut self, nn_ptr: NonNull<u8>, layout: Layout) {
        match AllocSelector::for_layout(&layout) {
            AllocSelector::Slab64 => self.m_slab_64.deallocate(nn_ptr),
            AllocSelector::Slab128 => self.m_slab_128.deallocate(nn_ptr),
            AllocSelector::Slab256 => self.m_slab_256.deallocate(nn_ptr),
            AllocSelector::Slab512 => self.m_slab_512.deallocate(nn_ptr),
            AllocSelector::Slab1024 => self.m_slab_1024.deallocate(nn_ptr),
            AllocSelector::Slab2048 => self.m_slab_2048.deallocate(nn_ptr),
            AllocSelector::Slab4096 => self.m_slab_4096.deallocate(nn_ptr),
            AllocSelector::Slab8192 => self.m_slab_8192.deallocate(nn_ptr),
            AllocSelector::LinkedList => self.m_linked_list.deallocate(nn_ptr, layout)
        }
        self.m_in_use_mem -= layout.size();
    }
}

impl Heap /* Getters */ {
    /**
     * Returns the total amount of memory returned by the
     * `HeapMemorySupplier`
     */
    pub fn memory_from_supplier(&self) -> usize {
        self.m_mem_from_supplier
    }

    /**
     * Returns the total amount of in-use memory (allocated)
     */
    pub fn memory_in_use(&self) -> usize {
        self.m_in_use_mem
    }

    /**
     * Returns the amount of currently available memory
     */
    pub fn memory_available(&self) -> usize {
        self.m_mem_from_supplier - self.m_in_use_mem
    }
}

impl Heap /* Privates */ {
    /**
     * Refills the memory pool for the given `SubHeapAllocator`
     */
    fn refill_mem_pool(&mut self,
                       allocator_selector: AllocSelector,
                       layout: &Layout)
                       -> bool {
        let sub_heap_allocator = match allocator_selector {
            AllocSelector::Slab64 => &mut self.m_slab_64 as &mut dyn HeapPool,
            AllocSelector::Slab128 => &mut self.m_slab_128 as &mut dyn HeapPool,
            AllocSelector::Slab256 => &mut self.m_slab_256 as &mut dyn HeapPool,
            AllocSelector::Slab512 => &mut self.m_slab_512 as &mut dyn HeapPool,
            AllocSelector::Slab1024 => &mut self.m_slab_1024 as &mut dyn HeapPool,
            AllocSelector::Slab2048 => &mut self.m_slab_2048 as &mut dyn HeapPool,
            AllocSelector::Slab4096 => &mut self.m_slab_4096 as &mut dyn HeapPool,
            AllocSelector::Slab8192 => &mut self.m_slab_8192 as &mut dyn HeapPool,
            AllocSelector::LinkedList => &mut self.m_linked_list as &mut dyn HeapPool
        };

        /* request to the supplier the maximum amount of memory */
        let mem_amount_to_request =
            max(sub_heap_allocator.preferred_extend_size(), layout.size());

        /* call the used-defined memory supplier */
        if let Some((start_area_addr, up_aligned_area_size)) =
            (self.m_mem_supplier)(mem_amount_to_request)
        {
            /* update the memory amount counter */
            self.m_mem_from_supplier += up_aligned_area_size;

            unsafe {
                /* add the region to the allocator and catch the eventual exceeding */
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
 * `Heap` sub-allocator pool
 */
pub trait HeapPool {
    /**
     * Puts the given region to the memory pool of the allocator
     */
    unsafe fn add_region(&mut self,
                         start_area_ptr: NonNull<u8>,
                         area_size: usize)
                         -> Option<(NonNull<u8>, usize)>;

    /**
     * Returns the preferred extend size value
     */
    fn preferred_extend_size(&self) -> usize;
}

pub trait PreferredExtendSize {
    /**
     * Preferred amount of memory for `add_region()`
     */
    const PREFERRED_EXTEND_SIZE: usize;
}

enum AllocSelector {
    Slab64,
    Slab128,
    Slab256,
    Slab512,
    Slab1024,
    Slab2048,
    Slab4096,
    Slab8192,
    LinkedList
}

impl AllocSelector /* Static Functions */ {
    /**
     * Returns the variants which servers as best the given `Layout` request
     */
    fn for_layout(layout: &Layout) -> Self {
        /* select the SubHeapAllocator from the given layout size */
        let mut allocator_selected = if layout.size() <= 64 && layout.align() <= 64 {
            Self::Slab64
        } else if layout.size() <= 128 && layout.align() <= 128 {
            Self::Slab128
        } else if layout.size() <= 256 && layout.align() <= 256 {
            Self::Slab256
        } else if layout.size() <= 512 && layout.align() <= 512 {
            Self::Slab512
        } else if layout.size() <= 1024 && layout.align() <= 1024 {
            Self::Slab1024
        } else if layout.size() <= 2048 && layout.align() <= 2048 {
            Self::Slab2048
        } else if layout.size() <= 4096 && layout.align() <= 4096 {
            Self::Slab4096
        } else if layout.size() <= 8192 && layout.align() <= 8192 {
            Self::Slab8192
        } else {
            Self::LinkedList
        };

        /* check for threshold to avoid memory waste */
        if let Some(slab_block_size) = allocator_selected.block_size() {
            /* rollback to linked-list if the selected slab waste too much memory */
            if slab_block_size - layout.size() > SLAB_THRESHOLD {
                allocator_selected = Self::LinkedList;
            }
        }
        allocator_selected
    }
}

impl AllocSelector /* Getters */ {
    /**
     * Returns the Block size for the current variant
     */
    fn block_size(&self) -> Option<usize> {
        match self {
            Self::Slab64 => Some(64),
            Self::Slab128 => Some(128),
            Self::Slab256 => Some(256),
            Self::Slab512 => Some(512),
            Self::Slab1024 => Some(1024),
            Self::Slab2048 => Some(2048),
            Self::Slab4096 => Some(4096),
            Self::Slab8192 => Some(8192),
            Self::LinkedList => None
        }
    }
}
