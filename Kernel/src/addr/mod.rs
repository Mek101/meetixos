/*! Virtual & Physical address wrappers */

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
    hash::Hash,
    ops::{
        Add,
        AddAssign,
        Sub,
        SubAssign
    }
};

use helps::align::{
    align_down,
    align_up
};

pub mod phys;
pub mod virt;

/**
 * Base interface of methods and dependencies common to all the addresses
 * implementations (both virtual and physical)
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
    + PartialOrd
    + Hash {
    /**
     * Constructs a validated `Address`
     */
    fn new(raw_addr: usize) -> Self;

    /**
     * Returns the inner contained address as `usize`
     */
    fn as_usize(&self) -> usize;

    /**
     * Constructs a null `Address`
     */
    fn new_zero() -> Self {
        Self::new(0)
    }

    /**
     * Returns the aligned up address using the given `align`
     */
    fn align_up<A>(&self, align: A) -> Self
        where A: Into<usize> {
        Self::new(align_up(self.as_usize(), align.into()))
    }

    /**
     * Returns the aligned down address using the given `align`
     */
    fn align_down<A>(&self, align: A) -> Self
        where A: Into<usize> {
        Self::new(align_down(self.as_usize(), align.into()))
    }

    /**
     * Returns the containing `Frame` for this `Address`
     */
    // fn containing_frame<S>(&self) -> Frame<Self, S>
    //     where S: PageSize {
    //     Frame::of_addr(self.clone())
    // }

    /**
     * Returns whether this `Address` is aligned with `align`
     */
    fn is_aligned<A>(&self, align: A) -> bool
        where A: Into<usize> {
        self.align_down(align).eq(self)
    }

    /**
     * Returns whether this `Address` contains a zero value
     */
    fn is_null(&self) -> bool {
        self.as_usize() == 0
    }
}

/**
 * `Address` creation error.
 *
 * Internally contains a raw address `usize` with the error value given
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct AddressErr {
    pub m_raw_value: usize
}

impl fmt::Display for AddressErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The given address was not properly aligned ({:#X})", self.m_raw_value)
    }
}

/**
 * Interface on which the `Address` trait relies to use the hardware
 * implementation of the addresses
 */
pub trait HwAddrBase:
    TryFrom<usize, Error = AddressErr>
    + Copy
    + Clone
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + Hash {
    /**  
     * Constructs a validated `HwAddrBase` based `Address`
     *
     * The returned instance can be a truncated/normalized version of the
     * `raw_addr` for the underling architecture
     */
    fn new(raw_addr: usize) -> Self;

    /**
     * Returns this hardware address as `usize`
     */
    fn as_usize(&self) -> usize;
}
