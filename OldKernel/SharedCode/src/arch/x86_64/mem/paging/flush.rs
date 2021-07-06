/*! x86_64 map flusher implementation */

use crate::{
    addr::{
        virt::VirtAddr,
        Address
    },
    mem::paging::flush::HwMapFlusherBase
};

/**
 * x86_64 `HwMapFlusherBase` implementation
 */
pub struct HwMapFlusher;

impl HwMapFlusherBase for HwMapFlusher {
    fn new() -> Self {
        Self
    }

    unsafe fn flush_addr(&self, addr: VirtAddr) {
        asm!("invlpg [{}]", in(reg) addr.as_usize(), options(nostack, preserves_flags));
    }

    unsafe fn flush_all(&self) {
        let cr3_value: usize;
        asm!("mov {}, cr3", out(reg) cr3_value, options(nomem, nostack, preserves_flags));
        asm!("mov cr3, {}", in(reg) cr3_value, options(nomem, preserves_flags));
    }
}
