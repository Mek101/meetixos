/*! Page directory page/page table allocator */

use shared::mem::paging::{
    allocator::FrameAllocator,
    frame::PhysFrame,
    Page4KiB
};

use crate::mem::phys::phys_alloc_frame;

/**
 * `FrameAllocator` implementation for hh_loader
 */
pub struct HHLPageDirAllocator;

impl FrameAllocator<Page4KiB> for HHLPageDirAllocator {
    fn alloc_page(&self) -> Option<PhysFrame<Page4KiB>> {
        phys_alloc_frame()
    }

    fn free_page(&self, _: PhysFrame<Page4KiB>) {
        panic!("HHLPageDirAllocator called to free a page")
    }

    fn alloc_page_table(&self) -> Option<PhysFrame<Page4KiB>> {
        phys_alloc_frame()
    }

    fn free_page_table(&self, _: PhysFrame<Page4KiB>) {
        panic!("HHLPageDirAllocator called to free a page table")
    }
}
