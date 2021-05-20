/*! Initial physical memory allocator */

use core::ops::Range;

use shared::{
    addr::{
        phys::PhysAddr,
        Address
    },
    dbg::MIB,
    mem::paging::{
        frame::{
            PhysFrame,
            PhysFrameRange
        },
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
    m_to_skip: Range<usize>,
    m_next_frame: usize
}

impl HHLPreInitAllocator {
    /**  
     * Constructs a `PreInitBootMemAllocator`
     */
    pub const fn new() -> Self {
        Self { m_to_skip: Range { start: 0,
                                  end: 0 },
               /* keep the first MiB reserved */
               m_next_frame: MIB / Page4KiB::SIZE }
    }

    /**
     * Sets the range of physical frames that the allocator must ignore
     */
    pub fn skip_range(&mut self, to_skip_frame_range: PhysFrameRange<Page4KiB>) {
        let raw_start_addr = to_skip_frame_range.start.start_addr().as_usize();
        let raw_end_addr = to_skip_frame_range.end.start_addr().as_usize();

        self.m_to_skip = Range { start: raw_start_addr / Page4KiB::SIZE,
                                 end: raw_end_addr / Page4KiB::SIZE };
    }

    /**
     * Returns the first available `PhysFrame`
     */
    pub fn allocate(&mut self) -> Option<PhysFrame<Page4KiB>> {
        self.iter_to_next()?.next().map(|phys_frame| {
                                       self.m_next_frame += 1;
                                       if self.m_next_frame == self.m_to_skip.start {
                                           self.m_next_frame = self.m_to_skip.end
                                       }
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
}
