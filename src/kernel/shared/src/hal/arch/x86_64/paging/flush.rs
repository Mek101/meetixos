/*! # x86_64 Map Flusher
 *
 * Implements the x86_64 hardware implementation of the mapping flusher
 * structure
 */

use crate::hal::{
    addr::{Address, VirtAddr},
    paging::HwMapFlusherBase
};

/** # x86_64 Mapper Flusher
 *
 * Implements the [`HwMapFlusherBase`] for the x86_64 architecture
 *
 * [`HwMapFlusherBase`]: /hal/paging/trait.HwMapFlusherBase.html
 */
pub struct X64MapFlusher;

impl HwMapFlusherBase for X64MapFlusher {
    /** Constructs an `X64MapFlusher` instance
     */
    fn new() -> Self {
        Self
    }

    /** Flushes the given virtual address invalidating the TLB entry
     */
    unsafe fn flush_addr(&self, addr: VirtAddr) {
        use x86_64::{instructions, VirtAddr as X64VirtAddr};

        instructions::tlb::flush(X64VirtAddr::new_unsafe(addr.as_usize() as u64));
    }

    /** Flushes all the TLB
     */
    unsafe fn flush_all(&self) {
        x86_64::instructions::tlb::flush_all();
    }
}
