/*! Page directory */

use core::fmt::Debug;

use crate::{
    addr::phys_addr::PhysAddr,
    arch::vm::hw_page_dir::HwPageDir
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
}

impl PageDir /* Getters */ {
    pub fn phys_frame(&self) -> PhysAddr {
        self.m_hw_page_dir.phys_frame()
    }
}

pub trait HwPageDirBase: Debug {
    fn from_phys_frame(phys_frame: PhysAddr) -> Self;

    fn active() -> Self;

    unsafe fn activate(&self);

    fn phys_frame(&self) -> PhysAddr;
}
