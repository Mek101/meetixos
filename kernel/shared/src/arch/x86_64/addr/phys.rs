/*! x86_64 physical address implementation */

use core::convert::TryFrom;

use x86_64::PhysAddr;

use crate::addr::{
    AddressErr,
    HwAddrBase
};

/**
 * x86_64 physical address implementation
 */
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct X64PhysAddr {
    m_addr: PhysAddr
}

impl HwAddrBase for X64PhysAddr {
    fn new(raw_addr: usize) -> Self {
        Self { m_addr: PhysAddr::new_truncate(raw_addr as u64) }
    }

    fn as_usize(&self) -> usize {
        self.m_addr.as_u64() as usize
    }
}

impl TryFrom<usize> for X64PhysAddr {
    type Error = AddressErr;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        PhysAddr::try_new(value as u64).map(|addr| Self { m_addr: addr })
                                       .map_err(|_| AddressErr(value))
    }
}
