/*! Page directory */

use core::fmt::Debug;

use crate::{
    addr::{
        phys_addr::PhysAddr,
        virt_addr::VirtAddr,
        Address
    },
    arch::vm::hw_page_dir::HwPageDir,
    vm::{
        mem_manager::MemManager,
        page_table::PageTable,
        page_table_entry::PageTableEntry
    }
};

pub struct PageDir {
    m_hw_page_dir: HwPageDir,
    m_phys_mem_offset: VirtAddr
}

impl PageDir /* Constructors */ {
    pub fn from_phys_frame(phys_frame: PhysAddr) -> Self {
        Self { m_hw_page_dir: HwPageDir::from_phys_frame(phys_frame),
               m_phys_mem_offset: MemManager::instance().layout_manager()
                                                        .phys_mem_mapping_range()
                                                        .start }
    }

    pub fn current() -> Self {
        Self { m_hw_page_dir: HwPageDir::current(),
               m_phys_mem_offset: MemManager::instance().layout_manager()
                                                        .phys_mem_mapping_range()
                                                        .start }
    }

    pub fn pre_phys_mapping() -> Self {
        Self { m_hw_page_dir: HwPageDir::current(),
               m_phys_mem_offset: VirtAddr::null() }
    }
}

impl PageDir /* Methods */ {
    pub unsafe fn activate(&self) {
        self.m_hw_page_dir.activate();
    }

    pub unsafe fn next_page_table(&self,
                                  page_table_entry: &PageTableEntry)
                                  -> &mut PageTable {
        assert!(page_table_entry.is_present(),
                "Tried to obtain a non-present `&mut PageTable`");

        self.frame_to_next_page_table(page_table_entry.phys_frame().unwrap())
    }
}

impl PageDir /* Getters */ {
    pub fn root_phys_frame(&self) -> PhysAddr {
        self.m_hw_page_dir.root_phys_frame()
    }

    pub fn root_page_table(&self) -> &mut PageTable {
        unsafe { self.frame_to_next_page_table(self.root_phys_frame()) }
    }
}

impl PageDir /* Privates */ {
    unsafe fn frame_to_next_page_table(&self, phys_frame: PhysAddr) -> &mut PageTable {
        let page_table_virt_addr: VirtAddr =
            (*phys_frame + *self.m_phys_mem_offset).into();

        page_table_virt_addr.as_ref_mut()
    }
}

pub trait HwPageDirBase: Debug {
    fn from_phys_frame(phys_frame: PhysAddr) -> Self;

    fn current() -> Self;

    unsafe fn activate(&self);

    fn root_phys_frame(&self) -> PhysAddr;
}
