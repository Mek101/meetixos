/*! # HAL Kernel Virtual Memory Layout
 *
 * Implements a descriptor which contains the virtual layout for the kernel,
 * prepared by the higher half loader. It is provided via [`BootInfos`]
 * instead of using static constants to use dynamic kernel layout (Meltdown
 * mitigation)
 *
 * [`BootInfos`]: struct.BootInfos.html
 */

use crate::addr::{Address, VirtAddr};

/** # Kernel Virtual Memory Layout
 *
 * Stores the collection of [`VMLayoutArea`] which defines the kernel core's
 * virtual memory layout
 *
 * [`VMLayoutArea`]: struct.VMLayoutArea.html
 */
#[derive(Debug, Clone)]
pub struct VMLayout {
    m_kern_text_area: VMLayoutArea,
    m_kern_heap_area: VMLayoutArea,
    m_phys_mem_bitmap_area: VMLayoutArea,
    m_phys_mem_mapping_area: VMLayoutArea,
    m_page_cache_area: VMLayoutArea,
    m_tmp_map_area: VMLayoutArea
}

impl VMLayout {
    /** # Constructs a `VMLayout`
     *
     * The returned instance is filled with the given [`VMLayoutArea`]s
     *
     * [`VMLayoutArea`]: struct.VMLayoutArea.html
     */
    pub const fn new(kern_text_area: VMLayoutArea,
                     kern_heap_area: VMLayoutArea,
                     phys_mem_bitmap_area: VMLayoutArea,
                     phys_mem_mapping_area: VMLayoutArea,
                     page_cache_area: VMLayoutArea,
                     tmp_map_area: VMLayoutArea)
                     -> Self {
        Self { m_kern_text_area: kern_text_area,
               m_kern_heap_area: kern_heap_area,
               m_phys_mem_bitmap_area: phys_mem_bitmap_area,
               m_phys_mem_mapping_area: phys_mem_mapping_area,
               m_page_cache_area: page_cache_area,
               m_tmp_map_area: tmp_map_area }
    }

    /** # Constructs a zero-filled `VMLayout`
     *
     * The returned instance is zero-filled
     */
    pub fn new_zero() -> Self {
        Self { m_kern_heap_area: VMLayoutArea::new_zero(),
               m_kern_text_area: VMLayoutArea::new_zero(),
               m_phys_mem_bitmap_area: VMLayoutArea::new_zero(),
               m_phys_mem_mapping_area: VMLayoutArea::new_zero(),
               m_page_cache_area: VMLayoutArea::new_zero(),
               m_tmp_map_area: VMLayoutArea::new_zero() }
    }

    /** Returns the reference to the [`VMLayoutArea`] of kernel's text
     *
     * [`VMLayoutArea`]: struct.VMLayoutArea.html
     */
    pub fn kern_text_area(&self) -> &VMLayoutArea {
        &self.m_kern_text_area
    }

    /** Returns the reference to the [`VMLayoutArea`] of kernel's heap
     *
     * [`VMLayoutArea`]: struct.VMLayoutArea.html
     */
    pub fn kern_heap_area(&self) -> &VMLayoutArea {
        &self.m_kern_heap_area
    }

    /** Returns the reference to the [`VMLayoutArea`] of physical memory
     * bitmap
     *
     * [`VMLayoutArea`]: struct.VMLayoutArea.html
     */
    pub fn phys_mem_bitmap_area(&self) -> &VMLayoutArea {
        &self.m_phys_mem_bitmap_area
    }

    /** Returns the reference to the [`VMLayoutArea`] of physical memory
     * mapping
     *
     * [`VMLayoutArea`]: struct.VMLayoutArea.html
     */
    pub fn phys_mem_mapping_area(&self) -> &VMLayoutArea {
        &self.m_phys_mem_mapping_area
    }

    /** Returns the reference to the [`VMLayoutArea`] of page cache
     *
     * [`VMLayoutArea`]: struct.VMLayoutArea.html
     */
    pub fn page_cache_area(&self) -> &VMLayoutArea {
        &self.m_page_cache_area
    }

    /** Returns the reference to the [`VMLayoutArea`] of temporary mapping
     *
     * [`VMLayoutArea`]: struct.VMLayoutArea.html
     */
    pub fn tmp_map_area(&self) -> &VMLayoutArea {
        &self.m_tmp_map_area
    }
}

/** # Virtual Memory Layout Area
 *
 * Represents a virtual memory area for the kernel's layout
 */
#[derive(Debug, Clone)]
pub struct VMLayoutArea {
    m_start_addr: VirtAddr,
    m_size: usize
}

impl VMLayoutArea {
    /** # Constructs a `VMLayoutArea`
     *
     * The returned instance is filled with the given data
     */
    pub const fn new(start_addr: VirtAddr, size: usize) -> Self {
        Self { m_start_addr: start_addr,
               m_size: size }
    }

    /** # Constructs a zero-filled `VMLayoutArea`
     *
     * The returned instance is zero-filled
     */
    pub fn new_zero() -> Self {
        Self { m_start_addr: VirtAddr::new_zero(),
               m_size: 0 }
    }

    /** Returns the start [`VirtAddr`]
     *
     * [`VirtAddr`]: struct.VirtAddr.html
     */
    pub fn start_addr(&self) -> VirtAddr {
        self.m_start_addr
    }

    /** Returns the size of the area in bytes
     */
    pub fn size(&self) -> usize {
        self.m_size
    }

    /** Returns the end [`VirtAddr`]
     *
     * [`VirtAddr`]: struct.VirtAddr.html
     */
    pub fn end_addr(&self) -> VirtAddr {
        self.m_start_addr + self.m_size
    }
}
