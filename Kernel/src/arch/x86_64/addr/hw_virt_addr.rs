/*! x86_64 virtual address implementation */

use core::{
    iter::Step,
    ops::Deref
};

use bits::bit_fields::BitFields;

use crate::{
    addr::{
        virt_addr::HwVirtAddrBase,
        HwAddrBase
    },
    vm::page_table::PageTableLevel
};

/**
 * x86_64 virtual address implementation.
 *
 * This wrapper ensures canonical addresses
 */
#[repr(transparent)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct HwVirtAddr {
    m_raw_virt_addr: usize
}

impl HwVirtAddrBase for HwVirtAddr {
    const BITS_PER_TABLE_LEVEL: usize = 9;

    fn raw_table_index_for_level(&self, page_table_level: PageTableLevel) -> u16 {
        match page_table_level {
            PageTableLevel::Root => self.m_raw_virt_addr.bits_at(39..48) as u16,
            PageTableLevel::OneGiB => self.m_raw_virt_addr.bits_at(30..39) as u16,
            PageTableLevel::TwoMiB => self.m_raw_virt_addr.bits_at(21..30) as u16,
            PageTableLevel::FourKiB => self.m_raw_virt_addr.bits_at(12..21) as u16
        }
    }
}

impl HwAddrBase for HwVirtAddr {
    const MAX: Self = Self { m_raw_virt_addr: 0x0000_ffff_ffff_ffff };
}

impl From<usize> for HwVirtAddr {
    #[inline]
    fn from(raw_virt_addr: usize) -> Self {
        Self { m_raw_virt_addr: ((raw_virt_addr << 16) as isize >> 16) as usize }
    }
}

impl Deref for HwVirtAddr {
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.m_raw_virt_addr
    }
}

impl Step for HwVirtAddr {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        usize::steps_between(&start.m_raw_virt_addr, &end.m_raw_virt_addr)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        if let Some(checked_raw_virt_addr) =
            usize::forward_checked(start.m_raw_virt_addr, count)
        {
            Some(Self { m_raw_virt_addr: checked_raw_virt_addr })
        } else {
            None
        }
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        if let Some(checked_raw_virt_addr) =
            usize::backward_checked(start.m_raw_virt_addr, count)
        {
            Some(Self { m_raw_virt_addr: checked_raw_virt_addr })
        } else {
            None
        }
    }
}
