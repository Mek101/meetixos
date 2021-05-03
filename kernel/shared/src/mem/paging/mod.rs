/*! # HAL Paging
 *
 * Implements the structures and the functions useful for managing the
 * pagination in a common way
 */

pub use allocator::*;
pub use dir::*;
pub use flush::*;
pub use frame::*;
pub use table::*;

mod allocator;
mod dir;
mod flush;
mod frame;
mod table;

use core::fmt::Debug;

use crate::arch::mem::paging::HwPageDirSupport;

/** # 4KiB Page Size
 *
 * Represents the 4KiB page size.
 *
 * This is the default page size used for almost all the page mappings
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Page4KiB;

impl PageSize for Page4KiB {
    /** The size in bytes of this page size
     */
    const SIZE: usize = 4 * 1024;

    /** Number of bits used to represent this PageSize
     */
    const USED_BITS: usize = 12;

    /** The [`PageTableLevel`] on which the page of this size can be mapped
     *
     * [`PageTableLevel`]: /hal/paging/enum.PageTableLevel.html
     */
    const MAP_LEVEL: PageTableLevel = HwPageDirSupport::PT_LEVEL_4KB;

    /** Whether this page size needs [`PTFlags::HUGE_PAGE`]
     *
     * [`PTFlags::HUGE_PAGE`]:
     * /hal/paging/struct.PTFlags.html#associatedconstant.HUGE_PAGE
     */
    const IS_BIG: bool = false;
}

/** # 2MiB Page Size
 *
 * Represents the 2MiB page size
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Page2MiB;

impl PageSize for Page2MiB {
    /** The size in bytes of this page size
     */
    const SIZE: usize = 2 * 1024 * 1024;

    /** Number of bits used to represent this PageSize
     */
    const USED_BITS: usize = 22;

    /** The [`PageTableLevel`] on which the page of this size can be mapped
     *
     * [`PageTableLevel`]: /hal/paging/enum.PageTableLevel.html
     */
    const MAP_LEVEL: PageTableLevel = HwPageDirSupport::PT_LEVEL_2MB;

    /** Whether this page size needs [`PTFlags::HUGE_PAGE`]
     *
     * [`PTFlags::HUGE_PAGE`]:
     * /hal/paging/struct.PTFlags.html#associatedconstant.HUGE_PAGE
     */
    const IS_BIG: bool = true;
}

/** # 1GiB Page Size
 *
 * Represents the 1GiB page size
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Page1GiB;

impl PageSize for Page1GiB {
    /** The size in bytes of this page size
     */
    const SIZE: usize = 1 * 1024 * 1024 * 1024;

    /** Number of bits used to represent this PageSize
     */
    const USED_BITS: usize = 31;

    /** The [`PageTableLevel`] on which the page of this size can be mapped
     *
     * [`PageTableLevel`]: /hal/paging/enum.PageTableLevel.html
     */
    const MAP_LEVEL: PageTableLevel = HwPageDirSupport::PT_LEVEL_1GB;

    /** Whether this page size needs [`PTFlags::HUGE_PAGE`]
     *
     * [`PTFlags::HUGE_PAGE`]:
     * /hal/paging/struct.PTFlags.html#associatedconstant.HUGE_PAGE
     */
    const IS_BIG: bool = true;
}

/** # Page Size Base
 *
 * Defines the base interface for the various supported page sizes
 */
pub trait PageSize: Debug + Copy + Clone + Eq + PartialEq + Ord + PartialOrd {
    /** The size in bytes of this page size
     */
    const SIZE: usize;

    /** The mask value to check this size
     */
    const MASK: usize = Self::SIZE - 1;

    /** Number of bits used to represent this PageSize
     */
    const USED_BITS: usize;

    /** The [`PageTableLevel`] on which the page of this size can be mapped
     *
     * [`PageTableLevel`]: /hal/paging/enum.PageTableLevel.html
     */
    const MAP_LEVEL: PageTableLevel;

    /** Whether this page size needs [`PTFlags::HUGE_PAGE`]
     *
     * [`PTFlags::HUGE_PAGE`]:
     * /hal/paging/struct.PTFlags.html#associatedconstant.HUGE_PAGE
     */
    const IS_BIG: bool;
}
