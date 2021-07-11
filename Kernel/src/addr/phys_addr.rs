/*! Physical address */

use core::{
    fmt,
    fmt::{
        Debug,
        Display
    },
    ops::Deref
};

use crate::{
    addr::Address,
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

impl Address for PhysAddr {
    /* No methods to implement */
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
        write!(f, "{:#018x}", **self)
    }
}
