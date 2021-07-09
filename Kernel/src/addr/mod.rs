/*! Virtual & Physical address wrappers */

use core::{
    fmt::{
        Debug,
        Display
    },
    hash::Hash,
    ops::Deref
};

use helps::align::{
    align_down,
    align_up
};

pub mod phys_addr;
pub mod virt_addr;

/**
 * Base interface of methods and dependencies common to all the addresses
 * implementations (both virtual and physical)
 */
pub trait Address:
    Default
    + From<usize>
    + Deref<Target = usize>
    + Copy
    + Clone
    + Debug
    + Display
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + Hash {
    /**
     * Constructs a null `Address`
     */
    fn null() -> Self {
        Self::from(0)
    }

    /**
     * Returns the aligned up `Address` using the given `align`
     */
    fn align_up<A>(&self, align: A) -> Self
        where A: Into<usize> {
        Self::from(align_up(**self, align.into()))
    }

    /**
     * Returns the aligned down `Address` using the given `align`
     */
    fn align_down<A>(&self, align: A) -> Self
        where A: Into<usize> {
        Self::from(align_down(**self, align.into()))
    }

    /**
     * Returns this `Address` + the given `offset`
     */
    fn offset(&self, offset: isize) -> Self {
        if offset > 0 {
            Self::from(**self + offset as usize)
        } else if offset < 0 {
            Self::from(**self - offset as usize)
        } else {
            *self /* copy self */
        }
    }

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
        **self == 0
    }
}

/**
 * Interface on which the `Address` trait implementors relies to use the
 * hardware implementation of the addresses
 */
pub trait HwAddrBase:
    From<usize>
    + Deref<Target = usize>
    + Copy
    + Clone
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + Hash {
    /* No additional methods are requested */
}
