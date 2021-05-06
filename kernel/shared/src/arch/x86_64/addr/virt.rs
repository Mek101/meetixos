/*! x86_64 virtual address implementation */

use core::convert::TryFrom;

use bit_field::BitField;
use x86_64::VirtAddr;

use crate::{
    addr::{
        virt::HwVirtAddrBase,
        AddressErr,
        HwAddrBase
    },
    mem::paging::table::PageTableIndex
};

/**
 * x86_64 virtual address implementation.
 *
 * This wrapper ensures canonical addresses
 */
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct X64VirtAddr {
    m_addr: VirtAddr
}

impl HwAddrBase for X64VirtAddr {
    fn new(raw_addr: usize) -> Self {
        Self { m_addr: VirtAddr::new(raw_addr as u64) }
    }

    fn as_usize(&self) -> usize {
        self.m_addr.as_u64() as usize
    }
}

impl HwVirtAddrBase for X64VirtAddr {
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

    fn from_1gib_indices(l4_index: PageTableIndex, l3_index: PageTableIndex) -> Self {
        let mut addr = 0;
        addr.set_bits(39..48, Into::<usize>::into(l4_index) as u64);
        addr.set_bits(30..39, Into::<usize>::into(l3_index) as u64);

        Self { m_addr: VirtAddr::new(addr) }
    }

    fn level_4_index(&self) -> u16 {
        u16::from(self.m_addr.p4_index())
    }

    fn level_3_index(&self) -> u16 {
        u16::from(self.m_addr.p3_index())
    }

    fn level_2_index(&self) -> u16 {
        u16::from(self.m_addr.p2_index())
    }

    fn level_1_index(&self) -> u16 {
        u16::from(self.m_addr.p1_index())
    }
}

impl TryFrom<usize> for X64VirtAddr {
    type Error = AddressErr;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        VirtAddr::try_new(value as u64).map(|addr| Self { m_addr: addr })
                                       .map_err(|_| AddressErr(value))
    }
}
