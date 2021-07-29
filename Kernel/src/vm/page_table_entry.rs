/*! Page table entry */

use core::{
    fmt::Debug,
    mem,
    ops::{
        Deref,
        DerefMut
    }
};

use crate::{
    addr::{
        phys_addr::PhysAddr,
        virt_addr::VirtAddr
    },
    arch::vm::hw_page_table_entry::HwPageTableEntry
};

pub struct PageTableMapping<'a> {
    m_virt_addr: VirtAddr,
    m_page_table_entry: &'a mut PageTableEntry
}

impl<'a> PageTableMapping<'a> /* Constructors */ {
    pub const fn new(virt_addr: VirtAddr,
                     page_table_entry: &'a mut PageTableEntry)
                     -> Self {
        Self { m_virt_addr: virt_addr,
               m_page_table_entry: page_table_entry }
    }
}

impl<'a> PageTableMapping<'a> /* Methods */ {
    pub fn forget(self) {
        mem::forget(self);
    }
}

impl<'a> Deref for PageTableMapping<'a> {
    type Target = PageTableEntry;

    fn deref(&self) -> &Self::Target {
        self.m_page_table_entry
    }
}

impl<'a> DerefMut for PageTableMapping<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.m_page_table_entry
    }
}

impl<'a> Drop for PageTableMapping<'a> {
    fn drop(&mut self) {
        unsafe { self.m_page_table_entry.invalidate_in_tlb(self.m_virt_addr) }
    }
}

#[repr(transparent)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct PageTableEntry {
    m_hw_entry: HwPageTableEntry
}

impl PageTableEntry /* Constructors */ {
    pub fn new() -> Self {
        Self { m_hw_entry: HwPageTableEntry::new() }
    }
}

impl PageTableEntry /* Getters */ {
    #[inline]
    pub fn phys_frame(&self) -> Option<PhysAddr> {
        if self.is_present() {
            Some(self.m_hw_entry.raw_phys_frame().into())
        } else {
            None
        }
    }

    #[inline]
    pub fn is_present(&self) -> bool {
        self.m_hw_entry.is_present()
    }

    #[inline]
    pub fn is_readable(&self) -> bool {
        self.m_hw_entry.is_readable()
    }

    #[inline]
    pub fn is_writeable(&self) -> bool {
        self.m_hw_entry.is_writeable()
    }

    #[inline]
    pub fn is_cacheable(&self) -> bool {
        self.m_hw_entry.is_cacheable()
    }

    #[inline]
    pub fn is_global(&self) -> bool {
        self.m_hw_entry.is_global()
    }

    #[inline]
    pub fn is_huge_page(&self) -> bool {
        self.m_hw_entry.is_huge_page()
    }

    #[inline]
    pub fn is_accessed(&self) -> bool {
        self.m_hw_entry.is_accessed()
    }

    #[inline]
    pub fn is_dirty(&self) -> bool {
        self.m_hw_entry.is_dirty()
    }

    #[inline]
    pub fn is_no_execute(&self) -> bool {
        self.m_hw_entry.is_no_execute()
    }

    #[inline]
    pub fn is_user(&self) -> bool {
        self.m_hw_entry.is_user()
    }

    #[inline]
    pub fn is_unused(&self) -> bool {
        self.m_hw_entry.is_unused()
    }
}

impl PageTableEntry /* Setters */ {
    #[inline]
    pub fn set_phys_frame(&mut self, phys_frame: PhysAddr) -> &mut Self {
        self.m_hw_entry.set_raw_phys_frame(*phys_frame);
        self
    }

    #[inline]
    pub fn set_present(&mut self, is_present: bool) -> &mut Self {
        self.m_hw_entry.set_present(is_present);
        self
    }

    #[inline]
    pub fn set_readable(&mut self, is_readable: bool) -> &mut Self {
        self.m_hw_entry.set_readable(is_readable);
        self
    }

    #[inline]
    pub fn set_writeable(&mut self, is_writeable: bool) -> &mut Self {
        self.m_hw_entry.set_writeable(is_writeable);
        self
    }

    #[inline]
    pub fn set_cacheable(&mut self, is_cacheable: bool) -> &mut Self {
        self.m_hw_entry.set_cacheable(is_cacheable);
        self
    }

    #[inline]
    pub fn set_global(&mut self, is_global: bool) -> &mut Self {
        self.m_hw_entry.set_global(is_global);
        self
    }

    #[inline]
    pub fn set_huge_page(&mut self, is_huge_page: bool) -> &mut Self {
        self.m_hw_entry.set_huge_page(is_huge_page);
        self
    }

    #[inline]
    pub fn set_accessed(&mut self, is_accessed: bool) -> &mut Self {
        self.m_hw_entry.set_accessed(is_accessed);
        self
    }

    #[inline]
    pub fn set_dirty(&mut self, is_dirty: bool) -> &mut Self {
        self.m_hw_entry.set_dirty(is_dirty);
        self
    }

    #[inline]
    pub fn set_no_execute(&mut self, is_no_execute: bool) -> &mut Self {
        self.m_hw_entry.set_no_execute(is_no_execute);
        self
    }

    #[inline]
    pub fn set_user(&mut self, is_user: bool) -> &mut Self {
        self.m_hw_entry.set_user(is_user);
        self
    }

    #[inline]
    pub fn set_unused(&mut self) -> &mut Self {
        self.m_hw_entry.set_unused();
        self
    }
}

impl PageTableEntry /* Privates */ {
    unsafe fn invalidate_in_tlb(&self, virt_addr: VirtAddr) {
        self.m_hw_entry.invalidate_in_tlb(virt_addr);
    }
}

pub trait THwPageTableEntry: Debug + Copy + Clone {
    fn new() -> Self;

    unsafe fn invalidate_in_tlb(&self, virt_addr: VirtAddr);

    fn raw_phys_frame(&self) -> usize;
    fn is_present(&self) -> bool;
    fn is_readable(&self) -> bool;
    fn is_writeable(&self) -> bool;
    fn is_cacheable(&self) -> bool;
    fn is_global(&self) -> bool;
    fn is_huge_page(&self) -> bool;
    fn is_accessed(&self) -> bool;
    fn is_dirty(&self) -> bool;
    fn is_no_execute(&self) -> bool;
    fn is_user(&self) -> bool;
    fn is_unused(&self) -> bool;

    fn set_raw_phys_frame(&mut self, raw_phys_frame: usize);
    fn set_present(&mut self, is_present: bool);
    fn set_readable(&mut self, is_readable: bool);
    fn set_writeable(&mut self, is_writeable: bool);
    fn set_cacheable(&mut self, is_cacheable: bool);
    fn set_global(&mut self, is_global: bool);
    fn set_huge_page(&mut self, is_huge_page: bool);
    fn set_accessed(&mut self, is_accessed: bool);
    fn set_dirty(&mut self, is_dirty: bool);
    fn set_no_execute(&mut self, is_no_execute: bool);
    fn set_user(&mut self, is_user: bool);
    fn set_unused(&mut self);
}
