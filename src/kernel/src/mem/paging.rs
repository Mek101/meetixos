/*! # Kernel High Level Paging Management
 *
 * This module implements various global functions for paging management
 * that are used across the kernel
 */

use hal::{
    addr::{Address, VirtAddr},
    boot::infos::BootInfos,
    paging::{
        MapFlusher, PTFlags, Page4KiB, PageDir, PageSize, PhysFrame, VirtFrame,
        VirtFrameRange
    }
};

use sync::{Lazy, SpinMutex};

use crate::mem::{
    frame_allocators::{KernAllocator, RangeAllocator},
    layout::{KRN_UNMNG_AREA_END, KRN_UNMNG_AREA_START}
};

/** Simple allocator used to manage the kernel's unmanaged area
 */
static mut UNMNG_AREA_ALLOCATOR: UnmngAreaLockedAllocator =
    UnmngAreaLockedAllocator::new();

/** Returns the currently active [`PageDir`]
 *
 * [`PageDir`]: /hal/paging/struct.PageDir.html
 */
pub fn paging_active_page_dir() -> PageDir {
    unsafe {
        PageDir::active_page_dir(BootInfos::obtain().hw_phys_mem_offset().as_usize())
    }
}

/** # Makes accessible the given `PhysFrame` range
 *
 * If `start_phys_frame` is [`Some`] the function maps the given
 * [`PhysFrame`] range into the first available address of the kernel's
 * unmanaged area.
 *
 * If `start_phys_frame` is [`None`] the function uses the [`KernAllocator`]
 * to map physical pages
 *
 * [`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
 * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
 * [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
 * [`KernAllocator`]: /kernel/mem/frame_allocators/struct.KernAllocator.html
 */
pub fn paging_map_unmanaged<T>(start_phys_frame: Option<PhysFrame<Page4KiB>>,
                               count: usize)
                               -> *mut T {
    /* allocate a free range into the unmanaged area.
     * The unmanaged area is not able to free allocated ranges, because it is
     * used for stuffs that are not freed anymore, like the physical memory
     * stacks, the ACPI, the modules and so on
     */
    let virt_map_range = unsafe {
        if let Some(frame_range) = UNMNG_AREA_ALLOCATOR.allocate(count) {
            frame_range
        } else {
            panic!("Reached the end of the kernel's unmanaged area")
        }
    };

    /* obtain the current page directory. Since we map using global pages any
     * page directory can be used
     */
    let mut page_dir = paging_active_page_dir();

    /* if given use the physical frame and a RangeAllocator, otherwise use a
     * kernel allocator
     */
    let result = if let Some(start_frame) = start_phys_frame {
        page_dir.map_range(virt_map_range.clone(),
                           &mut RangeAllocator::new(PhysFrame::range_of_count(start_frame, count)),
                           PTFlags::PRESENT
                           | PTFlags::GLOBAL
                           | PTFlags::READABLE
                           | PTFlags::WRITEABLE
                           | PTFlags::NO_EXECUTE)
    } else {
        page_dir.map_range(virt_map_range.clone(),
                           &mut KernAllocator::new(),
                           PTFlags::PRESENT
                           | PTFlags::GLOBAL
                           | PTFlags::READABLE
                           | PTFlags::WRITEABLE
                           | PTFlags::NO_EXECUTE)
    };

    /* flush the new created entries if success, panic otherwise */
    match result {
        Ok(map_flusher) => map_flusher.flush(),
        Err(err) => {
            panic!("Failed to map {:?} to kernel's free area: {}", virt_map_range, err)
        }
    }

    /* return the pointer to the mapped virtual range */
    virt_map_range.start.start_addr().as_ptr_mut()
}

/** # Converts an amount of bytes into pages amount
 *
 * Calculates how many pages of the given [`PageSize`] are necessary to
 * store `bytes_count`
 *
 * [`PageSize`]: /hal/paging/trait.PageSize.html
 */
pub fn bytes_to_pages_count<S>(bytes_count: usize) -> usize
    where S: PageSize {
    (bytes_count + S::MASK) >> S::USED_BITS
}

/** # Unmanaged Area Locked Allocator
 *
 * Very simple allocator that returns consecutive ranges of pages that are
 * never freed
 */
struct UnmngAreaLockedAllocator {
    m_inner: Lazy<SpinMutex<VirtFrame<Page4KiB>>>
}

impl UnmngAreaLockedAllocator {
    /** # Constructs a new `UnmngAreaLockedAllocator`
     *
     * The returned instance is lazily initialized
     */
    const fn new() -> Self {
        Self { m_inner: Lazy::new(|| {
                   SpinMutex::new(VirtFrame::of_addr(unsafe {
                                      VirtAddr::new_unchecked(KRN_UNMNG_AREA_START)
                                  }))
               }) }
    }

    /** # Allocates a new `VirtFrameRange`
     *
     * The returned [`VirtFrameRange`] is ensured to be of `count` pages or
     * [`None`] is returned
     *
     * [`VirtFrameRange`]: /hal/paging/type.VirtFrameRange.html
     * [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
     */
    fn allocate(&mut self, count: usize) -> Option<VirtFrameRange<Page4KiB>> {
        let mut current_frame = self.m_inner.lock();
        if (*current_frame + count).start_addr().as_usize() >= KRN_UNMNG_AREA_END {
            None
        } else {
            let start_frame = *current_frame;
            *current_frame += count;
            Some(VirtFrame::range_of_count(start_frame, count))
        }
    }
}
