/*! Physical address */

use core::{
    fmt,
    fmt::{
        Debug,
        Display
    },
    iter::Step,
    ops::Deref
};

use crate::{
    addr::{
        TAddress,
        THwAddr
    },
    arch::addr::hw_phys_addr::HwPhysAddr
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
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct PhysAddr {
    m_hw_phys_addr: HwPhysAddr
}

impl TAddress for PhysAddr {
    const MAX: Self = Self { m_hw_phys_addr: HwPhysAddr::MAX };
}

impl Default for PhysAddr {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl From<usize> for PhysAddr {
    #[inline]
    fn from(raw_phys_addr: usize) -> Self {
        Self { m_hw_phys_addr: HwPhysAddr::from(raw_phys_addr) }
    }
}

impl Deref for PhysAddr {
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.m_hw_phys_addr.deref()
    }
}

impl Debug for PhysAddr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhysAddr({:#018x})", **self)
    }
}

impl Display for PhysAddr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04x}", (**self & 0xffff_0000_0000_0000) >> 48)?;
        write!(f, "_{:04x}", (**self & 0x0000_ffff_0000_0000) >> 32)?;
        write!(f, "_{:04x}", (**self & 0x0000_0000_ffff_0000) >> 16)?;
        write!(f, "_{:04x}", **self & 0x0000_0000_0000_ffff)
    }
}

impl Step for PhysAddr {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        HwPhysAddr::steps_between(&start.m_hw_phys_addr, &end.m_hw_phys_addr)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        if let Some(check_phys_addr) =
            HwPhysAddr::forward_checked(start.m_hw_phys_addr, count)
        {
            Some(Self { m_hw_phys_addr: check_phys_addr })
        } else {
            None
        }
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        if let Some(check_phys_addr) =
            HwPhysAddr::backward_checked(start.m_hw_phys_addr, count)
        {
            Some(Self { m_hw_phys_addr: check_phys_addr })
        } else {
            None
        }
    }
}
