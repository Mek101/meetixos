use hal::paging::{FrameAllocator, Page4KiB, PageSize, PhysFrame, PhysFrameRange};

use crate::mem::phys::{phys_mem_alloc_frame, phys_mem_free_frame};

macro_rules! impl_hal_frame_allocator {
    ($AllocatorName:ident : $PageSize:ident) => {
        impl FrameAllocator<$PageSize> for $AllocatorName {
            fn alloc_page(&mut self) -> Option<PhysFrame<$PageSize>> {
                self.kern_alloc_page()
            }

            fn free_page(&mut self, frame: PhysFrame<$PageSize>) {
                self.kern_free_page(frame)
            }

            fn alloc_page_table(&mut self) -> Option<PhysFrame<Page4KiB>> {
                self.alloc_stats_mut().alloc_table(|| phys_mem_alloc_frame())
            }

            fn free_page_table(&mut self, frame: PhysFrame<Page4KiB>) {
                self.alloc_stats_mut().free_table(|| phys_mem_free_frame(frame))
            }

            fn allocated_pages(&self) -> usize {
                self.alloc_stats().m_allocated_pages
            }

            fn freed_pages(&self) -> usize {
                self.alloc_stats().m_freed_pages
            }

            fn allocated_page_tables(&self) -> usize {
                self.alloc_stats().m_allocated_tables
            }

            fn freed_page_tables(&self) -> usize {
                self.alloc_stats().m_freed_tables
            }
        }
    };
}

pub struct KernAllocator {
    m_stats: FrameAllocatorStats
}

impl KernAllocator {
    pub const fn new() -> Self {
        Self { m_stats: FrameAllocatorStats::new() }
    }
}

impl KernFrameAllocator<Page4KiB> for KernAllocator {
    fn alloc_stats(&self) -> &FrameAllocatorStats {
        &self.m_stats
    }

    fn alloc_stats_mut(&mut self) -> &mut FrameAllocatorStats {
        &mut self.m_stats
    }
}

impl_hal_frame_allocator!(KernAllocator: Page4KiB);

pub struct RangeAllocator {
    m_stats: FrameAllocatorStats,
    m_phys_frame_range: PhysFrameRange<Page4KiB>
}

impl RangeAllocator {
    pub const fn new(phys_frame_range: PhysFrameRange<Page4KiB>) -> Self {
        Self { m_stats: FrameAllocatorStats::new(),
               m_phys_frame_range: phys_frame_range }
    }
}

impl KernFrameAllocator<Page4KiB> for RangeAllocator {
    fn alloc_stats(&self) -> &FrameAllocatorStats {
        &self.m_stats
    }

    fn alloc_stats_mut(&mut self) -> &mut FrameAllocatorStats {
        &mut self.m_stats
    }

    fn kern_alloc_page(&mut self) -> Option<PhysFrame<Page4KiB>> {
        self.m_phys_frame_range.next()
    }

    fn kern_free_page(&mut self, _phys_frame: PhysFrame<Page4KiB>) {
        panic!("Freeing a page using RangeAllocator")
    }
}

impl_hal_frame_allocator!(RangeAllocator: Page4KiB);

struct FrameAllocatorStats {
    m_allocated_tables: usize,
    m_allocated_pages: usize,
    m_freed_tables: usize,
    m_freed_pages: usize
}

impl FrameAllocatorStats {
    const fn new() -> Self {
        Self { m_allocated_tables: 0,
               m_allocated_pages: 0,
               m_freed_tables: 0,
               m_freed_pages: 0 }
    }

    fn alloc_page<S, F>(&mut self, allocate: F) -> Option<PhysFrame<S>>
        where S: PageSize,
              F: FnOnce() -> Option<PhysFrame<S>> {
        allocate().map(|phys_frame| {
                      self.m_allocated_tables += 1;
                      phys_frame
                  })
    }

    fn free_page<F>(&mut self, free: F)
        where F: FnOnce() {
        free();
        self.m_freed_pages += 1;
    }

    fn alloc_table<F>(&mut self, allocate: F) -> Option<PhysFrame<Page4KiB>>
        where F: FnOnce() -> Option<PhysFrame<Page4KiB>> {
        allocate().map(|phys_frame| {
                      self.m_allocated_tables += 1;
                      phys_frame
                  })
    }

    fn free_table<F>(&mut self, free: F)
        where F: FnOnce() {
        free();
        self.m_freed_pages += 1;
    }
}

trait KernFrameAllocator<S>: FrameAllocator<S>
    where S: PageSize {
    fn alloc_stats(&self) -> &FrameAllocatorStats;
    fn alloc_stats_mut(&mut self) -> &mut FrameAllocatorStats;

    fn kern_alloc_page(&mut self) -> Option<PhysFrame<S>> {
        self.alloc_stats_mut().alloc_page(|| phys_mem_alloc_frame())
    }

    fn kern_free_page(&mut self, phys_frame: PhysFrame<S>) {
        self.alloc_stats_mut().free_page(|| phys_mem_free_frame(phys_frame))
    }
}
