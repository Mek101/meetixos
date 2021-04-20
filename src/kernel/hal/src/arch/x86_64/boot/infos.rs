/*! # x86_64 Boot Informations
 *
 * Implements the x86_64 boot informations gainer
 */

use core::convert::TryFrom;

use bootloader::{bootinfo::MemoryRegionType, BootInfo};

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
    /** Constructs the [`BootInfosInner`] from the Philipp Oppermann
     * bootloader's infos
     *
     * [`BootInfosInner`]: /hal/boot/struct.BootInfosInner.html
     */
    fn obtain_inner_from_arch_infos(raw_boot_infos_ptr: *const u8) -> BootInfosInner {
        /* obtain a copy of the BootInfo struct of the Philipp Oppermann's
         * bootloader. It would be a good idea to support too the GRUB bootloader
         */
        let bootloader_infos = unsafe {
            let boot_info_ptr = raw_boot_infos_ptr as *const BootInfo;
            boot_info_ptr.read()
        };

        /* prepare the various data to put into the BootInfosInner instance */
        let command_line = "-loglvl=Debug"; /* hardcode because no command line informations yet */
        let mut mem_areas = BootMemAreas::new();
        let hw_page_dir =
            VirtAddr::try_from(bootloader_infos.physical_memory_offset as usize).unwrap();

        /* iterate through the memory maps of the bootloader and select the regions
         * marked as usable or used to map the bootloader itself (it is not needed
         * anymore) or the boot infos
         */
        for mem_area in
            bootloader_infos.memory_map.iter().filter(|mem_region| {
                                                  match mem_region.region_type {
                                                      MemoryRegionType::Usable
                                                      | MemoryRegionType::Bootloader
                                                      | MemoryRegionType::BootInfo => {
                                                          true
                                                      },
                                                      _ => false
                                                  }
                                              })
        {
            let start_addr =
                PhysAddr::try_from(mem_area.range.start_addr() as usize).unwrap();
            let size = mem_area.range.end_addr() - start_addr.as_usize() as u64;

            /* put the next memory area */
            mem_areas.push(BootMemArea::new(start_addr, size as usize));
        }

        /* construct the instance to return */
        BootInfosInner::new(hw_page_dir, command_line, mem_areas)
    }
}
