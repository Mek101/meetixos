/*! Kernel virtual memory management */

use core::{
    fmt::Debug,
    hash::Hash
};

use helps::dbg::{
    C_KIB,
    C_MIB
};

use crate::vm::page_table::PageTableLevel;

pub mod layout_manager;
pub mod mem_manager;
pub mod page_dir;
pub mod page_table;
pub mod page_table_entry;

/**
 * Default 4KiB `PageSize`
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct Page4KiB;

impl TPageSize for Page4KiB {
    const SIZE: usize = 4 * C_KIB;
    const PAGE_TABLE_LEVEL: PageTableLevel = PageTableLevel::FourKiB;
    const IS_HUGE: bool = false;
}

/**
 * Huge 2MiB `PageSize`
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct Page2MiB;

impl TPageSize for Page2MiB {
    const SIZE: usize = 2 * C_MIB;
    const PAGE_TABLE_LEVEL: PageTableLevel = PageTableLevel::TwoMiB;
    const IS_HUGE: bool = false;
}

pub trait TPageSize:
    Debug + Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Hash {
    /**
     * The size in bytes for this kind of `PageSize`
     */
    const SIZE: usize;

    /**
     * How many page-table levels to reach the mapping for this page-size
     */
    const PAGE_TABLE_LEVEL: PageTableLevel;

    /**
     * Whether this `PageSize` needs huge-page flag
     */
    const IS_HUGE: bool;

    /**
     * The mask of the size
     */
    const MASK: usize = Self::SIZE - 1;
}
