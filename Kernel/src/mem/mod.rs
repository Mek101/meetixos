/*! Kernel memory management */

use core::{
    fmt::Debug,
    hash::Hash
};

/**
 * Default 4KiB page-frame size
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct Page4KiB;

impl PageSize for Page4KiB {
    const SIZE: usize = 4096;
    const IS_HUGE: bool = false;
}

/**
 * Huge 2MiB page-frame size
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct Page2MiB;

impl PageSize for Page2MiB {
    const SIZE: usize = 2 * 1024 * 1024;
    const IS_HUGE: bool = true;
}

pub trait PageSize:
    Debug + Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Hash {
    /**
     * The size in bytes of this `PageSize`
     */
    const SIZE: usize;

    /**
     * Whether this `PageSize` needs is a huge `PageSize`
     */
    const IS_HUGE: bool;

    /**
     * The mask value to check this `PageSize` alignment
     */
    const MASK: usize = Self::SIZE - 1;

    /**
     * Returns the size in bytes of this `PageSize`
     */
    fn size(&self) -> usize {
        Self::SIZE
    }

    /**
     * Returns the mask value to check this `PageSize` alignment
     */
    fn mask(&self) -> usize {
        Self::MASK
    }

    /**
     * Returns whether this `PageSize` needs is a huge `PageSize`
     */
    fn is_huge(&self) -> bool {
        Self::IS_HUGE
    }
}
