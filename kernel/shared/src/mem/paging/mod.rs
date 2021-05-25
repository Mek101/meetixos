/*! Paging management */

use core::fmt::Debug;

use crate::{
    arch::mem::paging::dir::HwPageDirSupport,
    mem::paging::{
        dir::HwPageDirSupportBase,
        table::PageTableLevel
    }
};

pub mod allocator;
pub mod dir;
pub mod flags;
pub mod flush;
pub mod frame;
pub mod table;

/**
 * Default 4KiB page-frame size
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Page4KiB;

impl PageSize for Page4KiB {
    const SIZE: usize = 4 * 1024;
    const MAP_LEVEL: PageTableLevel = HwPageDirSupport::PT_LEVEL_4KB;
    const IS_BIG: bool = false;
}

/**
 * 2MiB huge page-frame size
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Page2MiB;

impl PageSize for Page2MiB {
    const SIZE: usize = 2 * 1024 * 1024;
    const MAP_LEVEL: PageTableLevel = HwPageDirSupport::PT_LEVEL_2MB;
    const IS_BIG: bool = true;
}

/**
 * 1GiB huge page-frame size
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Page1GiB;

impl PageSize for Page1GiB {
    const SIZE: usize = 1 * 1024 * 1024 * 1024;
    const MAP_LEVEL: PageTableLevel = HwPageDirSupport::PT_LEVEL_1GB;
    const IS_BIG: bool = true;
}

/**
 * Base interface for the various supported page sizes
 */
pub trait PageSize: Debug + Copy + Clone + Eq + PartialEq + Ord + PartialOrd {
    /**
     * The size in bytes of this page size
     */
    const SIZE: usize;

    /**
     * The mask value to check this size
     */
    const MASK: usize = Self::SIZE - 1;

    /**
     * The `PageTableLevel` on which the page of this size can be mapped
     */
    const MAP_LEVEL: PageTableLevel;

    /**
     * Whether this page size needs `PTFlags::HUGE_PAGE`
     */
    const IS_BIG: bool;
}
