#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

use hal::boot::infos::BootInfos;

mod arch;

include!(env!("KERNEL_BIN"));

#[no_mangle]
pub unsafe extern "C" fn hhl_rust_entry(raw_info_ptr: *const u8) -> ! {
    /* initialize the higher half loader's instance of the BootInfos */
    let _ = BootInfos::from(raw_info_ptr);

    loop { /* loop forever here */ }
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
