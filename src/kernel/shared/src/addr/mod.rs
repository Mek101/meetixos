/*! # Virtual & Physical Address
 *
 * Implements the abstraction of the memory addresses into his two different
 * types: physical and virtual
 */

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

pub use phys::*;
pub use virt::*;

use crate::mem::paging::{
    Frame,
    PageSize
};

mod phys;
mod virt;

/** # Address Base
 *
 * Defines a base interface of methods and dependencies common to all the
 * addresses implementations (both virtual and physical)
 */
pub trait Address:
    Default
    + TryFrom<usize, Error = AddressErr>
    + Into<usize>
    + Copy
    + Clone
    + Debug
    + Binary
    + Octal
    + UpperHex
    + LowerHex
    + Add<usize, Output = Self>
    + AddAssign<usize>
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<usize, Output = Self>
    + SubAssign<usize>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Eq
    + PartialEq
    + Ord
    + PartialOrd {
    /** # Constructs an unchecked `Address`
     *
     * The returned address implementation may be invalid
     */
    unsafe fn new_unchecked(addr: usize) -> Self;

    /** Returns the inner contained address as `usize`
     */
    fn as_usize(&self) -> usize;

    /** # Constructs a null `Address`
     *
     * The returned address is null
     */
    fn new_zero() -> Self {
        unsafe { Self::new_unchecked(0) }
    }

    /** # Aligns up this `Address`
     *
     * Returns on [`Ok`] the aligned up address using the given `align`
     *
     * [`Ok`]: core::result::Result::Ok
     */
    fn align_up<A>(self, align: A) -> Result<Self, AddressErr>
        where A: Into<usize> {
        Self::try_from(align_up(self.into(), align.into()))
    }

    /** # Aligns down this `Address`
     *
     * Returns on [`Ok`] the aligned down address using the given `align`
     *
     * [`Ok`]: core::result::Result::Ok
     */
    fn align_down<A>(self, align: A) -> Result<Self, AddressErr>
        where A: Into<usize> {
        Self::try_from(align_down(self.into(), align.into()))
    }

    /** Returns the containing [`Frame`] for this `Address`
     *
     * [`Frame`]: /hal/paging/struct.Frame.html
     */
    fn containing_frame<S>(&self) -> Frame<Self, S>
        where S: PageSize {
        Frame::of_addr(*self)
    }

    /** Returns whether this `Address` is aligned with `align`
     */
    fn is_aligned<A>(&self, align: A) -> bool
        where A: Into<usize> {
        Self::from(*self).align_down(align)
                         .map(|aligned| aligned == *self)
                         .unwrap_or(false)
    }

    /** Returns whether this `Address` contains a zero value
     */
    fn is_null(&self) -> bool {
        self.as_usize() == 0
    }
}

/** # Address Error
 *
 * Represents an [`Address`] creation error.
 *
 * Internally contains a raw address `usize` with the error value given
 *
 * [`Address`]: /hal/addr/trait.Address.html
 */
#[derive(Debug, Copy, Clone)]
pub struct AddressErr(pub(crate) usize);

impl fmt::Display for AddressErr {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "The given address was not properly aligned ({:#X})", self.0)
    }
}

/** # Align the raw address down
 *
 * Returns the `addr` align down to the nearest value multiple of `align`
 */
pub const fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}

/** # Align the raw address up
 *
 * Returns the `addr` align up to the nearest value multiple of `align`
 */
pub const fn align_up(addr: usize, align: usize) -> usize {
    let align_mask = align - 1;
    if addr & align_mask != 0 {
        (addr | align_mask) + 1
    } else {
        addr
    }
}

/** # Hardware Address Base
 *
 * Defines the interface on which the [`Address`] trait relies to use the
 * hardware implementation of the addresses
 *
 * [`Address`]: /hal/addr/trait.Address.html
 */
pub(crate) trait HwAddrBase: TryFrom<usize, Error = AddressErr> {
    /** # Constructs an unchecked `HwAddrBase` based `Address`
     *
     * The returned instance could be invalid for the underling
     * architecture, no validity check must be performed inside this
     * method
     */
    unsafe fn new_unchecked(raw_addr: usize) -> Self;

    /** # Validate this HW address
     *
     * The instance must be made valid in if it doesn't for the underling
     * hardware architecture
     */
    fn validate(&mut self);

    /** Returns this hardware address as `usize`
     */
    fn as_usize(&self) -> usize;

    /** Returns whether this hardware address is valid
     */
    fn is_valid(&self) -> bool;
}
