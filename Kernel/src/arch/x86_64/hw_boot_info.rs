/*! x86_64 boot boot implementation */

use multiboot2::{
    load,
    BootInformation,
    BootLoaderNameTag,
    CommandLineTag,
    MemoryMapTag
};

use crate::{
    addr::phys_addr::PhysAddr,
    boot_info::{
        BootMemArea,
        BootMemAreas,
        HwBootInfoBase
    }
};

/**
 * `HwBootInfoBase` for x86_64 `Multiboot_2` standard
 */
pub struct HwBootInfo {
    m_multiboot_ptr: BootInformation
}

impl HwBootInfoBase for HwBootInfo {
    fn boot_loader_name(&self) -> &str {
        self.m_multiboot_ptr
            .boot_loader_name_tag()
            .map(BootLoaderNameTag::name)
            .unwrap_or("Multiboot based bootloader")
    }

    fn cmd_line_args(&self) -> &str {
        self.m_multiboot_ptr
            .command_line_tag()
            .map(CommandLineTag::command_line)
            .unwrap_or_default()
    }

    fn mem_areas(&self) -> BootMemAreas {
        self.m_multiboot_ptr
            .memory_map_tag()
            .map(MemoryMapTag::all_memory_areas)
            .map(|mem_areas| {
                let mut boot_mem_areas = BootMemAreas::default();

                /* put all the multiboot areas into the collector */
                for mem_area in mem_areas {
                    let boot_mem_area = {
                        let raw_phys_addr = mem_area.start_address() as usize;
                        let area_size = mem_area.size() as usize;

                        /* construct the <BootMemArea> */
                        BootMemArea::new(PhysAddr::from(raw_phys_addr), area_size)
                    };

                    /* unordered push, we rely on the right order by the bootloader */
                    boot_mem_areas.push_area(boot_mem_area);
                }
                boot_mem_areas
            })
            .expect("Bootloader doesn't provide memory areas")
    }
}

impl From<*const u8> for HwBootInfo {
    fn from(boot_info_ptr: *const u8) -> Self {
        Self { m_multiboot_ptr: unsafe { load(boot_info_ptr as usize) } }
    }
}
