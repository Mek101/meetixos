/*! x86_64 boot information implementation */

use crate::{
    addr::{
        phys::PhysAddr,
        Address
    },
    info::{
        info::{
            BootInfoInner,
            HwBootInfoBase
        },
        mem_area::{
            BootMemArea,
            BootMemAreas
        }
    }
};

/**
 * x86_64 `HwBootInfoBase` implementation.
 *
 * Interprets the given `raw_boot_info` as `Multiboot2` pointer
 */
pub struct HwBootInfo;

impl HwBootInfoBase for HwBootInfo {
    fn obtain_inner_from_arch_info(raw_boot_info_ptr: *const u8) -> BootInfoInner {
        /* load the multiboot information */
        let multiboot_hdr = unsafe { multiboot2::load(raw_boot_info_ptr as usize) };

        /* obtain the bootloader name */
        let name = if let Some(name_tag) = multiboot_hdr.boot_loader_name_tag() {
            name_tag.name()
        } else {
            "Multiboot2 based bootloader"
        };

        /* obtain the command line string */
        let raw_cmdline = if let Some(cmdline_tag) = multiboot_hdr.command_line_tag() {
            cmdline_tag.command_line()
        } else {
            "-log-level=Debug"
        };

        /* obtain the memory areas */
        let mem_areas = if let Some(mboot_mem_areas) = multiboot_hdr.memory_map_tag() {
            let mut mem_areas = BootMemAreas::new();

            /* collect all the valid memory areas given by the bootloader */
            for mmap in mboot_mem_areas.memory_areas() {
                let mem_area = BootMemArea::new(PhysAddr::new(mmap.start_address()
                                                              as usize),
                                                mmap.size() as usize);
                mem_areas.push(mem_area);
            }

            mem_areas
        } else {
            panic!("Multiboot2 header doesn't provide a valid memory map");
        };

        /* construct the instance to return */
        BootInfoInner::new(raw_cmdline, mem_areas, name)
    }
}
