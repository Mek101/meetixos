/*! # Physical Address
 *
 * Implements a struct that represents a 64bit physical address
 */

use core::{
    convert::TryFrom,
    fmt,
    fmt::{Binary, Debug, LowerHex, Octal, UpperHex},
    ops::{Add, AddAssign, Sub, SubAssign}
};

use crate::hal::{
    addr::{Address, AddressErr, HwAddrBase},
    arch::addr::HwPhysAddr
};

/** # Physical Address
 *
 * Represents an hardware 64bit wide physical address.
 *
 * Encapsulates the hardware implementation of the physical address for the
 * compiling target architecture.
 *
 * The inner address is responsible to perform validity checks, eventual
 * truncating and creation
 */
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PhysAddr {
    m_hw_addr: HwPhysAddr
}

impl Address for PhysAddr {
    /** Constructs an unchecked `PhysAddr`
     */
    unsafe fn new_unchecked(raw_addr: usize) -> Self {
        Self { m_hw_addr: HwPhysAddr::new_unchecked(raw_addr) }
    }

    /** Returns the inner contained address as `usize`
     */
    fn as_usize(&self) -> usize {
        self.m_hw_addr.as_usize()
    }
}

impl Default for PhysAddr {
    /** Returns the "default value" for a type
     */
    fn default() -> Self {
        Self::new_zero()
    }
}

impl TryFrom<usize> for PhysAddr {
    /** The type returned in the event of a conversion error.
     */
    type Error = AddressErr;

    /** Performs the conversion.
     */
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        HwPhysAddr::try_from(value).map(|hw_addr| Self { m_hw_addr: hw_addr })
    }
}

impl Into<usize> for PhysAddr {
    /** Performs the conversion.
     */
    fn into(self) -> usize {
        self.as_usize()
    }
}

impl Debug for PhysAddr {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhysAddr({:#x})", self.as_usize())
    }
}

impl Binary for PhysAddr {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Binary::fmt(&self.as_usize(), f)
    }
}

impl Octal for PhysAddr {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Octal::fmt(&self.as_usize(), f)
    }
}

impl UpperHex for PhysAddr {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        UpperHex::fmt(&self.as_usize(), f)
    }
}

impl LowerHex for PhysAddr {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(&self.as_usize(), f)
    }
}

impl Add<usize> for PhysAddr {
    /** The resulting type after applying the `+` operator.
     */
    type Output = Self;

    /** Constructs a new `PhysAddr` and validates it before returning, so the
     * returned value could not be the bit a bit math add operation
     */
    fn add(self, rhs: usize) -> Self::Output {
        let mut new_addr = unsafe { Self::new_unchecked(self.as_usize() + rhs) };
        if !new_addr.m_hw_addr.is_valid() {
            new_addr.m_hw_addr.validate();
        }
        new_addr
    }
}

impl AddAssign<usize> for PhysAddr {
    /** Performs the `+=` operation.
     */
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs
    }
}

impl Add<Self> for PhysAddr {
    /** The resulting type after applying the `+` operator.
     */
    type Output = Self;

    /** Constructs a new `PhysAddr` and validates it before returning, so the
     * returned value could not be the bit a bit math add operation
     */
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.as_usize()
    }
}

impl AddAssign<Self> for PhysAddr {
    /** Performs the `+=` operation.
     */
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs.as_usize()
    }
}

impl Sub<usize> for PhysAddr {
    /** The resulting type after applying the `-` operator.
     */
    type Output = Self;

    /** Constructs a new `PhysAddr` and validates it before returning, so the
     * returned value could not be the bit a bit math sub operation
     */
    fn sub(self, rhs: usize) -> Self::Output {
        let mut new_addr = unsafe { Self::new_unchecked(self.as_usize() - rhs) };
        if !new_addr.m_hw_addr.is_valid() {
            new_addr.m_hw_addr.validate();
        }
        new_addr
    }
}

impl SubAssign<usize> for PhysAddr {
    /** Performs the `-=` operation.
     */
    fn sub_assign(&mut self, rhs: usize) {
        *self = *self - rhs
    }
}

impl Sub<Self> for PhysAddr {
    /** The resulting type after applying the `-` operator.
     */
    type Output = Self;

    /** Constructs a new `PhysAddr` and validates it before returning, so the
     * returned value could not be the bit a bit math sub operation
     */
    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.as_usize()
    }
}

impl SubAssign<Self> for PhysAddr {
    /** Performs the `-=` operation.
     */
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs.as_usize()
    }
}
