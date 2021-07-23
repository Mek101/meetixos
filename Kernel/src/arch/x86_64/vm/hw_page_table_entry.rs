/*! x86_64 hardware page table entry */

use core::{
    fmt,
    fmt::Debug
};

use bits::bit_fields::TBitFields;

use crate::{
    addr::phys_addr::PhysAddr,
    vm::page_table_entry::THwPageTableEntry
};

/**
 * x86_64 `HwPageTableEntryBase` implementation
 */
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct HwPageTableEntry {
    m_entry_value: usize
}

impl THwPageTableEntry for HwPageTableEntry {
    #[inline]
    fn new() -> Self {
        Self { m_entry_value: 0 }
    }

    #[inline]
    unsafe fn invalidate_in_tlb(&self) {
        asm!("invlpg [{}]", in(reg) self.raw_phys_frame())
    }

    #[inline]
    fn raw_phys_frame(&self) -> usize {
        self.m_entry_value & 0x000f_ffff_ffff_f000
    }

    #[inline]
    fn is_present(&self) -> bool {
        self.m_entry_value.bit_at(0)
    }

    #[inline]
    fn is_readable(&self) -> bool {
        true /* TODO */
    }

    #[inline]
    fn is_writeable(&self) -> bool {
        self.m_entry_value.bit_at(1)
    }

    #[inline]
    fn is_cacheable(&self) -> bool {
        !self.m_entry_value.bit_at(4)
    }

    #[inline]
    fn is_global(&self) -> bool {
        self.m_entry_value.bit_at(8)
    }

    #[inline]
    fn is_huge_page(&self) -> bool {
        self.m_entry_value.bit_at(7)
    }

    #[inline]
    fn is_accessed(&self) -> bool {
        self.m_entry_value.bit_at(5)
    }

    #[inline]
    fn is_dirty(&self) -> bool {
        self.m_entry_value.bit_at(6)
    }

    #[inline]
    fn is_no_execute(&self) -> bool {
        self.m_entry_value.bit_at(63)
    }

    #[inline]
    fn is_user(&self) -> bool {
        self.m_entry_value.bit_at(2)
    }

    #[inline]
    fn is_unused(&self) -> bool {
        self.m_entry_value == 0
    }

    #[inline]
    fn set_raw_phys_frame(&mut self, raw_phys_frame: usize) {
        self.m_entry_value =
            raw_phys_frame | (self.m_entry_value & 0xfff0_0000_0000_0fff);
    }

    #[inline]
    fn set_present(&mut self, is_present: bool) {
        self.m_entry_value.set_bit(0, is_present);
    }

    #[inline]
    fn set_readable(&mut self, _is_readable: bool) {
        /* Unsupported bit in this architecture */
    }

    #[inline]
    fn set_writeable(&mut self, is_writeable: bool) {
        self.m_entry_value.set_bit(1, is_writeable);
    }

    #[inline]
    fn set_cacheable(&mut self, is_cacheable: bool) {
        self.m_entry_value.set_bit(4, !is_cacheable);
    }

    #[inline]
    fn set_global(&mut self, is_global: bool) {
        self.m_entry_value.set_bit(8, is_global);
    }

    #[inline]
    fn set_huge_page(&mut self, is_huge_page: bool) {
        self.m_entry_value.set_bit(7, is_huge_page);
    }

    #[inline]
    fn set_accessed(&mut self, is_accessed: bool) {
        self.m_entry_value.set_bit(5, is_accessed);
    }

    #[inline]
    fn set_dirty(&mut self, is_dirty: bool) {
        self.m_entry_value.set_bit(6, is_dirty);
    }

    #[inline]
    fn set_no_execute(&mut self, is_no_execute: bool) {
        self.m_entry_value.set_bit(63, is_no_execute);
    }

    #[inline]
    fn set_user(&mut self, is_user: bool) {
        self.m_entry_value.set_bit(2, is_user);
    }

    #[inline]
    fn set_unused(&mut self) {
        self.m_entry_value = 0;
    }
}

impl Debug for HwPageTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "HwPageTableEntry {{ m_phys_addr: {}, m_flags: ",
               PhysAddr::from(self.raw_phys_frame()))?;

        /* write the name of all the active bits */
        let all_printable_flags = [(self.is_present(), "is_present"),
                                   (self.is_readable(), "is_readable"),
                                   (self.is_writeable(), "is_writeable"),
                                   (self.is_cacheable(), "is_cacheable"),
                                   (self.is_global(), "is_global"),
                                   (self.is_huge_page(), "is_huge_page"),
                                   (self.is_accessed(), "is_accessed"),
                                   (self.is_dirty(), "is_dirty"),
                                   (self.is_no_execute(), "is_no_execute"),
                                   (self.is_user(), "is_user")];
        let mut is_first = true;
        for (bit_value, str_name) in all_printable_flags {
            if bit_value {
                if !is_first {
                    write!(f, " | ")?;
                } else {
                    is_first = false;
                }

                write!(f, "{}", str_name)?;
            }
        }

        /* write a 0 value if any bit is active */
        if is_first {
            write!(f, "0 }}")?;
        } else {
            write!(f, " }}")?;
        }
        Ok(())
    }
}
