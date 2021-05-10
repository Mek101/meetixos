/*! # Kernel Heap Manager
 *
 * Implements the global heap allocator used by the kernel
 */

use core::alloc::Layout;

use heap::locked::raw::RawLazyLockedHeap;
use shared::{
    addr::{
        align_up,
        virt::VirtAddr,
        Address
    },
    dbg::dbg_display_size,
    logger::{
        debug,
        info
    },
    mem::paging::{
        flush::MapFlusher,
        frame::VirtFrame,
        table::PTFlags,
        Page4KiB,
        PageSize
    }
};
use sync::{
    RawMutex,
    RawSpinMutex
};

use crate::mem::{
    frame_allocators::KernAllocator,
    layout::{
        KRN_HEAP_END,
        KRN_HEAP_START
    },
    paging::paging_active_page_dir
};

/** The lazy allocator is initialized before the first use using
 * [`init_heap()`] to avoid concurrency initialization
 *
 * [`init_heap()`]: /kernel/mem/heap/fn.init_heap.html
 */
#[global_allocator]
static mut HEAP_ALLOCATOR: RawLazyLockedHeap<RawSpinMutex> =
    unsafe { RawLazyLockedHeap::new(|| Some(RawSpinMutex::INIT), heap_mem_supplier) };

/** # Initializes the heap allocator
 *
 * Forces the initialization of the global lazy allocator to avoid
 * concurrency in the first usage
 */
pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR.force_init();
    }
    info!("Heap allocator initialized");
}

/** Returns the heap's currently managed virtual region size
 */
pub fn heap_managed_mem() -> usize {
    unsafe { HEAP_ALLOCATOR.managed_mem() }
}

/** Returns the heap's currently allocated memory amount
 */
pub fn heap_allocated_mem() -> usize {
    unsafe { HEAP_ALLOCATOR.allocated_mem() }
}

/** Returns the heap's currently free memory amount
 */
pub fn heap_free_memory() -> usize {
    unsafe { HEAP_ALLOCATOR.free_memory() }
}

/** # Allocation error handler
 *
 * Callback used by the Rust dynamic memory frontend to throw out of memory
 * errors.
 *
 * In particular this method is called when the kernel exhaust the virtual
 * memory region reserved for the heap
 */
#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Heap allocation failed, no more memory available, requested {:?}", layout);
}

/** # Heap memory supplier
 *
 * Implements the memory supplier requested by the [`Heap`] manager to
 * request more memory from the backend
 *
 * [`Heap`]: /heap/struct.Heap.html
 */
fn heap_mem_supplier(requested_size: usize) -> Option<(usize, usize)> {
    static mut HEAP_PAGES: usize = 0;

    /* calculate the immediate up-aligned size from the requested one, the number
     * of pages to map and the new heap's end address
     */
    let page_aligned_size = align_up(requested_size, Page4KiB::SIZE);
    let requested_pages = page_aligned_size / Page4KiB::SIZE;
    let next_heap_end = (unsafe { HEAP_PAGES } + requested_pages) * Page4KiB::SIZE;

    #[cfg(debug_assertions)]
    {
        use shared::{
            dbg::dbg_display_size,
            logger::debug
        };

        debug!("Supplying additional {} to the heap allocator",
               dbg_display_size(page_aligned_size));
    }

    /* ensure that the kernel heap's reserved virtual area is still in limits */
    if KRN_HEAP_START + next_heap_end >= KRN_HEAP_END {
        panic!("Reached the end of kernel's heap reserved virtual memory area");
    }

    /* construct the frame range to map */
    let mapping_frame_range = unsafe {
        let current_heap_end_addr =
            VirtAddr::new(KRN_HEAP_START + HEAP_PAGES * Page4KiB::SIZE);
        VirtFrame::range_of_count(VirtFrame::of_addr(current_heap_end_addr),
                                  requested_pages)
    };

    /* lets now map the new memory allocating physical frames explicitly
     * (PTFlags::PRESENT is given)
     */
    let mut page_dir = paging_active_page_dir();
    if let Ok(map_flusher) = page_dir.map_range(mapping_frame_range.clone(),
                                                &mut KernAllocator::new(),
                                                PTFlags::PRESENT
                                                | PTFlags::READABLE
                                                | PTFlags::WRITEABLE
                                                | PTFlags::GLOBAL)
    {
        /* update the amount of allocated pages for the heap. Note that any kind of
         * lock is used here, because the heap manager calls this supplier when is
         * already locked
         */
        unsafe { HEAP_PAGES += requested_pages };

        /* flush the new TLB entries */
        map_flusher.flush();

        /* return the start address and the aligned size */
        Some((mapping_frame_range.start.start_addr().as_usize(), page_aligned_size))
    } else {
        None
    }
}
