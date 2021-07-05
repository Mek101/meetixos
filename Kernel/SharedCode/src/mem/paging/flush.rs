/*! Map flusher */

use core::mem;

use crate::{
    addr::virt::VirtAddr,
    arch::mem::paging::flush::HwMapFlusher,
    mem::paging::{
        frame::{
            VirtFrame,
            VirtFrameRangeIncl
        },
        PageSize
    }
};

/**
 * Map flusher for a single frame.
 *
 * It tells to the hardware TLB to remove the page table entry that
 * references the given `VirtFrame` because is changed.
 *
 * The next access to the same virtual location will throw a TLB cache miss
 * and will force the CPU to perform a page dir lookup to read the new
 * address
 *
 * Ensures at compile time that `MapFlush::flush()` or
 * `MapFlush::ignore()` is called or the compiler will trow a warning
 */
#[must_use = "The page table must be flushed or ignored"]
pub struct MapFlush<S>
    where S: PageSize {
    m_virt: VirtFrame<S>,
    m_inner: MapFlusherInner<HwMapFlusher>
}

impl<S> MapFlush<S> where S: PageSize {
    /**
     * Constructs a new `MapFlush` which will flush the TLB entry for the
     * given `VirtFrame`
     */
    pub fn new(virt_frame: VirtFrame<S>) -> Self {
        Self { m_virt: virt_frame,
               m_inner: MapFlusherInner::new() }
    }
}

impl<S> MapFlusher for MapFlush<S> where S: PageSize {
    fn flush(self) {
        self.m_inner.flush_addr(self.m_virt)
    }
}

/**
 * Map flusher for an inclusive range of frames.
 *
 * It tells to the hardware TLB to remove the page table entries that
 * references the contiguous given `VirtFrame`s because they are changed.
 *
 * The next access to the same virtual locations will throw a TLB cache miss
 * and will force the CPU to perform a page dir lookup to read the new
 * addresses
 *
 * Ensures at compile time that `MapRangeFlush::flush()` or
 * `MapRangeFlush::ignore()` is called or the compiler will trow a warning
 */
#[must_use = "The page table must be flushed or ignored"]
pub struct MapRangeFlush<S>
    where S: PageSize {
    m_frame_range: VirtFrameRangeIncl<S>,
    m_inner: MapFlusherInner<HwMapFlusher>
}

impl<S> MapRangeFlush<S> where S: PageSize {
    /**
     * Constructs a `MapRangeFlush` which will flush the TLB entries for the
     * given `VirtFrameRangeIncl`
     */
    pub fn new(frame_range: VirtFrameRangeIncl<S>) -> Self {
        Self { m_frame_range: frame_range,
               m_inner: MapFlusherInner::new() }
    }

    /**
     * Returns whether the given range flusher is empty
     */
    pub fn is_empty(&self) -> bool {
        self.m_frame_range.is_empty()
    }
}

impl<S> MapFlusher for MapRangeFlush<S> where S: PageSize {
    fn flush(self) {
        if !self.m_frame_range.is_empty() {
            for virt_frame in self.m_frame_range {
                self.m_inner.flush_addr(virt_frame)
            }
        }
    }
}

/**
 * Flusher for the entire TLB.
 *
 * It tells to the hardware TLB to remove all the cached entries and restart
 * with a blank page.
 *
 * The next access to all the virtual addresses will throw a TLB cache miss
 * and will force the CPU to perform a page dir lookup to read the virtual
 * to physical conversion
 *
 * Ensures at compile time that `MapFlushAll::flush()` or
 * `MapFlushAll::ignore()` is called or the compiler will trow a warning
 */
#[must_use = "The page directory must be flushed or ignored"]
pub struct MapFlushAll {
    m_inner: MapFlusherInner<HwMapFlusher>
}

impl MapFlushAll {
    /**
     * Constructs a new `MapFlushAll` which will flush the entire TLB
     */
    pub fn new() -> Self {
        Self { m_inner: MapFlusherInner::new() }
    }
}

impl MapFlusher for MapFlushAll {
    fn flush(self) {
        self.m_inner.flush_all()
    }
}

/**
 * Encapsulates the hardware implementation of the map flusher and makes it
 * safe to use
 */
struct MapFlusherInner<T>
    where T: HwMapFlusherBase {
    m_hw_inner: T
}

impl<T> MapFlusherInner<T> where T: HwMapFlusherBase {
    /**
     * Constructs a new `MapFlusherInner`
     */
    fn new() -> Self {
        Self { m_hw_inner: T::new() }
    }

    /**
     * Flushes only the given `VirtFrame` table entry into the TLB
     */
    fn flush_addr<S>(&self, virt_frame: VirtFrame<S>)
        where S: PageSize {
        unsafe { self.m_hw_inner.flush_addr(virt_frame.start_addr()) }
    }

    /**
     * Flushes the entire TLB
     */
    fn flush_all(&self) {
        unsafe { self.m_hw_inner.flush_all() }
    }
}

/**
 * Common method interface for the map flusher
 */
pub trait MapFlusher {
    /**
     * It must flush the mapping for which the flusher was created
     */
    fn flush(self);

    /**
     * Forgets about this flusher and unsafely skips the TLB shot down
     */
    fn ignore(self)
        where Self: Sized {
        mem::forget(self);
    }
}

/**
 * Common interface on which `MapFlusherInner` relies to use the hardware
 * implementation of the flusher
 */
pub(crate) trait HwMapFlusherBase {
    /**  
     * Constructs a `HwMapFlusherBase` based obj
     */
    fn new() -> Self;

    /**
     * The hardware implementation must inform the hardware TLB that the
     * page table entry for the given `VirtAddr` is not more valid
     * because changed
     */
    unsafe fn flush_addr(&self, addr: VirtAddr);

    /**
     * The hardware implementation must inform the hardware TLB that all the
     * cached entries must become invalid after this call
     */
    unsafe fn flush_all(&self);
}
