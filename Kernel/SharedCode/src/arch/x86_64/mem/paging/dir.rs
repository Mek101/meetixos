/*! x86_64 page directory support */

use crate::{
    addr::{
        phys::PhysAddr,
        Address
    },
    mem::paging::{
        dir::HwPageDirSupportBase,
        frame::PhysFrame,
        table::PageTableLevel,
        Page4KiB
    }
};

/**
 * x86_64 `HwPageDirSupportBase` implementation
 */
pub struct HwPageDirSupport;

impl HwPageDirSupportBase for HwPageDirSupport {
    /* Page Table Entry flags */
    const PTE_PRESENT: usize = 0;
    const PTE_READABLE: usize = 9;
    const PTE_WRITEABLE: usize = 1;
    const PTE_GLOBAL: usize = 8;
    const PTE_HUGE: usize = 7;
    const PTE_ACCESSED: usize = 5;
    const PTE_DIRTY: usize = 6;
    const PTE_NO_EXECUTE: usize = 63;
    const PTE_USER: usize = 2;

    /* Page Table Entry mask */
    const PTE_ADDR_MASK: usize = 0x000F_FFFF_FFFF_F000;

    /* Page Table entries count */
    const PT_ENTRIES_COUNT: usize = 512;

    /* Page Table levels */
    const PT_LEVEL_PGDIR: PageTableLevel = PageTableLevel::Level4;
    const PT_LEVEL_1GB: PageTableLevel = PageTableLevel::Level3;
    const PT_LEVEL_2MB: PageTableLevel = PageTableLevel::Level2;
    const PT_LEVEL_4KB: PageTableLevel = PageTableLevel::Level1;

    unsafe fn active_page_dir_frame() -> PhysFrame<Page4KiB> {
        let cr3_value: usize;
        asm!("mov {}, cr3", out(reg) cr3_value, options(nomem, nostack, preserves_flags));

        PhysAddr::new(cr3_value & Self::PTE_ADDR_MASK).containing_frame()
    }

    unsafe fn activate_page_dir(phys_frame: PhysFrame<Page4KiB>) {
        let raw_addr = phys_frame.start_addr().as_usize();
        asm!("mov cr3, {}", in(reg) raw_addr, options(nomem, preserves_flags));
    }
}
