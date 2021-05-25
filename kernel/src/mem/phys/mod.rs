/*! Kernel physical memory management */

use shared::{
    logger::{
        trace,
        warn
    },
    mem::paging::{
        frame::PhysFrame,
        Page2MiB,
        Page4KiB,
        PageSize
    }
};
use sync::RawSpinMutex;

use crate::mem::{
    phys::allocator::LockedBitMapAllocator,
    vm_layout::vml_layout
};

mod allocator;

/* bitmap allocator, initialized by <phys_init()> */
static mut BITMAP_ALLOCATOR: LockedBitMapAllocator<RawSpinMutex> =
    LockedBitMapAllocator::new_uninitialized();

/**
 * Initializes the physical memory manager
 */
pub fn phys_init(allocated_bits: usize) {
    let bitmap_area = vml_layout().phys_mem_bitmap_area();

    /* initialize the bitmap allocator using the hh_loader's bitmap */
    unsafe {
        BITMAP_ALLOCATOR.init(bitmap_area.start_addr().as_ptr_mut(),
                              bitmap_area.size(),
                              allocated_bits);
    }
}

/**
 * Allocates a `PhysFrame` of the requested size
 */
pub fn phys_alloc_frame<S>() -> Option<PhysFrame<S>>
    where S: PageSize {
    let phys_frame = match S::SIZE {
        Page4KiB::SIZE => unsafe {
            BITMAP_ALLOCATOR.allocate_one()
                            .map(|phys_frame| phys_frame.into_generic_sized_frame())
        },
        Page2MiB::SIZE => unsafe {
            BITMAP_ALLOCATOR.allocate_contiguous(Page2MiB::SIZE / Page4KiB::SIZE)
                            .map(|phys_frame| phys_frame.start.into_generic_sized_frame())
        },
        _ => {
            warn!("phys_alloc_frame() called with unknown PageSize");
            None
        }
    };

    if let Some(phys_frame) = phys_frame {
        trace!("phys_alloc_frame: Allocated PhysFrame = {:?}", phys_frame);
    }
    phys_frame
}

/**
 * Makes available again, for further allocations, the given `PhysFrame`
 */
pub fn phys_free_frame<S>(phys_frame: PhysFrame<S>)
    where S: PageSize {
    match S::SIZE {
        Page4KiB::SIZE => unsafe {
            BITMAP_ALLOCATOR.free_one(phys_frame.into_generic_sized_frame())
        },
        Page2MiB::SIZE => unsafe {
            BITMAP_ALLOCATOR.free_contiguous(phys_frame.into_range_of())
        },
        _ => {
            warn!("phys_free_frame() called with unknown PageSize");
        }
    }
}
