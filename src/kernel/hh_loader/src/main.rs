#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(option_result_unwrap_unchecked)]

use core::panic::PanicInfo;

use hal::boot::infos::BootInfos;

use crate::log::{init_logger, info};
use hal::paging::PageDir;

mod arch;
mod log;

/* includes the module which links the kernel core binary */
include!(env!("KERNEL_BIN"));

#[no_mangle]
pub unsafe extern "C" fn hhl_rust_entry(raw_info_ptr: *const u8) -> ! {
    /* initialize the higher half loader's instance of the BootInfos */
    let _boot_info = BootInfos::from(raw_info_ptr);

    init_logger().unwrap();

    info!("MeetiX Kernel Loader v0.1.0");
    info!("Kernel size: {}", KERNEL_SIZE);
    info!("Kernel bytes[0]: {}", KERNEL_BYTES[0]);

    let page_dir = PageDir::active_page_dir(0);
    info!("\n{:?}", page_dir);

    loop { /* loop forever here */ }
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
