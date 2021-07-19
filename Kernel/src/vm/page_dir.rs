/*! Page directory */

use core::fmt::Debug;

use crate::{
    addr::{
        phys_addr::PhysAddr,
        virt_addr::VirtAddr
    },
    arch::vm::hw_page_dir::HwPageDir,
    vm::{
        mem_manager::MemManager,
        page_table::PageTable,
        page_table_entry::PageTableEntry
    }
};

pub struct PageDir {
    m_hw_page_dir: HwPageDir
}

impl PageDir /* Constructors */ {
    pub fn from_phys_frame(phys_frame: PhysAddr) -> Self {
        Self { m_hw_page_dir: HwPageDir::from_phys_frame(phys_frame) }
    }

    pub fn active() -> Self {
        Self { m_hw_page_dir: HwPageDir::active() }
    }
}

impl PageDir /* Methods */ {
    pub unsafe fn activate(&self) {
        self.m_hw_page_dir.activate();
    }

    pub unsafe fn next_page_table_from_entry(&self,
                                             page_table_entry: &PageTableEntry)
                                             -> &'static mut PageTable {
        assert!(page_table_entry.is_present());
        self.next_page_table(page_table_entry.phys_frame().unwrap())
    }
}

impl PageDir /* Getters */ {
    pub fn phys_frame(&self) -> PhysAddr {
        self.m_hw_page_dir.phys_frame()
    }

    pub fn root_page_table(&self) -> &mut PageTable {
        unsafe { self.next_page_table(self.phys_frame()) }
    }
}

impl PageDir /* Privates */ {
    unsafe fn next_page_table(&self, phys_frame: PhysAddr) -> &'static mut PageTable {
        let phys_mem_mapping_range_start =
            MemManager::instance().layout_manager().phys_mem_mapping_range().start;

        let page_table_virt_addr: VirtAddr =
            (*phys_frame + *phys_mem_mapping_range_start).into();

        page_table_virt_addr.as_ref_mut()
    }
}

pub trait HwPageDirBase: Debug {
    fn from_phys_frame(phys_frame: PhysAddr) -> Self;

    fn active() -> Self;

    unsafe fn activate(&self);

    fn phys_frame(&self) -> PhysAddr;
}
