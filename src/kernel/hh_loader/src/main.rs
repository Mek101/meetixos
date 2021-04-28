#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(option_result_unwrap_unchecked)]

use dbg_utils::debug_size_multiplier;
use hal::{boot_infos::BootInfos, paging::PageDir};
use logger::info;

use crate::{log::init_logger, version::HHL_VERSION};

mod arch;
mod log;
mod panic;
mod version;

/* includes the module which links the kernel core binary */
include!(env!("KERNEL_BIN"));

/** # Higher half loader rust entry point
 *
 * Here is where the 64bit rust code starts his execution
 */
#[no_mangle]
pub unsafe extern "C" fn hhl_rust_entry(raw_info_ptr: *const u8) -> ! {
    /* initialize the higher half loader's instance of the BootInfos */
    let boot_info = BootInfos::from(raw_info_ptr);

    /* initialize the logger */
    init_logger();

    info!("MeetiX Kernel Loader v{}", HHL_VERSION);
    info!("Kernel size: {}", debug_size_multiplier(KERNEL_SIZE));
    info!("Kernel code: {}{}{}", KERNEL_BYTES[0], KERNEL_BYTES[1], KERNEL_BYTES[2]);

    info!("Raw info ptr: {:#x}", raw_info_ptr as usize);
    boot_info.cmdline_args().iter().for_each(|arg| info!("Arg: {}", arg.as_str()));
    boot_info.mem_areas().iter().for_each(|mem_area| {
                                    info!("{:?}, {}",
                                          mem_area,
                                          debug_size_multiplier(mem_area.size()))
                                });

    let page_dir = PageDir::active_page_dir(0);
    info!("\n{:?}", page_dir);

    loop { /* loop forever here */ }
}
