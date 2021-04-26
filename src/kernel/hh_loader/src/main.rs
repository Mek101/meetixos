#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(option_result_unwrap_unchecked)]

use hal::{boot::infos::BootInfos, paging::PageDir};

use crate::log::{info, init_logger};

mod arch;
mod log;
mod panic;

/* includes the module which links the kernel core binary */
include!(env!("KERNEL_BIN"));

#[no_mangle]
pub unsafe extern "C" fn hhl_rust_entry(raw_info_ptr: *const u8) -> ! {
    /* initialize the higher half loader's instance of the BootInfos */
    let boot_info = BootInfos::from(raw_info_ptr);

    init_logger().unwrap();

    info!("Raw info ptr: {:x}", raw_info_ptr as usize);
    boot_info.cmdline_args().iter().for_each(|arg| info!("Arg: {}", arg.as_str()));
    boot_info.mem_areas().iter().for_each(|mem_area| info!("{:?}", mem_area));

    info!("MeetiX Kernel Loader v0.1.0");
    info!("Kernel size: {}", KERNEL_SIZE);
    info!("Kernel bytes[0]: {}", KERNEL_BYTES[0]);

    let page_dir = PageDir::active_page_dir(0);
    info!("\n{:?}", page_dir);

    loop { /* loop forever here */ }
}