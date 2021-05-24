/*! Kernel debugging */

use shared::{
    info::descriptor::LoaderInfo,
    logger::log_debug
};

/** # Dumps `BootInfo`
 *
 * Prints using the `log` module the `BootInfo` given by the [`HAL`]
 *
 * [`BootInfo`]: /hal/info/struct.BootInfo.html
 * [`HAL`]: /hal/
 */
pub fn dump_boot_info() {
    log_debug!("BootInfo:");
    //log_debug!("\tPhys Mem Offset: {:?}",
    // BootInfo::obtain().hw_phys_mem_offset());
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
    log_debug!("\tCommand Line:");
    /*for cmd_arg in BootInfo::obtain().cmdline_args().iter() {
        log_debug!("\t\t{}", cmd_arg.as_str());
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
    log_debug!("\tMemory Areas:");
    /*for mma in BootInfo::obtain().mem_areas().iter() {
        log_debug!("\t\tBootMemArea {{ {:?}, {} }}",
               mma.start_phys_addr(),
               dbg_display_size(mma.size()));
    }*/
}
