/*! Kernel physical memory manager */

use shared::{
    logger::info,
    mem::paging::{
        frame::PhysFrame,
        Page2MiB,
        Page4KiB,
        PageSize
    }
};
use sync::RawSpinMutex;

use crate::mem::phys::allocator::LockedBitMapAllocator;

mod allocator;

/* bitmap allocator */
static mut BITMAP_ALLOCATOR: LockedBitMapAllocator<RawSpinMutex> =
    LockedBitMapAllocator::new_uninitialized();

/* total amount of available physical memory in bytes */
static mut TOTAL_MEMORY: usize = 0;

/**
 * Initializes the global physical memory allocator
 */
pub fn init_phys_mem() {
    info!("Physical allocator initialized");
}

/**
 * Requests to the underling physical allocators to return an unused
 * `PhysFrame` of the requested size
 */
pub fn phys_mem_alloc_frame<S>() -> Option<PhysFrame<S>>
    where S: PageSize {
    /* match the PageSize requested and use the right bitmap allocator method */
    match S::SIZE {
        Page4KiB::SIZE => unsafe {
            BITMAP_ALLOCATOR.allocate_one()
                            .map(|phys_frame| phys_frame.into_generic_sized_frame())
        },
        Page2MiB::SIZE => unsafe {
            BITMAP_ALLOCATOR.allocate_contiguous(Page2MiB::SIZE / Page4KiB::SIZE)
                            .map(|phys_frame_range| {
                                phys_frame_range.start.into_generic_sized_frame()
                            })
        },
        _ => panic!("Requested a PhysFrame of a NOT supported PageSize")
    }
}

/**
 * Makes available again the given `PhysFrame`
 */
pub fn phys_mem_free_frame<S>(phys_frame: PhysFrame<S>)
    where S: PageSize {
    match S::SIZE {
        Page4KiB::SIZE => unsafe {
            BITMAP_ALLOCATOR.free_one(phys_frame.into_generic_sized_frame())
        },
        Page2MiB::SIZE => unsafe {
            BITMAP_ALLOCATOR.free_contiguous(phys_frame.into_range_of())
        },
        _ => panic!("Freeing a PhysFrame of a NOT supported PageSize")
    }
}

/**
 * Returns the total physical memory available in bytes
 */
pub fn phys_mem_total_mem() -> usize {
    unsafe { TOTAL_MEMORY }
}

/**
 * Returns the physical memory currently allocated in bytes
 */
pub fn phys_mem_allocated_mem() -> usize {
    unsafe { BITMAP_ALLOCATOR.allocated_mem() }
}

/**
 * Returns the physical memory currently free in bytes
 */
pub fn phys_mem_free_memory() -> usize {
    phys_mem_total_mem() - phys_mem_allocated_mem()
}
