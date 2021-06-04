/*! Kernel heap management */

use core::alloc::Layout;

use heap::locked::raw::RawLazyLockedHeap;
use shared::{
    addr::{
        align_up,
        Address
    },
    logger::trace,
    mem::paging::{
        flags::PDirFlags,
        flush::MapFlusher,
        frame::{
            VirtFrame,
            VirtFrameRange
        },
        Page4KiB,
        PageSize
    }
};
use sync::{
    RawMutex,
    RawSpinMutex
};

use crate::mem::{
    paging::{
        allocators::KernAllocator,
        paging_current_page_dir
    },
    vm_layout::vml_layout
};

/* lazy allocator initialized by <init_heap()> */
#[global_allocator]
static mut HEAP_ALLOCATOR: RawLazyLockedHeap<RawSpinMutex> =
    unsafe { RawLazyLockedHeap::new(|| Some(RawSpinMutex::INIT), heap_mem_supplier) };

/* keeps the count of physical frames requested */
static mut HEAP_ALLOCATED_FRAMES: usize = 0;

/**
 * Forces the initialization of the global lazy allocator to avoid
 * concurrency in the first usage
 */
pub fn heap_init() {
    unsafe {
        HEAP_ALLOCATOR.force_init();
    }
}

/**
 * Callback used by the Rust dynamic memory frontend to throw out of memory
 * errors
 */
#[alloc_error_handler]
fn heap_alloc_error_handler(layout: Layout) -> ! {
    panic!("Heap allocation failed, no more memory available, requested {:?}", layout);
}

/**
 * Supplies additional memory to the `HEAP_ALLOCATOR`
 */
fn heap_mem_supplier(requested_size: usize) -> Option<(usize, usize)> {
    trace!("Called heap_mem_supplier({})", requested_size);

    /* align up the requested size to page boundary */
    let page_aligned_size = align_up(requested_size, Page4KiB::SIZE);
    let requested_pages = page_aligned_size / Page4KiB::SIZE;
    let next_active_heap_end =
        unsafe { (HEAP_ALLOCATED_FRAMES + requested_pages) * Page4KiB::SIZE };

    trace!("heap_mem_supplier: page_aligned_size = {}, requested_pages = {}, \
            next_active_heap_end = {}",
           page_aligned_size,
           requested_pages,
           next_active_heap_end);

    /* check for VM limits */
    let (heap_start_address, heap_end_address) = vml_layout().kern_heap_area().as_parts();
    if heap_start_address + next_active_heap_end >= heap_end_address {
        panic!("Reached end of Kernel's Heap reserved area");
    }

    /* prepare the new frame range to map */
    let new_heap_to_map_range: VirtFrameRange<Page4KiB> = {
        let current_active_heap_end =
            heap_start_address + unsafe { HEAP_ALLOCATED_FRAMES } * Page4KiB::SIZE;

        VirtFrame::range_of_count(current_active_heap_end.containing_frame(),
                                  requested_pages)
    };

    /* map the new range of pages for the heap */
    let mapping_res =
        paging_current_page_dir().map_range(new_heap_to_map_range.clone(),
                                            &KernAllocator::new(),
                                            PDirFlags::new().set_present()
                                                            .set_readable()
                                                            .set_writeable()
                                                            .set_no_execute()
                                                            .set_global()
                                                            .build());
    match mapping_res {
        Ok(map_flusher) => {
            trace!("heap_mem_supplier: Mapped {:?}", new_heap_to_map_range);

            /* update the currently mapped pages counter */
            unsafe {
                HEAP_ALLOCATED_FRAMES += requested_pages;
            }

            /* flush the TLB entries and return the address */
            map_flusher.flush();
            Some((new_heap_to_map_range.start.start_addr().as_usize(), page_aligned_size))
        },
        Err(err) => {
            trace!("heap_mem_supplier: Failed to extend kernel heap: cause: {}", err);
            None
        }
    }
}
