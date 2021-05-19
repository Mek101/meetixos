/*! Page directory page/page table allocator */

use core::cell::UnsafeCell;

use shared::{
    addr::{
        phys::PhysAddr,
        Address
    },
    mem::paging::{
        allocator::FrameAllocator,
        frame::PhysFrame,
        Page2MiB,
        Page4KiB
    }
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

pub struct LinearAllocator {
    m_current_phys_frame: UnsafeCell<PhysFrame<Page2MiB>>
}

impl LinearAllocator {
    pub fn new_zero() -> Self {
        Self { m_current_phys_frame:
                   UnsafeCell::new(PhysAddr::new_zero().containing_frame()) }
    }
}

impl FrameAllocator<Page2MiB> for LinearAllocator {
    fn alloc_page(&self) -> Option<PhysFrame<Page2MiB>> {
        let phys_frame_ptr = self.m_current_phys_frame.get();
        let phys_frame = unsafe { *phys_frame_ptr };

        let phys_frame_mut = unsafe { &mut *phys_frame_ptr };
        *phys_frame_mut += 1;

        Some(phys_frame)
    }

    fn free_page(&self, _: PhysFrame<Page2MiB>) {
        panic!("LinearAllocator called to free a page")
    }

    fn alloc_page_table(&self) -> Option<PhysFrame<Page4KiB>> {
        phys_alloc_frame()
    }

    fn free_page_table(&self, _: PhysFrame<Page4KiB>) {
        panic!("LinearAllocator called to free a page table")
    }
}
