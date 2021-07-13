/*! Kernel paging management */

use core::{
    fmt::Debug,
    hash::Hash
};

use crate::dbg::{
    C_KIB,
    C_MIB
};

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

impl PageSize for Page4KiB {
    const SIZE: usize = 4 * C_KIB;
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

impl PageSize for Page2MiB {
    const SIZE: usize = 2 * C_MIB;
    const IS_HUGE: bool = false;
}

pub trait PageSize:
    Debug + Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Hash {
    /**
     * The size in bytes for this kind of `PageSize`
     */
    const SIZE: usize;

    /**
     * Whether this `PageSize` needs huge-page flag
     */
    const IS_HUGE: bool;

    /**
     * The mask of the size
     */
    const MASK: usize = Self::SIZE - 1;
}
