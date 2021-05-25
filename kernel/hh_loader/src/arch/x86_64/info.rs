/*! x86_64 boot information implementation */

use shared::{
    addr::{
        phys::PhysAddr,
        virt::VirtAddr,
        Address
    },
    mem::paging::frame::VirtFrame
};

use crate::info::{
    info::{
        BootInfo,
        HwBootInfoBase
    },
    mem_area::{
        BootMemArea,
        BootMemAreas
    }
};

/* NOTE: keep the following value aligned with the code in loader_start.S */
const LOADER_MAPPED_RANGE_LAST: usize = 0x01E00000;

extern "C" {
    static __hhl_text_begin: usize;
    static __hhl_text_end: usize;
}

/**
 * x86_64 `HwBootInfoBase` implementation.
 *
 * Interprets the given `raw_boot_info` as `Multiboot2` pointer
 */
pub struct HwBootInfo;

impl HwBootInfoBase for HwBootInfo {
    fn obtain_from_arch_info(raw_boot_info_ptr: *const u8) -> BootInfo {
        /* load the multiboot information */
        let mboot_hdr = unsafe { multiboot2::load(raw_boot_info_ptr as usize) };

        /* obtain the bootloader name */
        let bootloader_name = if let Some(name_tag) = mboot_hdr.boot_loader_name_tag() {
            name_tag.name()
        } else {
            "Multiboot2 compliant bootloader"
        };

        /* obtain the command line string */
        let raw_cmdline = if let Some(cmdline_tag) = mboot_hdr.command_line_tag() {
            cmdline_tag.command_line()
        } else {
            "-log-level=Debug"
        };

        /* obtain the memory areas */
        let mem_areas = if let Some(mboot_mem_areas) = mboot_hdr.memory_map_tag() {
            let mut mem_areas = BootMemAreas::new();

            /* collect all the valid memory areas given by the bootloader */
            for mmap in mboot_mem_areas.memory_areas() {
                let mem_area = {
                    let phys_start_addr = PhysAddr::new(mmap.start_address() as usize);
                    let area_size = mmap.size() as usize;

                    BootMemArea::new(phys_start_addr, area_size)
                };
                mem_areas.insert(mem_area);
            }

            mem_areas
        } else {
            panic!("Multiboot2 header doesn't provide a valid memory map");
        };

        /* hh_loader resides with his text and data into this range */
        let loader_reserved_range = {
            let start_frame = {
                let raw_ptr = unsafe { &__hhl_text_begin as *const _ };

                VirtAddr::from(raw_ptr).containing_frame()
            };
            let end_frame = {
                let raw_ptr = unsafe { &__hhl_text_end as *const _ };

                VirtAddr::from(raw_ptr).containing_frame()
            };

            VirtFrame::range_incl_of(start_frame, end_frame)
        };

        /* hh_loader identity maps the first 32MiB of RAM */
        let loader_mapped_range = {
            let start_frame = loader_reserved_range.end().clone() + 1;
            let last_frame = VirtAddr::new(LOADER_MAPPED_RANGE_LAST).containing_frame();

            VirtFrame::range_incl_of(start_frame, last_frame)
        };

        /* construct the instance to return */
        BootInfo::new(mem_areas,
                      raw_cmdline,
                      loader_reserved_range,
                      loader_mapped_range,
                      bootloader_name)
    }
}
