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
pub struct X64PageDirSupport;

impl HwPageDirSupportBase for X64PageDirSupport {
    /* Page Table Entry flags */
    const PTE_PRESENT: usize = 1 << 0;
    const PTE_READABLE: usize = 0;
    const PTE_WRITEABLE: usize = 1 << 1;
    const PTE_GLOBAL: usize = 1 << 8;
    const PTE_HUGE: usize = 1 << 7;
    const PTE_ACCESSED: usize = 1 << 5;
    const PTE_DIRTY: usize = 1 << 6;
    const PTE_NO_EXECUTE: usize = 1 << 63;
    const PTE_USER: usize = 1 << 2;

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
        use x86_64::registers::control::Cr3;

        PhysAddr::new(Cr3::read().0.start_address().as_u64() as usize).containing_frame()
    }

    unsafe fn activate_page_dir(phys_frame: PhysFrame<Page4KiB>) {
        use x86_64::{
            registers::control::Cr3,
            structures::paging::PhysFrame as X64PhysFrame,
            PhysAddr as X64PhysAddr
        };

        /* construct the x86_64 PhysAddr */
        let x64_phys_addr =
            X64PhysAddr::new_unsafe(phys_frame.start_addr().as_usize() as u64);

        /* update the CR3 content */
        Cr3::write(X64PhysFrame::from_start_address_unchecked(x64_phys_addr),
                   Cr3::read().1);
    }
}
