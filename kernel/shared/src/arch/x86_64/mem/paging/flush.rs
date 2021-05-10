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
pub struct X64MapFlusher;

impl HwMapFlusherBase for X64MapFlusher {
    fn new() -> Self {
        Self
    }

    unsafe fn flush_addr(&self, addr: VirtAddr) {
        use x86_64::{
            instructions,
            VirtAddr as X64VirtAddr
        };

        instructions::tlb::flush(X64VirtAddr::new_unsafe(addr.as_usize() as u64));
    }

    unsafe fn flush_all(&self) {
        x86_64::instructions::tlb::flush_all();
    }
}
