/*! Page directory page/page table allocator */

use crate::mem::phys::{
    phys_alloc_frame,
    phys_free_frame
};
use shared::{
    logger::trace,
    mem::paging::{
        allocator::FrameAllocator,
        frame::PhysFrame,
        Page4KiB,
        PageSize
    }
};

/**
 * `FrameAllocator` Implementation which doesn't allocates/de-allocates any
 * physical frame
 */
pub struct NoAllocator;

impl<S> FrameAllocator<S> for NoAllocator where S: PageSize {
    fn alloc_page(&self) -> Option<PhysFrame<S>> {
        None
    }

    fn free_page(&self, frame: PhysFrame<S>) {
        trace!("NoAllocator::free_page({:?})", frame);
    }

    fn alloc_page_table(&self) -> Option<PhysFrame<Page4KiB>> {
        None
    }

    fn free_page_table(&self, frame: PhysFrame<Page4KiB>) {
        trace!("NoAllocator::free_page_table({:?})", frame);
    }
}
/**
 * Default `FrameAllocator` implementation, allocates and de-allocates pages
 * and page tables
 */
pub struct KernAllocator {
    m_free_page_table: bool
}

impl KernAllocator {
    /**
     * Constructs a `KernAllocator`
     */
    pub const fn new() -> Self {
        Self::new_tweak(true)
    }

    /**
     * Constructs a `KernAllocator` which can choose whether
     * `free_page_table()` must mark as free or not the `PhysFrame` given
     */
    pub const fn new_tweak(free_page_table: bool) -> Self {
        Self { m_free_page_table: free_page_table }
    }
}

impl<S> FrameAllocator<S> for KernAllocator where S: PageSize {
    fn alloc_page(&self) -> Option<PhysFrame<S>> {
        phys_alloc_frame()
    }

    fn free_page(&self, frame: PhysFrame<S>) {
        trace!("KernAllocator::free_page({:?})", frame);
        phys_free_frame(frame);
    }

    fn alloc_page_table(&self) -> Option<PhysFrame<Page4KiB>> {
        phys_alloc_frame()
    }

    fn free_page_table(&self, frame: PhysFrame<Page4KiB>) {
        if self.m_free_page_table {
            trace!("KernAllocator::free_page_table({:?})", frame);
            phys_free_frame(frame);
        } else {
            trace!("KernAllocator::free_page_table({:?}) -> <IGNORED>", frame);
        }
    }
}
