/*! x86_64 page directory */

use crate::{
    addr::phys_addr::PhysAddr,
    vm::page_dir::HwPageDirBase
};

#[derive(Debug)]
pub struct HwPageDir {
    m_phys_frame: PhysAddr
}

impl HwPageDirBase for HwPageDir {
    fn from_phys_frame(phys_frame: PhysAddr) -> Self {
        Self { m_phys_frame: phys_frame }
    }

    fn current() -> Self {
        let cr3_value: usize;
        unsafe {
            asm!("mov {}, cr3", out(reg) cr3_value, options(nomem, nostack, preserves_flags));
        }

        Self::from_phys_frame((cr3_value & 0x000f_ffff_ffff_f000).into())
    }

    unsafe fn activate(&self) {
        asm!("mov cr3, {}", in(reg) *self.m_phys_frame, options(nomem, preserves_flags));
    }

    #[inline]
    fn root_phys_frame(&self) -> PhysAddr {
        self.m_phys_frame
    }
}
