/*! Virtual address */

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

use crate::{
    addr::{
        Address,
        AddressErr,
        HwAddrBase
    },
    arch::addr::HwVirtAddr,
    mem::paging::table::PageTableIndex
};

/**
 * Hardware 64bit wide virtual address.
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
    /**
     * Constructs a `VirtAddr` from the given 4 page table indices
     */
    pub fn from_4kib_indices(l4_index: PageTableIndex,
                             l3_index: PageTableIndex,
                             l2_index: PageTableIndex,
                             l1_index: PageTableIndex)
                             -> Self {
        Self { m_hw_addr: HwVirtAddr::from_4kib_indices(l4_index, l3_index, l2_index,
                                                        l1_index) }
    }

    /**
     * Constructs a `VirtAddr` from the given 3 page table indices
     */
    pub fn from_2mib_indices(l4_index: PageTableIndex,
                             l3_index: PageTableIndex,
                             l2_index: PageTableIndex)
                             -> Self {
        Self { m_hw_addr: HwVirtAddr::from_2mib_indices(l4_index, l3_index, l2_index) }
    }

    /**
     * Constructs a `VirtAddr` from the given 2 page table indices
     */
    pub fn from_1gib_indices(l4_index: PageTableIndex, l3_index: PageTableIndex) -> Self {
        Self { m_hw_addr: HwVirtAddr::from_1gib_indices(l4_index, l3_index) }
    }

    /**
     * Returns this virtual address as constant raw pointer
     */
    pub fn as_ptr<T>(self) -> *const T {
        self.as_usize() as *const T
    }

    /**
     * Returns this virtual address as mutable raw pointer
     */
    pub fn as_ptr_mut<T>(&self) -> *mut T {
        self.as_ptr::<T>() as *mut T
    }

    /**
     * Returns the fourth level `PageTableIndex`
     */
    pub fn level_4_index(&self) -> PageTableIndex {
        PageTableIndex::new(self.m_hw_addr.level_4_index())
    }

    /**
     * Returns the third level `PageTableIndex`
     */
    pub fn level_3_index(&self) -> PageTableIndex {
        PageTableIndex::new(self.m_hw_addr.level_3_index())
    }

    /**
     * Returns the second level `PageTableIndex`
     */
    pub fn level_2_index(&self) -> PageTableIndex {
        PageTableIndex::new(self.m_hw_addr.level_2_index())
    }

    /**
     * Returns the first level `PageTableIndex`
     */
    pub fn level_1_index(&self) -> PageTableIndex {
        PageTableIndex::new(self.m_hw_addr.level_1_index())
    }
}

impl Address for VirtAddr {
    fn new(raw_addr: usize) -> Self {
        Self { m_hw_addr: HwVirtAddr::new(raw_addr) }
    }

    fn as_usize(&self) -> usize {
        self.m_hw_addr.as_usize()
    }
}

impl Default for VirtAddr {
    fn default() -> Self {
        Self::new_zero()
    }
}

impl<T> From<*const T> for VirtAddr {
    fn from(raw_ptr: *const T) -> Self {
        Self::new(raw_ptr as usize)
    }
}

impl TryFrom<usize> for VirtAddr {
    type Error = AddressErr;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        HwVirtAddr::try_from(value).map(|hw_addr| Self { m_hw_addr: hw_addr })
    }
}

impl Into<usize> for VirtAddr {
    fn into(self) -> usize {
        self.as_usize()
    }
}

impl Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VirtAddr({:#018x})", self.as_usize())
    }
}

impl Binary for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Binary::fmt(&self.as_usize(), f)
    }
}

impl Octal for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Octal::fmt(&self.as_usize(), f)
    }
}

impl UpperHex for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#018X}", self.as_usize())
    }
}

impl LowerHex for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#018x}", self.as_usize())
    }
}

impl Add<usize> for VirtAddr {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::new(self.as_usize() + rhs)
    }
}

impl AddAssign<usize> for VirtAddr {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs
    }
}

impl Add<Self> for VirtAddr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.as_usize()
    }
}

impl AddAssign<Self> for VirtAddr {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs.as_usize()
    }
}

impl Sub<usize> for VirtAddr {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self::new(self.as_usize() - rhs)
    }
}

impl SubAssign<usize> for VirtAddr {
    fn sub_assign(&mut self, rhs: usize) {
        *self = *self - rhs
    }
}

impl Sub<Self> for VirtAddr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.as_usize()
    }
}

impl SubAssign<Self> for VirtAddr {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs.as_usize()
    }
}

/**
 * Interface on which the `VirtAddr` relies to export his methods related to
 * paging
 */
pub(crate) trait HwVirtAddrBase: HwAddrBase {
    /**
     * Constructs a `VirtAddr` from the given 4 page table indices
     */
    fn from_4kib_indices(l4_index: PageTableIndex,
                         l3_index: PageTableIndex,
                         l2_index: PageTableIndex,
                         l1_index: PageTableIndex)
                         -> Self;

    /**
     * Constructs a `VirtAddr` from the given 3 page table indices
     */
    fn from_2mib_indices(l4_index: PageTableIndex,
                         l3_index: PageTableIndex,
                         l2_index: PageTableIndex)
                         -> Self;

    /**
     * Constructs a `VirtAddr` from the given 2 page table indices
     */
    fn from_1gib_indices(l4_index: PageTableIndex, l3_index: PageTableIndex) -> Self;

    /**
     * Returns the raw fourth level page table entry index
     */
    fn level_4_index(&self) -> u16;

    /**
     * Returns the raw third level page table entry index
     */
    fn level_3_index(&self) -> u16;

    /**
     * Returns the raw second level page table entry index
     */
    fn level_2_index(&self) -> u16;

    /**
     * Returns the raw first level page table entry index
     */
    fn level_1_index(&self) -> u16;
}
