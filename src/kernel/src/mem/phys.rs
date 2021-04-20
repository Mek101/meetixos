/*! # Kernel Physical Memory Manager
 *
 * Implements the global physical memory allocator used by the kernel
 */

use core::{ops::Range, slice};

use bit_field::{BitArray, BitField};

use hal::{
    addr::{Address, PhysAddr},
    boot::infos::BootInfos,
    paging::{Page2MiB, Page4KiB, PageSize, PhysFrame, PhysFrameRange}
};
use sync::SpinMutex;

#[cfg(debug_assertions)]
use crate::log::debug;
use crate::{
    debug::debug_size_multiplier,
    log::{info, warn},
    mem::paging::{bytes_to_pages_count, paging_map_unmanaged}
};

/* bitmap allocator */
static mut BITMAP_ALLOCATOR: LockedBitMapAllocator =
    LockedBitMapAllocator::new_uninitialized();

/* ugly and inefficient pre-init allocator, used to allocate the physical
 * frames used to map the bitmap
 */
static mut PRE_INIT_BOOT_ALLOCATOR: PreInitBootMemAllocator =
    PreInitBootMemAllocator::new();

/* until this is false the <PRE_INIT_BOOT_ALLOCATOR> is used */
static mut IS_INITIALIZED: bool = false;
static mut TOTAL_MEMORY: usize = 0;

/** # Initializes the physical memory allocator
 *
 * Initializes the global physical memory allocator mapping the physical
 * frames to store the bitmap
 */
pub fn init_phys_mem() {
    let boot_infos = BootInfos::obtain();

    /* calculate the total amount of the available memory and obtain how many
     * pages are necessary to store the physical frames (divided by 4KiB frames)
     * into the bitmap allocator
     */
    let (bitmap_pages_count, bitmap_bytes_count) = {
        /* calculate the total memory available and write out a warning when the
         * kernel receives less than 8MiB of memory
         */
        let total_mem = boot_infos.mem_areas().iter().map(|area| area.size()).sum();
        if total_mem < 4 * Page2MiB::SIZE {
            warn!("Detected a VERY SMALL amount of physical memory: less than 8MiB");
        }

        /* assign out the global total memory counter.
         * This is the only place where is modified, so is why it is not locked in
         * any way
         */
        unsafe { TOTAL_MEMORY = total_mem };

        /* calculate how many space reserve for the bitmap allocator */
        let bitmap_pages_count =
            bytes_to_pages_count::<Page4KiB>(total_mem / Page4KiB::SIZE / u8::BIT_LENGTH);
        let bitmap_bytes_count = bitmap_pages_count * Page4KiB::SIZE;

        (bitmap_pages_count, bitmap_bytes_count)
    };

    {
        let total_mem = phys_mem_total_mem();
        info!("Available memory: {}Bytes ({})",
              total_mem,
              debug_size_multiplier(total_mem));
    }

    #[cfg(debug_assertions)]
    debug!("4KiB Pages necessary for the bitmap: {}", bitmap_pages_count);

    /* map into virtual memory the area reserved for the bitmap.
     * This step allocates at least <bitmap_pages_count> pages and at max 3 page
     * tables. This is where <PRE_BOOT_INIT_ALLOCATOR> is used
     */
    let bitmap_area_ptr = paging_map_unmanaged(None, bitmap_pages_count);

    #[cfg(debug_assertions)]
    debug!("Bitmap allocated at: {:#x}", bitmap_area_ptr as usize);

    unsafe {
        /* initialize now the bitmap allocator with the virtual area allocated */
        BITMAP_ALLOCATOR.init(bitmap_area_ptr, bitmap_bytes_count);

        /* obtain the <PRE_INIT_BOOT_ALLOCATOR>'s frames iterator */
        if let Some(frames_it) = PRE_INIT_BOOT_ALLOCATOR.iter_to_next() {
            /* iterate now each remaining physical frame and put it into the bitmap */
            for phys_frame in frames_it {
                BITMAP_ALLOCATOR.add_frame(phys_frame)
            }
        } else {
            /* the <PRE_INIT_BOOT_ALLOCATOR> have exhausted the physical frames
             * available only allocating the bitmap pages...hummm then is not possible
             * to continue the kernel's execution
             */
            panic!("Out of physical memory");
        }

        /* the physical allocator module is now ready to use the bitmap allocator */
        IS_INITIALIZED = true;
    }
    info!("Physical allocator initialized");
}

/** # Allocates a new `PhysFrame`
 *
 * Requests to the underling physical allocators to return an unused
 * [`PhysFrame`]
 *
 * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
 */
