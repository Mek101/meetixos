/*! Bitmap allocator */

use core::{
    ops::Range,
    slice
};

use bits::bit_field::{
    BitArray,
    BitFindMode
};

use crate::{
    addr::{
        phys::PhysAddr,
        Address
    },
    mem::paging::{
        frame::{
            PhysFrame,
            PhysFrameRange
        },
        Page4KiB,
        PageSize
    }
};

/**
 * Simple allocator that relies on a bitmap to keep track of the allocated
 * `PhysFrame` and ensures thread safety through a lock.
 *
 * Each bit represents a `PhysFrame<Page4KiB>` so the allocations (and
 * deallocations happen with this granularity)
 */
pub struct BitMapAllocator<'a> {
    m_inner: Option<BitMapAllocatorInner<'a>>,
    m_added_frames: usize
}

impl<'a> BitMapAllocator<'a> {
    /**
     * Constructs an uninitialized `BitMapAllocator`, which must be
     * initialized with `BitMapAllocator::init()`
     */
    pub const fn new_uninitialized() -> Self {
        Self { m_inner: None,
               m_added_frames: 0 }
    }

    /**
     * Construct the `BitMapAllocatorInner` to become ready to free frames
     */
    pub unsafe fn init(&mut self,
                       bitmap_area_ptr: *mut u8,
                       bytes_count: usize,
                       allocated_bits: usize) {
        self.m_inner =
            Some(BitMapAllocatorInner::new(bitmap_area_ptr, bytes_count, allocated_bits))
    }

    /**
     * Finds the first available bit and maps the returned bit-index to a
     * `PhysFrame<Page4KiB>`
     */
    pub fn allocate_one(&mut self) -> Option<PhysFrame<Page4KiB>> {
        if let Some(ref mut inner) = self.m_inner {
            inner.allocate_bit().map(|bit_index| {
                                    let raw_addr = bit_index * Page4KiB::SIZE;
                                    PhysAddr::new(raw_addr).containing_frame()
                                })
        } else {
            None
        }
    }

    /**
     * Finds the first available byte-aligned block of bits and maps them
     * into a `PhysFrameRange<Page4KiB>`
     */
    pub fn allocate_contiguous(&mut self,
                               frames_count: usize,
                               alignment: usize)
                               -> Option<PhysFrameRange<Page4KiB>> {
        if let Some(ref mut inner) = self.m_inner {
            inner.allocate_bits(frames_count, alignment/Page4KiB::SIZE)
                .map(|range| {
                    let start_frame = {
                        let raw_addr = range.start * Page4KiB::SIZE;
                        PhysAddr::new(raw_addr).containing_frame()
                    };

                    let end_frame = {
                        let raw_addr = range.end * Page4KiB::SIZE;
                        PhysAddr::new(raw_addr).containing_frame()
                    };

                    PhysFrame::range_of(start_frame, end_frame)
                })
        } else {
            None
        }
    }

    /**
     * Sets as available the bit that corresponds to the given
     * `PhysFrame<Page4KiB>`
     */
    pub fn free_one(&mut self, phys_frame: PhysFrame<Page4KiB>) {
        if let Some(ref mut inner) = self.m_inner {
            inner.free_bit(phys_frame.start_addr().as_usize() / Page4KiB::SIZE)
        }
    }

    /**
     * Sets as available the bits that correspond to the given
     * `PhysFrameRange<Page4KiB>`
     */
    pub fn free_contiguous(&mut self, frames_range: PhysFrameRange<Page4KiB>) {
        if let Some(ref mut inner) = self.m_inner {
            let bits_range_to_free =
                Range { start: frames_range.start.start_addr().as_usize()
                               / Page4KiB::SIZE,
                        end: frames_range.end.start_addr().as_usize() / Page4KiB::SIZE };

            inner.free_bits(bits_range_to_free)
        }
    }

    /**
     * Makes the given `PhysFrame<Page4KiB>` available for further
     * allocations.
     *
     * Used for initializations or swap(?)
     */
    pub fn add_phys_frame(&mut self, phys_frame: PhysFrame<Page4KiB>) {
        if let Some(ref mut inner) = self.m_inner {
            inner.add_bit(phys_frame.start_addr().as_usize() / Page4KiB::SIZE);
            self.m_added_frames += 1;
        }
    }

