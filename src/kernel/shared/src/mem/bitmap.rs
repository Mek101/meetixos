/*! # Bitmap Allocator
 *
 * Implements a simple bitmap allocator used both by the hh_loader and the
 * kernel core
 */

use core::{ops::Range, slice};

use bit_field::{BitArray, BitField};

use sync::{Mutex, RawMutex};

use crate::{
    addr::{Address, PhysAddr},
    mem::paging::{Page4KiB, PageSize, PhysFrame, PhysFrameRange}
};

/** # Locked Bitmap Allocator
 *
 * Implements a simple allocator that relies on a bitmap to keep track of
 * the allocated [`PhysFrame`] and ensures thread safety through a lock.
 *
 * Each bit represents a [`PhysFrame`]<[`Page4KiB`]> so the allocations (and
 * deallocations happen with this granularity)
 *
 * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
 * [`Page4KiB`]: /hal/paging/struct.Page4KiB.html
 */
pub struct LockedBitMapAllocator<'a, L>
    where L: RawMutex {
    m_inner: Mutex<L, Option<BitMapAllocatorInner<'a>>>
}

impl<'a, L> LockedBitMapAllocator<'a, L> where L: RawMutex {
    /** # Constructs an uninitialized `LockedBitMapAllocator`
     *
     * The returned instance must be initialized with the
     * [`LockedBitMapAllocator::init()`]
     *
     * [`LockedBitMapAllocator::init()`]:
     * /kernel/mem/phys/struct.LockedBitMapAllocator.html#method.init
     */
    pub const fn new_uninitialized() -> Self {
        Self { m_inner: Mutex::new(None) }
    }

    /** # Initializes the `LockedBitMapAllocator`
     *
     * Construct the [`BitMapAllocatorInner`] to become ready to free frames
     *
     * [`BitMapAllocatorInner`]:
     * /kernel/mem/phys/struct.BitMapAllocatorInner.html
     */
    pub unsafe fn init(&mut self, bitmap_area_ptr: *mut u8, bytes_count: usize) {
        *self.m_inner.lock() =
            Some(BitMapAllocatorInner::new(bitmap_area_ptr, bytes_count))
    }

    /** # Allocates a single `PhysFrame<Page4KiB>`
     *
     * Finds the first available bit and maps the returned bit-index to a
     * [`PhysFrame`]<[`Page4KiB`]>
     *
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     * [`Page4KiB`]: /hal/paging/struct.Page4KiB.html
     */
    pub fn allocate_one(&self) -> Option<PhysFrame<Page4KiB>> {
        if let Some(ref mut inner) = *self.m_inner.lock() {
            inner.allocate_bit().map(|bit_index| {
                                    PhysFrame::of_addr(unsafe {
                                        PhysAddr::new_unchecked(bit_index
                                                                * Page4KiB::SIZE)
                                    })
                                })
        } else {
            None
        }
    }

    /** # Allocates a `Range` of `PhysFrame<Page4KiB>`
     *
     * Finds the first available byte-aligned block of bits and maps them
     * into a [`PhysFrameRange`]<[`Page4KiB`]>
     *
     * [`PhysFrameRange`]: /hal/paging/type.PhysFrameRange.html
     * [`Page4KiB`]: /hal/paging/struct.Page4KiB.html
     */
    pub fn allocate_contiguous(&self,
                               frames_count: usize)
                               -> Option<PhysFrameRange<Page4KiB>> {
        if let Some(ref mut inner) = *self.m_inner.lock() {
            inner.allocate_bits(frames_count).map(|range| {
                let start_frame = PhysFrame::of_addr(unsafe {
                    PhysAddr::new_unchecked(range.start * Page4KiB::SIZE)
                });
                let end_frame = PhysFrame::of_addr(unsafe {
                    PhysAddr::new_unchecked(range.end * Page4KiB::SIZE)
                });

                PhysFrame::range_of(start_frame, end_frame)
            })
        } else {
            None
        }
    }

    /** # Frees a single `PhysFrame<Page4KiB>`
     *
     * Sets as available the bit that corresponds to the given
     * [`PhysFrame`]<[`Page4KiB`]>
     *
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     * [`Page4KiB`]: /hal/paging/struct.Page4KiB.html
     */
    pub fn free_one(&self, phys_frame: PhysFrame<Page4KiB>) {
        if let Some(ref mut inner) = *self.m_inner.lock() {
            inner.free_bit(phys_frame.start_addr().as_usize() / Page4KiB::SIZE)
        }
    }

    /** # Frees a `Range` of `PhysFrame<Page4KiB>`
     *
     * Sets as available the bits that correspond to the given
     * [`PhysFrameRange`]<[`Page4KiB`]>
     *
     * [`PhysFrameRange`]: /hal/paging/type.PhysFrameRange.html
     * [`Page4KiB`]: /hal/paging/struct.Page4KiB.html
     */
    pub fn free_contiguous(&self, frames_range: PhysFrameRange<Page4KiB>) {
        if let Some(ref mut inner) = *self.m_inner.lock() {
            let bits_range_to_free =
                Range { start: frames_range.start.start_addr().as_usize()
                               / Page4KiB::SIZE,
                        end: frames_range.end.start_addr().as_usize() / Page4KiB::SIZE };

            inner.free_bits(bits_range_to_free)
        }
    }

