/*! Physical address */

use core::{
    convert::TryFrom,
    fmt,
    fmt::{
        Binary,
        Debug,
        LowerHex,
        Octal,
        UpperHex
    },
    ops::{
        Add,
        AddAssign,
        Sub,
        SubAssign
    }
};

use crate::{
    addr::{
        Address,
        AddressErr,
        HwAddrBase
    },
    arch::addr::phys::HwPhysAddr
};

/**
 * Hardware 64bit wide physical address.
 *
 * Encapsulates the hardware implementation of the physical address for the
 * compiling target architecture.
 *
 * The inner address is responsible to perform validity checks, eventual
 * truncating and creation
 */
#[repr(transparent)]
#[derive(Hash)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
pub struct PhysAddr {
    m_hw_addr: HwPhysAddr
}

impl Address for PhysAddr {
    fn new(raw_addr: usize) -> Self {
        Self { m_hw_addr: HwPhysAddr::new(raw_addr) }
    }

    fn as_usize(&self) -> usize {
        self.m_hw_addr.as_usize()
    }
}

impl Default for PhysAddr {
    fn default() -> Self {
        Self::new_zero()
    }
}

impl TryFrom<usize> for PhysAddr {
    type Error = AddressErr;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        HwPhysAddr::try_from(value).map(|hw_addr| Self { m_hw_addr: hw_addr })
    }
}

impl Into<usize> for PhysAddr {
    fn into(self) -> usize {
        self.as_usize()
    }
}

impl Debug for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhysAddr({:#018x})", self.as_usize())
    }
}

impl Binary for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Binary::fmt(&self.as_usize(), f)
    }
}

impl Octal for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Octal::fmt(&self.as_usize(), f)
    }
}

impl UpperHex for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#018X}", self.as_usize())
    }
}

impl LowerHex for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#018x}", self.as_usize())
    }
}

impl Add<usize> for PhysAddr {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::new(self.as_usize() + rhs)
    }
}

impl AddAssign<usize> for PhysAddr {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs
    }
}

impl Add<Self> for PhysAddr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.as_usize()
    }
}

impl AddAssign<Self> for PhysAddr {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs.as_usize()
    }
}

impl Sub<usize> for PhysAddr {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self::new(self.as_usize() - rhs)
    }
}

impl SubAssign<usize> for PhysAddr {
    fn sub_assign(&mut self, rhs: usize) {
        *self = *self - rhs
    }
}

impl Sub<Self> for PhysAddr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.as_usize()
    }
}

impl SubAssign<Self> for PhysAddr {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs.as_usize()
    }
}
