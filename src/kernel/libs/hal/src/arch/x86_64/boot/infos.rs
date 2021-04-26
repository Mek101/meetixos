/*! # x86_64 Boot Informations
 *
 * Implements the x86_64 boot informations gainer
 */

use crate::{
    addr::{Address, PhysAddr, VirtAddr},
    boot::infos::{BootInfosInner, BootMemArea, BootMemAreas, HwBootInfosBase}
};

/** # x86_64 Boot Information Gainer
 *
 * This struct simply implements the [`HwBootInfosBase`] to construct the
 * [`BootInfosInner`]
 *
 * [`HwBootInfosBase`]: /hal/boot/trait.HwBootInfosBase.html
 * [`BootInfosInner`]: /hal/boot/struct.BootInfosInner.html
 */
pub struct X64BootInfos;

impl HwBootInfosBase for X64BootInfos {
    /** Constructs the [`BootInfosInner`] from the Multiboot2 struct
     *
     * [`BootInfosInner`]: /hal/boot/struct.BootInfosInner.html
     */
    fn obtain_inner_from_arch_infos(raw_boot_infos_ptr: *const u8) -> BootInfosInner {
        /* load the multiboot informations */
        let multiboot_hdr = unsafe { multiboot2::load(raw_boot_infos_ptr as usize) };

        /* obtain the command line string */
        let command_line = if let Some(cmdline_tag) = multiboot_hdr.command_line_tag() {
            cmdline_tag.command_line()
        } else {
            "-log-level=Debug"
        };

        /* obtain the memory areas */
        let mem_areas = if let Some(mboot_mem_areas) = multiboot_hdr.memory_map_tag() {
            let mut mem_areas = BootMemAreas::new();

            /* collect all the valid memory areas given by the bootloader */
            for mmap in mboot_mem_areas.memory_areas() {
                let mem_area = BootMemArea::new(unsafe {
                                                    PhysAddr::new_unchecked(mmap.start_address() as usize)
                                                },
                                                mmap.size() as usize);
                mem_areas.push(mem_area);
            }

            mem_areas
        } else {
            panic!("Multiboot2 header doesn't provide a valid memory map");
        };

        /* construct the instance to return */
        BootInfosInner::new(VirtAddr::new_zero(), command_line, mem_areas)
    }
}