    /** # Adds the given `PhysFrame<Page4KiB>` as available
     *
     * Makes the given [`PhysFrame`]<[`Page4KiB`]> available for further
     * allocations
     *
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     * [`Page4KiB`]: /hal/paging/struct.Page4KiB.html
     */
    pub fn add_frame(&self, phys_frame: PhysFrame<Page4KiB>) {
        if let Some(ref mut inner) = *self.m_inner.lock() {
            inner.add_bit(phys_frame.start_addr().as_usize() / Page4KiB::SIZE);
        }
    }

    /** Returns the total amount of memory allocated
     */
    pub fn allocated_mem(&self) -> usize {
        if let Some(ref inner) = *self.m_inner.lock() {
            inner.allocated_bits() * Page4KiB::SIZE
        } else {
            0
        }
    }
}

/** # Bitmap Allocator Inner
 *
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
    /** # Constructs a new `BitMapAllocatorInner`
     *
     * Constructs a mutable slice from the given raw parameters and sets
     * every byte to 0
     */
    unsafe fn new(bitmap_area: *mut u8, bytes_count: usize) -> Self {
        /* mark all the bits as un available */
        let slice = slice::from_raw_parts_mut(bitmap_area, bytes_count);
        slice.fill(0);

        Self { m_bits: slice,
               m_allocated_bits: 0 }
    }

    /** # Allocates a bit index
     *
     * Returns the first available bit index and increases the counter of
     * the allocated bits
     */
    fn allocate_bit(&mut self) -> Option<usize> {
        for (byte_index, byte) in self.m_bits.iter_mut().enumerate() {
            /* find the first non-zero byte. Since <true> is used to identify available
             * bits non-zero value bytes means that contains at least one bit
             * allocatable
             */
            if *byte != 0u8 {
                /* same for the bits, find the first available and use it */
                for bit_index in 0..u8::BIT_LENGTH {
                    if byte.get_bit(bit_index) {
                        byte.set_bit(bit_index, false);
                        self.m_allocated_bits += 1;

                        /* returns the absolute allocated index */
                        return Some(byte_index * u8::BIT_LENGTH + bit_index);
                    }
                }
            }
        }
        None
    }

    /** # Allocates a contiguous `Range` of bit indexes
     *
     * Returns the Range of bit-indexes that are contiguous and respects
     */
    fn allocate_bits(&mut self, bits_count: usize) -> Option<Range<usize>> {
        assert_eq!(bits_count % u8::BIT_LENGTH, 0);

        self.find_free_block(bits_count).map(|first_free_block_bit| {
                                            let bit_range = first_free_block_bit
                                                            ..first_free_block_bit
                                                              + bits_count;

                                            /* mark all the selected bits as allocated */
                                            for bit_index in bit_range.clone() {
                                                self.m_bits.set_bit(bit_index, false);
                                            }
                                            self.m_allocated_bits += bits_count;

                                            bit_range
                                        })
    }

    /** # Free a single bit-index
     *
     * Makes available again the bit given
     */
    fn free_bit(&mut self, bit_index: usize) {
        assert_eq!(self.m_bits.get_bit(bit_index), false);

        self.m_bits.set_bit(bit_index, true);
        self.m_allocated_bits -= 1;
    }

    /** # Free a `Range` of bit-indexes
     *
     * Makes available again the bits given
     */
    fn free_bits(&mut self, range_to_free: Range<usize>) {
        self.m_allocated_bits -= range_to_free.end - range_to_free.start;
        for bit in range_to_free {
            self.m_bits.set_bit(bit, true);
        }
    }

    /** # Adds an available bit
     *
     * Makes the given bit as available for allocations
     */
    fn add_bit(&mut self, bit_index: usize) {
        self.m_bits.set_bit(bit_index, true);
    }

    /** Returns the current amount of allocated bits
     */
    fn allocated_bits(&self) -> usize {
        self.m_allocated_bits
    }

    /** # Finds the first block available of the requested size
     *
     * The bits count aligned must be aligned to the byte size (8 bits)
     */
    fn find_free_block(&self, bits_count: usize) -> Option<usize> {
        let bytes_count = bits_count / u8::BIT_LENGTH;

        /* iterate each byte in requested blocks */
        for byte_index in (0..self.m_bits.len() - bytes_count).step_by(bytes_count) {
            let slice_to_check = &self.m_bits[byte_index..byte_index + bytes_count];
            if Self::is_slice_all_available(slice_to_check) {
                return Some(byte_index * u8::BIT_LENGTH);
            }
        }
        None
    }

    /** Returns whether the given slice is available
     */
    fn is_slice_all_available(slice_to_check: &[u8]) -> bool {
        for byte in slice_to_check {
            if *byte != u8::MAX {
                return false;
            }
        }
        return true;
    }
}
