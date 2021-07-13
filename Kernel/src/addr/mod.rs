/*! Virtual & Physical address wrappers */

use core::{
    fmt::{
        Debug,
        Display
    },
    hash::Hash,
    ops::{
        Deref,
        Range
    }
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
     * Maximum value reachable by this `Address` implementation
     */
    const MAX: Self;

    /**
     * Constructs a null `Address`
     */
    #[inline]
    fn null() -> Self {
        Self::from(0)
    }

    /**
     * Returns the aligned up `Address` using the given `align`
     */
    #[inline]
    fn align_up<A>(&self, align: A) -> Self
        where A: Into<usize> {
        Self::from(align_up(**self, align.into()))
    }

    /**
     * Returns the aligned down `Address` using the given `align`
     */
    #[inline]
    fn align_down<A>(&self, align: A) -> Self
        where A: Into<usize> {
        Self::from(align_down(**self, align.into()))
    }

    /**
     * Returns this `Address` + the given `offset`
     */
    #[inline]
    fn offset(&self, offset: usize) -> Self {
        if offset > 0 {
            Self::from(**self + offset)
        } else {
            *self /* copy self */
        }
    }

    /**
     * Constructs a `Range<Self>` which start from this for `range_size`
     */
    fn to_range(&self, range_size: usize) -> Range<Self> {
        Range { start: self.clone(),
                end: self.offset(range_size) }
    }

    /**
     * Returns whether this `Address` is aligned with `align`
     */
    #[inline]
    fn is_aligned<A>(&self, align: A) -> bool
        where A: Into<usize> {
        self.align_down(align).eq(self)
    }

    /**
     * Returns whether this `Address` contains a zero value
     */
    #[inline]
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
    /**
     * Maximum value reachable by this `HwAddrBase` implementation
     */
    const MAX: Self;
}
