/*! Kernel heap management */

use core::{
    alloc::Layout,
    ptr::NonNull
};

use heap::lazy_locked_heap::LazyLockedHeap;
use helps::{
    align::align_up,
    dbg::{
        DisplaySizePretty,
        C_MIB
    }
};
use sync::mutex::{
    spin_mutex::RawSpinMutex,
    ConstCreatBackRawMutex
};

use crate::{
    dbg_print::DbgLevel,
    dbg_println,
    vm::{
        Page4KiB,
        PageSize
    }
};

/* global heap allocator */
#[global_allocator]
static mut SM_HEAP_ALLOCATOR: LazyLockedHeap<RawSpinMutex> = unsafe {
    LazyLockedHeap::new(kernel_heap_raw_mutex_supplier, kernel_heap_mem_supplier)
};

/**
 * Forces the kernel heap initialization
 */
pub fn kernel_heap_init() {
    unsafe {
        SM_HEAP_ALLOCATOR.force_init();
    }
}

/**
 * `RawMutex` supplier for lazily initialized `Heap`
 */
fn kernel_heap_raw_mutex_supplier() -> Option<RawSpinMutex> {
    Some(RawSpinMutex::CONST_CREAT)
}

/**
 * Supplies additional memory regions to the `Heap` allocator
 */
fn kernel_heap_mem_supplier(requested_size: usize) -> Option<(NonNull<u8>, usize)> {
    const C_INIT_POOL_SIZE: usize = 1 * C_MIB;

    static mut SM_INIT_POOL_USED_PAGES: usize = 0;
    static mut SM_INIT_POOL: [u8; C_INIT_POOL_SIZE] = [0; C_INIT_POOL_SIZE];

    unsafe {
        let requested_pages = align_up(requested_size, Page4KiB::SIZE) / Page4KiB::SIZE;
        let requested_size = requested_pages * Page4KiB::SIZE;
        dbg_println!(DbgLevel::Trace,
                     "requested_pages: {} ({})",
                     requested_pages,
                     requested_size.display_pretty());

        /* return a sub-split of the <SM_INIT_HEAP_POOL> if still available */
        if C_INIT_POOL_SIZE / Page4KiB::SIZE < SM_INIT_POOL_USED_PAGES + requested_pages {
            None /* TODO allocate a new kernel region
                  *      MemManager::instance().allocate_kernel_region(...) */
        } else {
            let init_pool_used_size = SM_INIT_POOL_USED_PAGES * Page4KiB::SIZE;

            /* allocate a new sub-slice of the <SM_INIT_HEAP_POOL> */
            let allocated_pool_slice = {
                let new_pool_range =
                    init_pool_used_size..init_pool_used_size + requested_size;
                let new_pool_slice = &mut SM_INIT_POOL[new_pool_range];
                SM_INIT_POOL_USED_PAGES += requested_pages;

                new_pool_slice
            };

            let init_pool_rem_space =
                C_INIT_POOL_SIZE - SM_INIT_POOL_USED_PAGES * Page4KiB::SIZE;
            dbg_println!(DbgLevel::Trace,
                         "SM_INIT_HEAP_POOL remaining space: {}",
                         init_pool_rem_space.display_pretty());

            /* return the new memory for the Heap */
            Some((NonNull::new_unchecked(allocated_pool_slice.as_mut_ptr()),
                  requested_size))
        }
    }
}

/**
 * Kernel heap allocation error handler
 */
#[alloc_error_handler]
fn kernel_heap_alloc_error_handler(layout: Layout) -> ! {
    panic!("Heap allocation failed, no more memory available, requested {:?}", layout);
}
