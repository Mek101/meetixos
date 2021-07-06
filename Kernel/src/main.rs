#![no_std]
#![no_main]
#![feature(asm, global_asm, panic_info_message, const_fn_trait_bound)]
#![allow(dead_code)]

use crate::{
    dbg::print::DbgLevel,
    version::KERNEL_VERSION
};

mod addr;
mod arch;
mod dbg;
mod dev;
mod mem;
mod panic;
mod version;

#[no_mangle]
pub extern "C" fn kernel_rust_start(_raw_info_ptr: usize) -> ! {
    dbg_println!(DbgLevel::Info, "MeetiX Kernel v{} is Booting...", KERNEL_VERSION);
    loop {}
}
