/*! # x86_64 Physical Address
 *
 * Implements the abstraction of the x86_64 physical address
 */

use core::convert::TryFrom;

use x86_64::PhysAddr;

use crate::hal::addr::{AddressErr, HwAddrBase};

/** # x86_64 Physical Address
 *
 * Implements the concrete physical address for the x86_64 architecture.
 *
 * The type ensures that the stored addresses are valid when validated with
 * [`X64PhysAddr::validate()`] using the [`x86_64::PhysAddr`]
 *
 * [`X64PhysAddr::validate()`]:
 * /hal/arch/x86_64/addr/struct.X64PhysAddr.html#method.validate
 * [`x86_64::PhysAddr`]: https://docs.rs/x86_64/0.13.2/x86_64/addr/struct.PhysAddr.html
 */
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct X64PhysAddr {
    m_addr: PhysAddr
}

impl HwAddrBase for X64PhysAddr {
    /** Constructs an unchecked `X64PhysAddr`
     */
    unsafe fn new_unchecked(raw_addr: usize) -> Self {
        Self { m_addr: PhysAddr::new_unsafe(raw_addr as u64) }
    }

    /** Truncates the bits from the 52 to the 64
     */
    fn validate(&mut self) {
        *self = if let Ok(addr) = Self::try_from(self.as_usize()) {
            addr
        } else {
            Self { m_addr: PhysAddr::new_truncate(self.m_addr.as_u64()) }
        };
    }

    /** Returns the inner contained address as `usize`
     */
    fn as_usize(&self) -> usize {
        self.m_addr.as_u64() as usize
    }

    /** Returns whether this address doesn't contains active bits into the
     * range `52..64`
     */
    fn is_valid(&self) -> bool {
        if let Ok(_) = PhysAddr::try_new(self.m_addr.as_u64()) {
            true
        } else {
            false
        }
    }
}

impl TryFrom<usize> for X64PhysAddr {
    /** The type returned in the event of a conversion error.
     */
    type Error = AddressErr;

    /** Performs the conversion.
     */
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        PhysAddr::try_new(value as u64).map(|addr| Self { m_addr: addr })
                                       .map_err(|_| AddressErr(value))
    }
}
