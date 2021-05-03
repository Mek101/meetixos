/*! # x86_64 Virtual Address
 *
 * Implements the abstraction of the x86_64 virtual address
 */

use core::convert::TryFrom;

use bit_field::BitField;
use x86_64::VirtAddr;

use crate::{
    addr::{
        AddressErr,
        HwAddrBase,
        HwVirtAddrBase
    },
    mem::paging::PageTableIndex
};

/** # x86_64 Physical Address
 *
 * Implements the concrete physical address for the x86_64 architecture.
 *
 * This type ensures that the addresses are always represented as
 * `canonical` addresses because wraps an [`x86_64::VirtAddr`] which
 * guarantees the same constriction
 *
 * [`x86_64::VirtAddr`]: x86_64::addr::VirtAddr
 */
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct X64VirtAddr {
    m_addr: VirtAddr
}

impl HwAddrBase for X64VirtAddr {
    /** Constructs an unchecked `X64VirtAddr`
     */
    unsafe fn new_unchecked(raw_addr: usize) -> Self {
        Self { m_addr: VirtAddr::new_unsafe(raw_addr as u64) }
    }

    /** Validates the inner [`x86_64::VirtAddr`] truncating it if not
     * `canonical`
     *
     * [`x86_64::VirtAddr`]: x86_64::addr::VirtAddr
     */
    fn validate(&mut self) {
        *self = if let Ok(addr) = Self::try_from(self.as_usize()) {
            addr
        } else {
            Self { m_addr: VirtAddr::new_truncate(self.m_addr.as_u64()) }
        };
    }

    /** Returns the inner contained address as usize
     */
    fn as_usize(&self) -> usize {
        self.m_addr.as_u64() as usize
    }

    /** Returns whether this address is in `canonical` form
     */
    fn is_valid(&self) -> bool {
        if let Ok(_) = VirtAddr::try_new(self.m_addr.as_u64()) {
            true
        } else {
            false
        }
    }
}

impl HwVirtAddrBase for X64VirtAddr {
    /** Constructs an hardware virtual address
     */
    fn from_4kib_indices(l4_index: PageTableIndex,
                         l3_index: PageTableIndex,
                         l2_index: PageTableIndex,
                         l1_index: PageTableIndex)
                         -> Self {
        let mut addr = 0;
        addr.set_bits(39..48, Into::<usize>::into(l4_index) as u64);
        addr.set_bits(30..39, Into::<usize>::into(l3_index) as u64);
        addr.set_bits(21..30, Into::<usize>::into(l2_index) as u64);
        addr.set_bits(12..21, Into::<usize>::into(l1_index) as u64);

        Self { m_addr: VirtAddr::new(addr) }
    }

    /** Constructs an hardware virtual address
     */
    fn from_2mib_indices(l4_index: PageTableIndex,
                         l3_index: PageTableIndex,
                         l2_index: PageTableIndex)
                         -> Self {
        let mut addr = 0;
        addr.set_bits(39..48, Into::<usize>::into(l4_index) as u64);
        addr.set_bits(30..39, Into::<usize>::into(l3_index) as u64);
        addr.set_bits(21..30, Into::<usize>::into(l2_index) as u64);

        Self { m_addr: VirtAddr::new(addr) }
    }

    /** Constructs an hardware virtual address
     */
    fn from_1gib_indices(l4_index: PageTableIndex, l3_index: PageTableIndex) -> Self {
        let mut addr = 0;
        addr.set_bits(39..48, Into::<usize>::into(l4_index) as u64);
        addr.set_bits(30..39, Into::<usize>::into(l3_index) as u64);

        Self { m_addr: VirtAddr::new(addr) }
    }

    /** Returns the raw fourth level page table entry index
     */
    fn level_4_index(&self) -> u16 {
        u16::from(self.m_addr.p4_index())
    }

    /** Returns the raw third level page table entry index
     */
    fn level_3_index(&self) -> u16 {
        u16::from(self.m_addr.p3_index())
    }

    /** Returns the raw second level page table entry index
     */
    fn level_2_index(&self) -> u16 {
        u16::from(self.m_addr.p2_index())
    }

    /** Returns the raw first level page table entry index
     */
    fn level_1_index(&self) -> u16 {
        u16::from(self.m_addr.p1_index())
    }
}

impl TryFrom<usize> for X64VirtAddr {
    /** The type returned in the event of a conversion error.
     */
    type Error = AddressErr;

    /** Performs the conversion.
     */
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        VirtAddr::try_new(value as u64).map(|addr| Self { m_addr: addr })
                                       .map_err(|_| AddressErr(value))
    }
}