pub fn phys_mem_alloc_frame<S>() -> Option<PhysFrame<S>>
    where S: PageSize {
    if !is_initialized() {
        unsafe {
            /* use the inefficient allocator until the module is not ready */
            PRE_INIT_BOOT_ALLOCATOR.allocate().map(|phys_frame| {
                                                  phys_frame.into_generic_sized_frame()
                                              })
        }
    } else {
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
}

/** # Frees an in-use `PhysFrame`
 *
 * Returns the given [`PhysFrame`] to the frame allocator that allocated it
 *
 * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
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

/** Returns the total physical memory available in bytes
 */
pub fn phys_mem_total_mem() -> usize {
    unsafe { TOTAL_MEMORY }
}

/** Returns the physical memory currently allocated in bytes
 */
pub fn phys_mem_allocated_mem() -> usize {
    unsafe { BITMAP_ALLOCATOR.allocated_mem() + PRE_INIT_BOOT_ALLOCATOR.allocated_mem() }
}

/** Returns the physical memory currently free in bytes
 */
pub fn phys_mem_free_memory() -> usize {
    phys_mem_total_mem() - phys_mem_allocated_mem()
}

/** Returns whether the module is initialized, used to limit the unsafe scope
 */
fn is_initialized() -> bool {
    unsafe { IS_INITIALIZED }
}

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
struct LockedBitMapAllocator<'a> {
    m_inner: SpinMutex<Option<BitMapAllocatorInner<'a>>>
}

impl<'a> LockedBitMapAllocator<'a> {
    /** # Constructs an uninitialized `LockedBitMapAllocator`
     *
     * The returned instance must be initialized with the
     * [`LockedBitMapAllocator::init()`]
     *
     * [`LockedBitMapAllocator::init()`]:
     * /kernel/mem/phys/struct.LockedBitMapAllocator.html#method.init
     */
    const fn new_uninitialized() -> Self {
        Self { m_inner: SpinMutex::new(None) }
    }

    /** # Initializes the `LockedBitMapAllocator`
     *
     * Construct the [`BitMapAllocatorInner`] to become ready to free frames
     *
     * [`BitMapAllocatorInner`]:
     * /kernel/mem/phys/struct.BitMapAllocatorInner.html
     */
    unsafe fn init(&mut self, bitmap_area_ptr: *mut u8, bytes_count: usize) {
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
    fn allocate_one(&self) -> Option<PhysFrame<Page4KiB>> {
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
    fn allocate_contiguous(&self,
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
    fn free_one(&self, phys_frame: PhysFrame<Page4KiB>) {
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
    fn free_contiguous(&self, frames_range: PhysFrameRange<Page4KiB>) {
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
    fn add_frame(&self, phys_frame: PhysFrame<Page4KiB>) {
        if let Some(ref mut inner) = *self.m_inner.lock() {
            inner.add_bit(phys_frame.start_addr().as_usize() / Page4KiB::SIZE);
        }
    }

    /** Returns the total amount of memory allocated
     */
    fn allocated_mem(&self) -> usize {
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

/** # Pre Init Boot Memory Allocator
 *
 * Ugly and inefficient allocator that directly uses the [`BootMemAreas`]
 * collection to sequentially iterate the 4KiB [`PhysFrame`]s available.
 * With this allocator is not possible to deallocate the frames
 *
 * [`BootMemAreas`]: /hal/boot/infos/struct.BootMemAreas.html
 * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
 */
struct PreInitBootMemAllocator {
    m_next_frame: usize
}

impl PreInitBootMemAllocator {
    /** # Constructs a `PreInitBootMemAllocator`
     *
     * The returned instance starts from the first available frame to
     * allocate
     */
    const fn new() -> Self {
        Self { m_next_frame: 0 }
    }

    /** # Allocates a `PhysFrame`
     *
     * Returns the first available [`PhysFrame`]
     *
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     */
    fn allocate(&mut self) -> Option<PhysFrame<Page4KiB>> {
        self.iter_to_next()?.next().map(|phys_frame| {
                                       self.m_next_frame += 1;
                                       phys_frame
                                   })
    }

    /** Returns an [`Iterator`] that starts from the next available
     * [`PhysFrame`]
     *
     * [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     */
    fn iter_to_next(&mut self)
                    -> Option<impl Iterator<Item = PhysFrame<Page4KiB>> + 'static> {
        let infos = BootInfos::obtain();
        let mut it =
            infos.mem_areas()
                 .iter()
                 .map(|mem_area| {
                     let start_addr = mem_area.start_phys_addr();
                     let end_addr = start_addr + mem_area.size();

                     /* create the range of integer addresses */
                     start_addr.as_usize()..end_addr.as_usize()
                 })
                 .flat_map(|range| {
                     /* add a new iterators that iterates the values of the
                      * range from the first to the last with a step of 4KiB
                      */
                     range.step_by(Page4KiB::SIZE)
                 })
                 .map(|raw_addr| {
                     /* create the wrapper frame */
                     PhysFrame::of_addr(unsafe { PhysAddr::new_unchecked(raw_addr) })
                 });
        it.advance_by(self.m_next_frame).map(|_| it).ok()
    }

    /** Returns the amount of memory allocated in bytes
     */
    fn allocated_mem(&self) -> usize {
        self.m_next_frame * Page4KiB::SIZE
    }
}
