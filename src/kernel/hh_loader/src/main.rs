#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

mod arch;

//include!(concat!(env!("OUT_DIR"), "/kernel.rs"));

#[no_mangle]
pub unsafe extern "C" fn hhl_rust_entry(_raw_info_ptr: *const u8) -> ! {
    loop { /* loop forever here */ }
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
