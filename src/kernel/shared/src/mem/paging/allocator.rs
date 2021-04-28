/*! # HAL Frame Allocator
 *
 * Exposes the trait used by the paging module to allocate physical frames
 * and page tables
 */

use crate::mem::paging::{Page4KiB, PageSize, PhysFrame};

/** # Frame Allocator Base
 *
 * Defines the methods that are used by the paging module to allocate/free
 * page frames or page table frames when mapping memory
 */
pub trait FrameAllocator<S>
    where S: PageSize {
    /** # Allocate a `PhysFrame` for a page
     *
     * The returned [`PhysFrame`] is usable as valid physical address for
     * mapping a page
     *
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     */
    fn alloc_page(&mut self) -> Option<PhysFrame<S>>;

    /** # Frees the given page `PhysFrame`
     *
     * The given `frame` belonged to a page mapping
     */
    fn free_page(&mut self, frame: PhysFrame<S>);

    /** # Allocate a `PhysFrame` for a page table
     *
     * The returned [`PhysFrame`] is usable as valid physical address for
     * mapping a page table
     *
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     */
    fn alloc_page_table(&mut self) -> Option<PhysFrame<Page4KiB>>;

    /** # Frees the given page table `PhysFrame`
     *
     * The given `frame` belonged to a page table mapping
     */
    fn free_page_table(&mut self, frame: PhysFrame<Page4KiB>);

    /** Returns the number of [`PhysFrame`]s allocated via
     * [`FrameAllocator::alloc_page()`]
     *
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     * [`FrameAllocator::alloc_page()`]:
     * /hal/paging/trait.FrameAllocator.html#method.alloc_page
     */
    fn allocated_pages(&self) -> usize {
        0
    }

    /** Returns the number of [`PhysFrame`]s freed via
     * [`FrameAllocator::free_page()`]
     *
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     * [`FrameAllocator::free_page()`]:
     * /hal/paging/trait.FrameAllocator.html#method.free_page
     */
    fn freed_pages(&self) -> usize {
        0
    }

    /** Returns the number of [`PhysFrame`]s allocated via
     * [`FrameAllocator::alloc_page_table()`]
     *
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     * [`FrameAllocator::alloc_page()`]:
     * /hal/paging/trait.FrameAllocator.html#method.alloc_page_table
     */
    fn allocated_page_tables(&self) -> usize {
        0
    }

    /** Returns the number of [`PhysFrame`]s freed via
     * [`FrameAllocator::free_page_table()`]
     *
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     * [`FrameAllocator::free_page()`]:
     * /hal/paging/trait.FrameAllocator.html#method.free_page_table
     */
    fn freed_page_tables(&self) -> usize {
        0
    }
}
