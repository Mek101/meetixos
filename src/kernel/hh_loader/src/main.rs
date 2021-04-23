#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

use hal::boot::infos::BootInfos;

mod arch;
mod log;

/* includes the module which links the kernel core binary */
include!(env!("KERNEL_BIN"));

#[no_mangle]
pub unsafe extern "C" fn hhl_rust_entry(raw_info_ptr: *const u8) -> ! {
    /* initialize the higher half loader's instance of the BootInfos */
    let _boot_info = BootInfos::from(raw_info_ptr);

    loop { /* loop forever here */ }
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
