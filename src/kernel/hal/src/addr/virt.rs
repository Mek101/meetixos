/*! # Virtual Address
 *
 * Implements a struct that represents a 64bit virtual address
 */

use core::{
    convert::TryFrom,
    fmt,
    fmt::{Binary, Debug, LowerHex, Octal, UpperHex},
    ops::{Add, AddAssign, Sub, SubAssign}
};

use crate::{
    addr::{Address, AddressErr, HwAddrBase},
    arch::addr::HwVirtAddr,
    paging::PageTableIndex
};

/** # Virtual Address
 *
 * Represents an hardware 64bit wide virtual address.
 *
 * Encapsulates the hardware implementation of the virtual address for the
 * compiling target architecture.
 *
 * The inner address is responsible to perform validity checks, eventual
 * truncating and creation
 */
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct VirtAddr {
    m_hw_addr: HwVirtAddr
}

impl VirtAddr {
    /** # Constructs a `VirtAddr`
     *
     * The address returned is composed by the given 4 table indexes
     */
    pub fn from_4kib_indices(l4_index: PageTableIndex,
                             l3_index: PageTableIndex,
                             l2_index: PageTableIndex,
                             l1_index: PageTableIndex)
                             -> Self {
        Self { m_hw_addr: HwVirtAddr::from_4kib_indices(l4_index, l3_index, l2_index,
                                                        l1_index) }
    }

    /** # Constructs a `VirtAddr`
     *
     * The address returned is composed by the given 3 table indexes
     */
    pub fn from_2mib_indices(l4_index: PageTableIndex,
                             l3_index: PageTableIndex,
                             l2_index: PageTableIndex)
                             -> Self {
        Self { m_hw_addr: HwVirtAddr::from_2mib_indices(l4_index, l3_index, l2_index) }
    }

    /** # Constructs a `VirtAddr`
     *
     * The address returned is composed by the given 2 table indexes
     */
    pub fn from_1gib_indices(l4_index: PageTableIndex, l3_index: PageTableIndex) -> Self {
        Self { m_hw_addr: HwVirtAddr::from_1gib_indices(l4_index, l3_index) }
    }

    /** Returns this virtual address as constant raw pointer
     */
    pub fn as_ptr<T>(self) -> *const T {
        self.as_usize() as *const T
    }

    /** Returns this virtual address as mutable raw pointer
     */
    pub fn as_ptr_mut<T>(&self) -> *mut T {
        self.as_ptr::<T>() as *mut T
    }

    /** Returns the fourth level [`PageTableIndex`]
     *
     * [`PageTableIndex`]: /hal/paging/struct.PageTableIndex.html
     */
    pub fn level_4_index(&self) -> PageTableIndex {
        PageTableIndex::new(self.m_hw_addr.level_4_index())
    }

    /** Returns the third level [`PageTableIndex`]
     *
     * [`PageTableIndex`]: /hal/paging/struct.PageTableIndex.html
     */
    pub fn level_3_index(&self) -> PageTableIndex {
        PageTableIndex::new(self.m_hw_addr.level_3_index())
    }

    /** Returns the second level [`PageTableIndex`]
     *
     * [`PageTableIndex`]: /hal/paging/struct.PageTableIndex.html
     */
    pub fn level_2_index(&self) -> PageTableIndex {
        PageTableIndex::new(self.m_hw_addr.level_2_index())
    }

    /** Returns the first level [`PageTableIndex`]
     *
     * [`PageTableIndex`]: /hal/paging/struct.PageTableIndex.html
     */
    pub fn level_1_index(&self) -> PageTableIndex {
        PageTableIndex::new(self.m_hw_addr.level_1_index())
    }
}

impl Address for VirtAddr {
    /** Constructs an unchecked `VirtAddr`
     */
    unsafe fn new_unchecked(raw_addr: usize) -> Self {
        Self { m_hw_addr: HwVirtAddr::new_unchecked(raw_addr) }
    }

    /** Returns the inner contained address as `usize`
     */
    fn as_usize(&self) -> usize {
        self.m_hw_addr.as_usize()
    }
}

impl Default for VirtAddr {
    /** Returns the "default value" for a type
     */
    fn default() -> Self {
        Self::new_zero()
    }
}

impl<T> From<*const T> for VirtAddr {
    /** Performs the conversion.
     */
    fn from(raw_ptr: *const T) -> Self {
        unsafe { Self::new_unchecked(raw_ptr as usize) }
    }
}

