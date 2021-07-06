#![no_std]
#![no_main]
#![feature(global_asm, panic_info_message)]

mod arch;
mod panic;

#[no_mangle]
pub extern "C" fn kernel_rust_start(_raw_info_ptr: usize) -> ! {
    loop {}
}
