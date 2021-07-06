/*! Physical memory allocator based on bitmap */

use core::slice;

pub struct BitMapAllocator<'a> {
    m_inner: Option<BitMapAllocatorInner<'a>>
}

struct BitMapAllocatorInner<'a> {
    m_frames_bitmap: &'a mut [u8]
}

impl<'a> BitMapAllocatorInner<'a> {
    unsafe fn new(frames_bitmap_ptr: *mut u8, frames_bitmap_size: usize) -> Self {
        Self { m_frames_bitmap: slice::from_raw_parts_mut(frames_bitmap_ptr,
                                                          frames_bitmap_size) }
    }

    fn allocate_bit(&mut self) -> Option<usize> {
        //self.m_frames_bitmap.find_bit()
        None
    }
}
