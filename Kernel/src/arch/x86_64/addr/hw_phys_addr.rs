/*! x86_64 physical address implementation */

use core::{
    iter::Step,
    ops::Deref
};

use crate::addr::THwAddr;

/**
 * x86_64 physical address implementation
 */
#[repr(transparent)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct HwPhysAddr {
    m_raw_phys_addr: usize
}

impl THwAddr for HwPhysAddr {
    const MAX: Self = Self { m_raw_phys_addr: 0x000f_ffff_ffff_ffff };
}

impl From<usize> for HwPhysAddr {
    #[inline]
    fn from(raw_phys_addr: usize) -> Self {
        Self { m_raw_phys_addr: raw_phys_addr & 0x000f_ffff_ffff_ffff }
    }
}

impl Deref for HwPhysAddr {
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.m_raw_phys_addr
    }
}

impl Step for HwPhysAddr {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        usize::steps_between(&start.m_raw_phys_addr, &end.m_raw_phys_addr)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        if let Some(checked_raw_phys_addr) =
            usize::forward_checked(start.m_raw_phys_addr, count)
        {
            Some(Self { m_raw_phys_addr: checked_raw_phys_addr })
        } else {
            None
        }
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        if let Some(checked_raw_phys_addr) =
            usize::backward_checked(start.m_raw_phys_addr, count)
        {
            Some(Self { m_raw_phys_addr: checked_raw_phys_addr })
        } else {
            None
        }
    }
}
