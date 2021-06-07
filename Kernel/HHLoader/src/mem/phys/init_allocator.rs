/*! Initial physical memory allocator */

use shared::{
    addr::{
        phys::PhysAddr,
        Address
    },
    mem::paging::{
        frame::PhysFrame,
        Page4KiB,
        PageSize
    }
};

use crate::info::boot_info;

/**
 * Ugly and inefficient allocator that directly uses the `BootMemAreas` to
 * allocate `PhysFrame<Page4KiB>` frames
 */
pub(super) struct HHLPreInitAllocator {
    m_next_frame: usize
}

impl HHLPreInitAllocator {
    /**  
     * Constructs a `PreInitBootMemAllocator`
     */
    pub const fn new() -> Self {
        Self { m_next_frame: 0 }
    }

    /**
     * Sets the range of physical frames that the allocator must ignore
     */
    pub fn start_after(&mut self, first_usable_frame: PhysFrame<Page4KiB>) {
        self.m_next_frame = first_usable_frame.start_addr().as_usize() / Page4KiB::SIZE;
    }

    /**
     * Returns the first available `PhysFrame`
     */
    pub fn allocate(&mut self) -> Option<PhysFrame<Page4KiB>> {
        self.iter_to_next()?.next().map(|phys_frame| {
                                       self.m_next_frame += 1;
                                       phys_frame
                                   })
    }

    /**
     * Returns an `Iterator` to the next available `PhysFrame`
     */
    pub fn iter_to_next(&mut self) -> Option<impl Iterator<Item = PhysFrame<Page4KiB>>> {
        let mut it = boot_info().mem_areas()
                                .iter()
                                .map(|mem_area| {
                                    let start_addr = mem_area.start_phys_addr();
                                    let end_addr = start_addr + mem_area.size();

                                    /* create the range of integer addresses */
                                    start_addr.as_usize()..end_addr.as_usize()
                                })
                                .flat_map(|range| {
                                    /* divide the ranges into 4KiB frames */
                                    range.step_by(Page4KiB::SIZE)
                                })
                                .map(|raw_addr| {
                                    /* wrap the frame */
                                    PhysAddr::new(raw_addr).containing_frame()
                                });
        it.advance_by(self.m_next_frame).map(|_| it).ok()
    }

    /**
     * Returns the amount of allocated 4KiB frames
     */
    pub fn allocated_frames(&self) -> usize {
        self.m_next_frame
    }
}