impl TryFrom<usize> for VirtAddr {
    /** The type returned in the event of a conversion error.
     */
    type Error = AddressErr;

    /** Performs the conversion.
     */
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        HwVirtAddr::try_from(value).map(|hw_addr| Self { m_hw_addr: hw_addr })
    }
}

impl Into<usize> for VirtAddr {
    /** Performs the conversion.
     */
    fn into(self) -> usize {
        self.as_usize()
    }
}

impl Debug for VirtAddr {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VirtAddr({:#x})", self.as_usize())
    }
}

impl Binary for VirtAddr {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Binary::fmt(&self.as_usize(), f)
    }
}

impl Octal for VirtAddr {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Octal::fmt(&self.as_usize(), f)
    }
}

impl UpperHex for VirtAddr {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        UpperHex::fmt(&self.as_usize(), f)
    }
}

impl LowerHex for VirtAddr {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(&self.as_usize(), f)
    }
}

impl Add<usize> for VirtAddr {
    /** The resulting type after applying the `+` operator.
     */
    type Output = Self;

    /** Constructs a new `VirtAddr` and validates it before returning, so the
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

impl AddAssign<usize> for VirtAddr {
    /** Performs the `+=` operation.
     */
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs
    }
}

impl Add<Self> for VirtAddr {
    /** The resulting type after applying the `+` operator.
     */
    type Output = Self;

    /** Constructs a new `VirtAddr` and validates it before returning, so the
     * returned value could not be the bit a bit math add operation
     */
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.as_usize()
    }
}

impl AddAssign<Self> for VirtAddr {
    /** Performs the `+=` operation.
     */
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs.as_usize()
    }
}

impl Sub<usize> for VirtAddr {
    /** The resulting type after applying the `-` operator.
     */
    type Output = Self;

    /** Constructs a new `VirtAddr` and validates it before returning, so the
     * returned value could not be the bit a bit math sub operation
     */
    fn sub(self, rhs: usize) -> Self::Output {
        let mut new_addr = unsafe { Self::new_unchecked(self.as_usize() + rhs) };
        if !new_addr.m_hw_addr.is_valid() {
            new_addr.m_hw_addr.validate();
        }
        new_addr
    }
}

impl SubAssign<usize> for VirtAddr {
    /** Performs the `-=` operation.
     */
    fn sub_assign(&mut self, rhs: usize) {
        *self = *self - rhs
    }
}

impl Sub<Self> for VirtAddr {
    /** The resulting type after applying the `-` operator.
     */
    type Output = Self;

    /** Constructs a new `VirtAddr` and validates it before returning, so the
     * returned value could not be the bit a bit math sub operation
     */
    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.as_usize()
    }
}

impl SubAssign<Self> for VirtAddr {
    /** Performs the `-=` operation.
     */
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs.as_usize()
    }
}

/** # Hardware Virtual Address Base
 *
 * Defines the interface on which the [`VirtAddr`] relies to export his
 * methods related to paging
 *
 * [`VirtAddr`]: /hal/addr/struct.VirtAddr.html
 */
pub(crate) trait HwVirtAddrBase: HwAddrBase {
    /** # Constructs an hardware virtual address
     *
     * The address returned is composed by the given 4 table indexes
     */
    fn from_4kib_indices(l4_index: PageTableIndex,
                         l3_index: PageTableIndex,
                         l2_index: PageTableIndex,
                         l1_index: PageTableIndex)
                         -> Self;

    /** # Constructs an hardware virtual address
     *
     * The address returned is composed by the given 3 table indexes
     */
    fn from_2mib_indices(l4_index: PageTableIndex,
                         l3_index: PageTableIndex,
                         l2_index: PageTableIndex)
                         -> Self;

    /** # Constructs an hardware virtual address
     *
     * The address returned is composed by the given 2 table indexes
     */
    fn from_1gib_indices(l4_index: PageTableIndex, l3_index: PageTableIndex) -> Self;

    /** Returns the raw fourth level page table entry index
     */
    fn level_4_index(&self) -> u16;

    /** Returns the raw third level page table entry index
     */
    fn level_3_index(&self) -> u16;

    /** Returns the raw second level page table entry index
     */
    fn level_2_index(&self) -> u16;

    /** Returns the raw first level page table entry index
     */
    fn level_1_index(&self) -> u16;
}
