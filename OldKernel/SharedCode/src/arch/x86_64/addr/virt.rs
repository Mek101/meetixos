/*! x86_64 virtual address implementation */

use core::convert::TryFrom;

use bits::bit_fields::BitFields;

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
pub struct HwVirtAddr {
    m_raw_addr: usize
}

impl HwAddrBase for HwVirtAddr {
    fn new(raw_addr: usize) -> Self {
        Self { m_raw_addr: ((raw_addr << 16) as isize >> 16) as usize }
    }

    fn as_usize(&self) -> usize {
        self.m_raw_addr
    }
}

impl HwVirtAddrBase for HwVirtAddr {
    fn from_4kib_indices(l4_index: PageTableIndex,
                         l3_index: PageTableIndex,
                         l2_index: PageTableIndex,
                         l1_index: PageTableIndex)
                         -> Self {
        let mut raw_addr = 0;
        raw_addr.set_bits(39..48, l4_index.into());
        raw_addr.set_bits(30..39, l3_index.into());
        raw_addr.set_bits(21..30, l2_index.into());
        raw_addr.set_bits(12..21, l1_index.into());

        Self { m_raw_addr: raw_addr }
    }

    fn from_2mib_indices(l4_index: PageTableIndex,
                         l3_index: PageTableIndex,
                         l2_index: PageTableIndex)
                         -> Self {
        let mut raw_addr = 0;
        raw_addr.set_bits(39..48, l4_index.into());
        raw_addr.set_bits(30..39, l3_index.into());
        raw_addr.set_bits(21..30, l2_index.into());

        Self { m_raw_addr: raw_addr }
    }

    fn from_1gib_indices(l4_index: PageTableIndex, l3_index: PageTableIndex) -> Self {
        let mut raw_addr = 0;
        raw_addr.set_bits(39..48, l4_index.into());
        raw_addr.set_bits(30..39, l3_index.into());

        Self { m_raw_addr: raw_addr }
    }

    fn level_4_index(&self) -> u16 {
        self.m_raw_addr.bits_at(39..48) as u16
    }

    fn level_3_index(&self) -> u16 {
        self.m_raw_addr.bits_at(30..39) as u16
    }

    fn level_2_index(&self) -> u16 {
        self.m_raw_addr.bits_at(21..30) as u16
    }

    fn level_1_index(&self) -> u16 {
        self.m_raw_addr.bits_at(12..21) as u16
    }
}

impl TryFrom<usize> for HwVirtAddr {
    type Error = AddressErr;

    fn try_from(raw_addr: usize) -> Result<Self, Self::Error> {
        match raw_addr.bits_at(47..64) {
            0 | 0x1ffff => Ok(Self { m_raw_addr: raw_addr }),
            1 => Ok(Self::new(raw_addr)),
            _ => Err(AddressErr(raw_addr))
        }
    }
}
