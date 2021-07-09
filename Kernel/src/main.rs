#![no_std]
#![no_main]
#![feature(asm, global_asm, panic_info_message, const_fn_trait_bound)]
#![feature(array_methods)]
#![allow(dead_code)]

/* TODO heap allocation */
//extern crate alloc;

use crate::{
    boot_info::BootInfo,
    dbg::{
        display_pretty::DisplaySizePretty,
        print::{
            dbg_print_init,
            DbgLevel
        }
    },
    version::KERNEL_VERSION
};

mod addr;
mod arch;
mod boot_info;
mod cpu;
mod dbg;
mod dev;
mod heap;
mod mem;
mod panic;
mod version;

#[no_mangle]
pub extern "C" fn kernel_rust_start(raw_boot_info_ptr: *const u8) -> ! {
    /* initialize the global instance of the boot boot structure */
    BootInfo::init_instance(raw_boot_info_ptr);

    /* initialize debug printing and print the header */
    dbg_print_init();
    dbg_println!(DbgLevel::Info, "MeetiX Kernel v{} is Booting...", KERNEL_VERSION);
    dbg_println!(DbgLevel::Info, "An Open Source OS Project written in Rust");

    dbg_println!(DbgLevel::Debug,
                 "raw_info_ptr: {:#018x}, kernel_rust_start: {:#018x}",
                 raw_boot_info_ptr as usize,
                 kernel_rust_start as usize);
    dbg_println!(DbgLevel::Debug,
                 "boot_loader_name: '{}'",
                 BootInfo::instance().boot_loader_name());
    dbg_println!(DbgLevel::Debug,
                 "cmd_line_args:    '{}'",
                 BootInfo::instance().cmd_line_args());
    for boot_mem_area in BootInfo::instance().boot_mem_areas().iter() {
        dbg_println!(DbgLevel::Debug,
                     "BootMemArea({}, {})",
                     boot_mem_area.start_addr(),
                     boot_mem_area.size().display_pretty());
    }
    dbg_println!(DbgLevel::Info,
                 "Available memory: {}",
                 BootInfo::instance().boot_mem_areas()
                                     .iter()
                                     .map(|mem_area| mem_area.size())
                                     .sum::<usize>()
                                     .display_pretty());

    panic!("TODO implement the remaining code");
}