    /**
     * Returns the total amount of memory allocated
     */
    pub fn allocated_mem(&self) -> usize {
        if let Some(ref inner) = self.m_inner {
            inner.allocated_bits() * Page4KiB::SIZE
        } else {
            0
        }
    }

    pub fn added_frames(&self) -> usize {
        self.m_added_frames
    }
}

/**
 * Implements the bitmap allocator algorithm.
 *
 * It doesn't threat frames but only bit-indexes, and allows to allocate
 * them in single or a contiguous block way.
 *
 * for each bit 0 means un-available, 1 means allocatable
 */
struct BitMapAllocatorInner<'a> {
    m_bits: &'a mut [u8],
    m_allocated_bits: usize
}

impl<'a> BitMapAllocatorInner<'a> {
    /**
     * Constructs a mutable slice from the given raw parameters and sets
     * every byte to 0
     */
    unsafe fn new(bitmap_area_ptr: *mut u8,
                  bytes_count: usize,
                  allocated_bits: usize)
                  -> Self {
        Self { m_bits: slice::from_raw_parts_mut(bitmap_area_ptr, bytes_count),
               m_allocated_bits: allocated_bits }
    }

    /**
     * Returns the first available bit index and increases the counter of
     * the allocated bits
     */
    fn allocate_bit(&mut self) -> Option<usize> {
        self.m_bits.find_bit(true, BitFindMode::Regular).map(|abs_bit_index| {
                                                            self.m_bits
                                                                .set_bit(abs_bit_index,
                                                                         false);
                                                            self.m_allocated_bits += 1;
                                                            abs_bit_index
                                                        })
    }

    /**
     * Returns the Range of bit-indexes that are contiguous and respects
     */
    fn allocate_bits(&mut self,
                     bits_count: usize,
                     alignment: usize)
                     -> Option<Range<usize>> {
        assert_eq!(bits_count % u8::BITS as usize, 0);

        self.find_first_free_aligned_block(bits_count, alignment)
            .map(|first_free_block_bit| {
                let bit_range = first_free_block_bit..first_free_block_bit + bits_count;

                /* mark all the selected bits as
                 * allocated */
                for bit_index in bit_range.clone() {
                    self.m_bits.set_bit(bit_index, false);
                }
                self.m_allocated_bits += bits_count;

                bit_range
            })
    }

    /**
     * Makes available again the bit given
     */
    fn free_bit(&mut self, bit_index: usize) {
        assert_eq!(self.m_bits.bit_at(bit_index), false);

        self.m_bits.set_bit(bit_index, true);
        self.m_allocated_bits -= 1;
    }

    /**
     * Makes available again the bits given
     */
    fn free_bits(&mut self, range_to_free: Range<usize>) {
        self.m_allocated_bits -= range_to_free.end - range_to_free.start;
        for bit_index in range_to_free {
            self.m_bits.set_bit(bit_index, true);
        }
    }

    /**
     * Makes the given bit as available for allocations
     */
    fn add_bit(&mut self, bit_index: usize) {
        self.m_bits.set_bit(bit_index, true);
    }

    /**
     * Returns the current amount of allocated bits
     */
    fn allocated_bits(&self) -> usize {
        self.m_allocated_bits
    }

    /**
     * Finds the first block available of the requested size
     *
     * The bits count aligned must be aligned to the byte size (8 bits)
     */
    fn find_first_free_aligned_block(&self,
                                     bits_count: usize,
                                     _alignment: usize)
                                     -> Option<usize> {
        let bytes_count = bits_count / u8::BITS as usize;

        /* iterate each byte in requested blocks */
        for byte_index in (0..self.m_bits.len() - bytes_count).step_by(bytes_count) {
            /* obtain the reference to the current slice of bytes */
            let slice_to_check = &self.m_bits[byte_index..byte_index + bytes_count];

            /* check whether this slice contains all 1 (available bits) */
            if Self::is_slice_all_available(slice_to_check) {
                return Some(byte_index * u8::BITS as usize);
            }
        }
        None
    }

    /**
     * Returns whether the given slice is available
     */
    fn is_slice_all_available(slice_to_check: &[u8]) -> bool {
        let (prefix, aligned, suffix) = unsafe { slice_to_check.align_to::<u64>() };

        prefix.iter().all(|&byte| byte == u8::MAX)
        && aligned.iter().all(|&value| value == u64::MAX)
        && suffix.iter().all(|&byte| byte == u8::MAX)
    }
}
