/*! # Kernel Debugging Utils
 *
 * Implements dump & debug utility functions
 */

use dbg_utils::dbg_display_size;
use hal::boot_infos::BootInfos;
use logger::debug;

/** # Dumps `BootInfos`
 *
 * Prints using the `log` module the [`BootInfos`] given by the [`HAL`]
 *
 * [`BootInfos`]: /hal/infos/struct.BootInfos.html
 * [`HAL`]: /hal/
 */
pub fn dump_boot_infos() {
    debug!("BootInfos:");
    //debug!("\tPhys Mem Offset: {:?}", BootInfos::obtain().hw_phys_mem_offset());
    dump_boot_cmdline();
    dump_boot_mem_areas();
}

/** # Dumps boot command line
 *
 * Prints using the `log` module the command line inside the [`BootInfos`]
 * given by the [`HAL`]
 *
 * [`BootInfos`]: /hal/infos/struct.BootInfos.html
 * [`HAL`]: /hal/
 */
pub fn dump_boot_cmdline() {
    debug!("\tCommand Line:");
    for cmd_arg in BootInfos::obtain().cmdline_args().iter() {
        debug!("\t\t{}", cmd_arg.as_str());
    }
}

/** # Dumps boot memory areas
 *
 * Prints using the `log` module the memory areas inside the [`BootInfos`]
 * given by the [`HAL`]
 *
 * [`BootInfos`]: /hal/infos/struct.BootInfos.html
 * [`HAL`]: /hal/
 */
pub fn dump_boot_mem_areas() {
    debug!("\tMemory Areas:");
    /*for mma in BootInfos::obtain().mem_areas().iter() {
        debug!("\t\tBootMemArea {{ {:?}, {} }}",
               mma.start_phys_addr(),
               dbg_display_size(mma.size()));
    }*/
}
