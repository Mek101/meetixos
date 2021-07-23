/*! Virtual & Physical address wrappers */

use core::{
    fmt::{
        Debug,
        Display
    },
    hash::Hash,
    iter::Step,
    ops::{
        Deref,
        Range
    }
};

use helps::align::{
    align_down,
    align_up
};

use crate::vm::TPageSize;

pub mod phys_addr;
pub mod virt_addr;

/**
 * Base interface of methods and dependencies common to all the addresses
 * implementations (both virtual and physical)
 */
pub trait TAddress:
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
    + Hash
    + Step {
    /**
     * Maximum value reachable by this `TAddress` implementation
     */
    const MAX: Self;

    /**
     * Constructs a null `TAddress`
     */
    #[inline]
    fn null() -> Self {
        Self::from(0)
    }

    /**
     * Returns the aligned up `TAddress` using the given `align`
     */
    #[inline]
    fn align_up<A>(&self, align: A) -> Self
        where A: Into<usize> {
        Self::from(align_up(**self, align.into()))
    }

    /**
     * Returns the aligned down `TAddress` using the given `align`
     */
    #[inline]
    fn align_down<A>(&self, align: A) -> Self
        where A: Into<usize> {
        Self::from(align_down(**self, align.into()))
    }

    /**
     * Returns this `TAddress` + the given `offset`
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
     * Returns this `TAddress` as Page index
     */
    fn as_page_index<S>(&self) -> usize
        where S: TPageSize {
        **self / S::SIZE
    }

    /**
     * Returns whether this `TAddress` is aligned with `align`
     */
    #[inline]
    fn is_aligned<A>(&self, align: A) -> bool
        where A: Into<usize> {
        self.align_down(align).eq(self)
    }

    /**
     * Returns whether this `TAddress` contains a zero value
     */
    #[inline]
    fn is_null(&self) -> bool {
        **self == 0
    }
}

/**
 * Interface on which the `TAddress` trait implementors relies to use the
 * hardware implementation of the addresses
 */
pub trait THwAddr:
    From<usize>
    + Deref<Target = usize>
    + Copy
    + Clone
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + Hash
    + Step {
    /**
     * Maximum value reachable by this `HwAddrBase` implementation
     */
    const MAX: Self;
}
