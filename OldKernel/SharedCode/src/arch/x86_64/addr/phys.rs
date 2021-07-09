/*! x86_64 physical address implementation */

use core::convert::TryFrom;

use bits::bit_fields::BitFields;

use crate::addr::{
    AddressErr,
    HwAddrBase
};

/**
 * x86_64 physical address implementation
 */
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HwPhysAddr {
    m_raw_addr: usize
}

impl HwAddrBase for HwPhysAddr {
    fn new(raw_addr: usize) -> Self {
        Self { m_raw_addr: raw_addr % (1 << 52) }
    }

    fn as_usize(&self) -> usize {
        self.m_raw_addr
    }
}

impl TryFrom<usize> for HwPhysAddr {
    type Error = AddressErr;

    fn try_from(raw_addr: usize) -> Result<Self, Self::Error> {
        match raw_addr.bits_at(52..64) {
            0 => Ok(Self { m_raw_addr: raw_addr }),
            _ => Err(AddressErr(raw_addr))
        }
    }
}
