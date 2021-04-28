/*! # x86_64 Page Directory
 *
 * Implements the page table flags for the x86_64 architecture
 */

use crate::{
    addr::{Address, PhysAddr},
    mem::paging::{HwPageDirSupportBase, Page4KiB, PageTableLevel, PhysFrame}
};

/** # x86_64 Page Dir Mapping Flags
 *
 * Implements the [`HwPdMapFlagsBase`] for the x86_64 architecture
 *
 * [`HwPdMapFlagsBase`]: /hal/paging/trait.HwPdMapFlagsBase.html
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

    /** Returns the current [`PageDir`]'s [`PhysFrame`]
     *
     * [`PageDir`]: /hal/paging/struct.PageDir.html
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     */
    unsafe fn active_page_dir_frame() -> PhysFrame<Page4KiB> {
        use x86_64::registers::control::Cr3;

        /* read the current CR3 value */
        let (phys_frame, ..) = Cr3::read();

        /* return back the PhysFrame */
        let phys_addr =
            PhysAddr::new_unchecked(phys_frame.start_address().as_u64() as usize);
        PhysFrame::of_addr(phys_addr)
    }

    /** Activates the given [`PhysFrame`] as current [`PageDir`]
     *
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     * [`PageDir`]: /hal/paging/struct.PageDir.html
     */
    unsafe fn activate_page_dir(phys_frame: PhysFrame<Page4KiB>) {
        use x86_64::{
            registers::control::Cr3, structures::paging::PhysFrame as X64PhysFrame,
            PhysAddr as X64PhysAddr
        };

        /* read the previous CR3 content to obtain the active flags */
        let (_, cr3_flags) = Cr3::read();

        /* construct the x86_64 PhysAddr */
        let x64_phys_addr =
            X64PhysAddr::new_unsafe(phys_frame.start_addr().as_usize() as u64);

        /* construct the x86_64 PhysFrame */
        let x64_phys_frame = X64PhysFrame::from_start_address_unchecked(x64_phys_addr);

        /* update the CR3 content */
        Cr3::write(x64_phys_frame, cr3_flags);
    }
}
