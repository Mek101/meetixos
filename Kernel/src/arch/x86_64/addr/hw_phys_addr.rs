/*! x86_64 physical address implementation */

use core::ops::Deref;

use crate::addr::HwAddrBase;

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

impl HwAddrBase for HwPhysAddr {
    /* No methods to implement */
}

impl From<usize> for HwPhysAddr {
    fn from(raw_phys_addr: usize) -> Self {
        Self { m_raw_phys_addr: raw_phys_addr % (1 << 52) }
    }
}

impl Deref for HwPhysAddr {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.m_raw_phys_addr
    }
}
