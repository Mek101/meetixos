/*! x86_64 hardware page table entry */

use core::{
    fmt,
    fmt::Debug
};

use crate::vm::paging::page_table_entry::HwPageTableEntryBase;

#[repr(transparent)]
pub struct HwPageTableEntry {
    m_entry_value: usize
}

impl HwPageTableEntryBase for HwPageTableEntry {
    #[inline]
    fn new() -> Self {
        Self { m_entry_value: 0 }
    }

    #[inline]
    unsafe fn invalidate_in_tlb(&self) {
        todo!()
    }

    #[inline]
    fn raw_phys_frame(&self) -> usize {
        todo!()
    }

    #[inline]
    fn is_present(&self) -> bool {
        todo!()
    }

    #[inline]
    fn is_readable(&self) -> bool {
        todo!()
    }

    #[inline]
    fn is_writeable(&self) -> bool {
        todo!()
    }

    #[inline]
    fn is_global(&self) -> bool {
        todo!()
    }

    #[inline]
    fn is_huge_page(&self) -> bool {
        todo!()
    }

    #[inline]
    fn is_accessed(&self) -> bool {
        todo!()
    }

    #[inline]
    fn is_dirty(&self) -> bool {
        todo!()
    }

    #[inline]
    fn is_no_execute(&self) -> bool {
        todo!()
    }

    #[inline]
    fn is_user(&self) -> bool {
        todo!()
    }

    #[inline]
    fn is_unused(&self) -> bool {
        todo!()
    }

    #[inline]
    fn set_raw_phys_frame(&mut self, _raw_phys_frame: usize) {
        todo!()
    }

    #[inline]
    fn set_present(&mut self, _is_present: bool) {
        todo!()
    }

    #[inline]
    fn set_readable(&mut self, _is_readable: bool) {
        todo!()
    }

    #[inline]
    fn set_writeable(&mut self, _is_writeable: bool) {
        todo!()
    }

    #[inline]
    fn set_global(&mut self, _is_global: bool) {
        todo!()
    }

    #[inline]
    fn set_huge_page(&mut self, _is_huge_page: bool) {
        todo!()
    }

    #[inline]
    fn set_accessed(&mut self, _is_accessed: bool) {
        todo!()
    }

    #[inline]
    fn set_dirty(&mut self, _is_dirty: bool) {
        todo!()
    }

    #[inline]
    fn set_no_execute(&mut self, _is_no_execute: bool) {
        todo!()
    }

    #[inline]
    fn set_user(&mut self, _is_user: bool) {
        todo!()
    }

    #[inline]
    fn set_unused(&mut self) {
        todo!()
    }
}

impl Debug for HwPageTableEntry {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
