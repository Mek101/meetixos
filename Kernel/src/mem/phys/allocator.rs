/*! Physical memory allocator implementation */

use shared::mem::{
    bitmap::BitMapAllocator,
    paging::{
        frame::{
            PhysFrame,
            PhysFrameRange
        },
        Page4KiB
    }
};
use sync::{
    Mutex,
    RawMutex
};

/**
 * Thread safe `BitMapAllocator`
 */
pub(super) struct LockedBitMapAllocator<'a, L>
    where L: RawMutex {
    m_inner: Mutex<L, BitMapAllocator<'a>>
}

impl<'a, L> LockedBitMapAllocator<'a, L> where L: RawMutex {
    /**
     * Constructs an uninitialized `LockedBitMapAllocator`
     */
    pub const fn new_uninitialized() -> Self {
        Self { m_inner: Mutex::new(BitMapAllocator::new_uninitialized()) }
    }

    /**
     * Initializes the instance
     */
    pub fn init(&mut self,
                bitmap_area_ptr: *mut u8,
                bytes_count: usize,
                allocated_bits: usize) {
        unsafe {
            self.m_inner.lock().init(bitmap_area_ptr, bytes_count, allocated_bits);
        }
    }

    /**
     * Allocates the first available `PhysFrame<Page4KiB>`
     */
    pub fn allocate_one(&self) -> Option<PhysFrame<Page4KiB>> {
        self.m_inner.lock().allocate_one()
    }

    /**
     * Allocates the first available contiguous `frames_count` bits
     */
    pub fn allocate_contiguous(&self,
                               frames_count: usize,
                               alignment: usize)
                               -> Option<PhysFrameRange<Page4KiB>> {
        self.m_inner.lock().allocate_contiguous(frames_count, alignment)
    }

    /**
     * Frees the given `PhysFrame<Page4KiB>`
     */
    pub fn free_one(&self, phys_frame: PhysFrame<Page4KiB>) {
        self.m_inner.lock().free_one(phys_frame)
    }

    /**
     * Frees the given `PhysFrameRange<Page4KiB>`
     */
    pub fn free_contiguous(&self, frames_range: PhysFrameRange<Page4KiB>) {
        self.m_inner.lock().free_contiguous(frames_range)
    }

    /*
     * Marks as available the given `PhysFrame<Page4KiB>`.
     *
     * Used by the swapper
     */
    //pub fn add_frame(&self, phys_frame: PhysFrame<Page4KiB>) {
    //    self.m_inner.lock().add_phys_frame(phys_frame)
    //}

    /*
     * Returns the total amount of memory allocated
     */
    //pub fn allocated_mem(&self) -> usize {
    //   self.m_inner.lock().allocated_mem()
    //}
}
