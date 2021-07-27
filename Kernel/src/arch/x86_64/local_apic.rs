/*! Local Advanced Programmable Interrupt Controller */

use core::arch::x86_64::__cpuid;

use crate::addr::{
    virt_addr::VirtAddr,
    TAddress
};

pub struct LocalApic {
    m_virt_addr: VirtAddr,
    m_enabled: bool
}

impl LocalApic /* Constructors */ {
    pub fn new() -> Self {
        Self { m_virt_addr: VirtAddr::null(),
               m_enabled: false }
    }
}

impl LocalApic /* Methods */ {
}

impl LocalApic /* Static Functions */ {
    pub fn is_supported() -> bool {
        (unsafe { __cpuid(0x01) }.edx & (1 << 9)) != 0
    }
}
