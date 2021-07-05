#![feature(global_asm)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod arch;
mod panic;

#[no_mangle]
pub unsafe extern "C" fn kernel_rust_start() -> ! {
    loop {}
}
