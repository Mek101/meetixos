/*! Kernel virtual memory layout */

use core::fmt;

use crate::{
    addr::{
        virt::VirtAddr,
        Address
    },
    dbg::dbg_display_size,
    mem::paging::{
        frame::{
            VirtFrame,
            VirtFrameRange
        },
        PageSize
    }
};

/**
 * Fixed collection of `VMLayoutArea` which defines the kernel core's
 * virtual memory layout
 */
#[derive(Debug, Clone)]
pub struct VMLayout {
    m_kern_text_area: VMLayoutArea,
    m_kern_heap_area: VMLayoutArea,
    m_kern_stack_area: VMLayoutArea,
    m_phys_mem_bitmap_area: VMLayoutArea,
    m_phys_mem_mapping_area: VMLayoutArea,
    m_page_cache_area: VMLayoutArea,
    m_tmp_map_area: VMLayoutArea
}

impl VMLayout {
    /**
     * Constructs a `VMLayout` filled with the given `VMLayoutArea`s
     */
    pub const fn new(kern_text_area: VMLayoutArea,
                     kern_heap_area: VMLayoutArea,
                     kern_stack_area: VMLayoutArea,
                     phys_mem_bitmap_area: VMLayoutArea,
                     phys_mem_mapping_area: VMLayoutArea,
                     page_cache_area: VMLayoutArea,
                     tmp_map_area: VMLayoutArea)
                     -> Self {
        Self { m_kern_text_area: kern_text_area,
               m_kern_heap_area: kern_heap_area,
               m_kern_stack_area: kern_stack_area,
               m_phys_mem_bitmap_area: phys_mem_bitmap_area,
               m_phys_mem_mapping_area: phys_mem_mapping_area,
               m_page_cache_area: page_cache_area,
               m_tmp_map_area: tmp_map_area }
    }

    /**
     * Constructs a zero-filled `VMLayout`
     */
    pub fn new_zero() -> Self {
        Self { m_kern_heap_area: VMLayoutArea::new_zero(),
               m_kern_text_area: VMLayoutArea::new_zero(),
               m_kern_stack_area: VMLayoutArea::new_zero(),
               m_phys_mem_bitmap_area: VMLayoutArea::new_zero(),
               m_phys_mem_mapping_area: VMLayoutArea::new_zero(),
               m_page_cache_area: VMLayoutArea::new_zero(),
               m_tmp_map_area: VMLayoutArea::new_zero() }
    }

    /**
     * Returns the reference to the `VMLayoutArea` of kernel's text
     */
    pub fn kern_text_area(&self) -> &VMLayoutArea {
        &self.m_kern_text_area
    }

    /**
     * Returns the reference to the `VMLayoutArea` of kernel's heap
     */
    pub fn kern_heap_area(&self) -> &VMLayoutArea {
        &self.m_kern_heap_area
    }

    /**
     * Returns the reference to the `VMLayoutArea` of kernel's stack
     */
    pub fn kern_stack_area(&self) -> &VMLayoutArea {
        &self.m_kern_stack_area
    }

    /**
     * Returns the reference to the `VMLayoutArea` of physical memory bitmap
     */
    pub fn phys_mem_bitmap_area(&self) -> &VMLayoutArea {
        &self.m_phys_mem_bitmap_area
    }

    /**
     * Returns the reference to the `VMLayoutArea` of physical memory
     * mapping
     */
    pub fn phys_mem_mapping_area(&self) -> &VMLayoutArea {
        &self.m_phys_mem_mapping_area
    }

    /**
     * Returns the reference to the `VMLayoutArea` of page cache
     */
    pub fn page_cache_area(&self) -> &VMLayoutArea {
        &self.m_page_cache_area
    }

    /**
     * Returns the reference to the `VMLayoutArea` of temporary mapping
     */
    pub fn tmp_map_area(&self) -> &VMLayoutArea {
        &self.m_tmp_map_area
    }
}

impl fmt::Display for VMLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "Kernel Text:       {}\nKernel Heap:       {}\nKernel Stack:      \
                {}\nPhysMem Bitmap:    {}\nPhysMem Mapping:   {}\nDisk Page Cache:   \
                {}\nTemporary Mapping: {}",
               self.m_kern_text_area,
               self.m_kern_heap_area,
               self.m_kern_stack_area,
               self.m_phys_mem_bitmap_area,
               self.m_phys_mem_mapping_area,
               self.m_page_cache_area,
               self.m_tmp_map_area)
    }
}

/**
 * Virtual memory area for the kernel's layout
 */
#[derive(Debug, Copy, Clone)]
pub struct VMLayoutArea {
    m_start_addr: VirtAddr,
    m_size: usize
}

impl VMLayoutArea {
    /**  
     * Constructs a `VMLayoutArea` filled with the given data
     */
    pub const fn new(start_addr: VirtAddr, size: usize) -> Self {
        Self { m_start_addr: start_addr,
               m_size: size }
    }

    /**
     * Constructs a zero-filled `VMLayoutArea`
     */
    pub fn new_zero() -> Self {
        Self { m_start_addr: VirtAddr::new_zero(),
               m_size: 0 }
    }

    /**
     * Returns the start `VirtAddr`
     */
    pub fn start_addr(&self) -> VirtAddr {
        self.m_start_addr
    }

    /**
     * Returns the size of the area in bytes
     */
    pub fn size(&self) -> usize {
        self.m_size
    }

    /**
     * Returns the end `VirtAddr`
     */
    pub fn end_addr(&self) -> VirtAddr {
        self.m_start_addr + self.m_size
    }

    /**
     * Returns this `VMLayoutArea` as `VirtFrameRange`
     */
    pub fn as_frame_range<S>(&self) -> VirtFrameRange<S>
        where S: PageSize {
        assert_eq!(self.m_size % S::SIZE, 0);

        let start_frame = self.start_addr().containing_frame();
        VirtFrame::range_of(start_frame, start_frame + self.m_size / S::SIZE)
    }

    /**
     * Returns layout area as his parts
     */
    pub fn as_parts(&self) -> (VirtAddr, VirtAddr) {
        (self.start_addr(), self.end_addr())
    }
}

impl fmt::Display for VMLayoutArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "{:x}..{:x} ({})",
               self.start_addr(),
               self.end_addr(),
               dbg_display_size(self.size()))
    }
}
