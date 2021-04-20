#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

mod arch;

pub extern "C" fn loader_rust_entry(_raw_info_ptr: *const u8) -> ! {
    loop {}
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
