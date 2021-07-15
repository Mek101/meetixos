/*! x86_64 boot boot implementation */

use multiboot2::{
    load,
    BootInformation,
    BootLoaderNameTag,
    CommandLineTag,
    MemoryMapTag
};

use helps::dbg::C_MIB;

use crate::{
    addr::{
        phys_addr::PhysAddr,
        Address
    },
    boot_info::{
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
            .map(MemoryMapTag::memory_areas)
            .map(|mem_areas| {
                let mut boot_mem_areas = BootMemAreas::default();

                /* put all the multiboot areas into the collector */
                for mem_area in mem_areas {
                    let start_phys_addr: PhysAddr =
                        (mem_area.start_address() as usize).into();
                    let phys_area_size = mem_area.size() as usize;

                    /* don't map memory below one MiB.
                     * Since it contains the BIOS and because it can be mapped by the
                     * video-drivers
                     */
                    if start_phys_addr < C_MIB.into()
                       && start_phys_addr.offset(phys_area_size) < C_MIB.into()
                    {
                        continue;
                    } else {
                        /* unordered push, we rely on the right order by the bootloader */
                        boot_mem_areas.push_area(start_phys_addr.to_range(phys_area_size));
                    }
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
