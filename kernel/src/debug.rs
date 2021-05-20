/*! Kernel debugging */

use shared::{
    info::descriptor::LoaderInfo,
    logger::debug
};

/** # Dumps `BootInfo`
 *
 * Prints using the `log` module the `BootInfo` given by the [`HAL`]
 *
 * [`BootInfo`]: /hal/info/struct.BootInfo.html
 * [`HAL`]: /hal/
 */
pub fn dump_boot_info() {
    debug!("BootInfo:");
    //debug!("\tPhys Mem Offset: {:?}", BootInfo::obtain().hw_phys_mem_offset());
    dump_boot_cmdline();
    dump_boot_mem_areas();
}

/** # Dumps boot command line
 *
 * Prints using the `log` module the command line inside the [`BootInfo`]
 * given by the [`HAL`]
 *
 * [`BootInfo`]: /hal/info/struct.BootInfo.html
 * [`HAL`]: /hal/
 */
pub fn dump_boot_cmdline() {
    debug!("\tCommand Line:");
    /*for cmd_arg in BootInfo::obtain().cmdline_args().iter() {
        debug!("\t\t{}", cmd_arg.as_str());
    }*/
}

/** # Dumps boot memory areas
 *
 * Prints using the `log` module the memory areas inside the [`BootInfo`]
 * given by the [`HAL`]
 *
 * [`BootInfo`]: /hal/info/struct.BootInfo.html
 * [`HAL`]: /hal/
 */
pub fn dump_boot_mem_areas() {
    debug!("\tMemory Areas:");
    /*for mma in BootInfo::obtain().mem_areas().iter() {
        debug!("\t\tBootMemArea {{ {:?}, {} }}",
               mma.start_phys_addr(),
               dbg_display_size(mma.size()));
    }*/
}
