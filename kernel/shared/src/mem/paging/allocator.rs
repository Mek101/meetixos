/*! `PageDir` frame allocator */

use crate::mem::paging::{
    frame::PhysFrame,
    Page4KiB,
    PageSize
};

/**
 * Base interface for the `PhysFrame` management in `PageDir`
 */
pub trait FrameAllocator<S>
    where S: PageSize {
    /**  
     * Allocates a dynamically sized `PhysFrame` for a mapping page
     */
    fn alloc_page(&mut self) -> Option<PhysFrame<S>>;

    /**
     * Frees the given page `PhysFrame` which belonged to a page mapping
     */
    fn free_page(&mut self, frame: PhysFrame<S>);

    /**  
     * Allocate a `PhysFrame<Page4KiB>` for a page table
     */
    fn alloc_page_table(&mut self) -> Option<PhysFrame<Page4KiB>>;

    /**
     * Frees the given page table `PhysFrame<Page4KiB>` belonged to a page
     * table mapping
     */
    fn free_page_table(&mut self, frame: PhysFrame<Page4KiB>);
}
