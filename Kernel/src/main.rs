/*! # MeetiX Kernel
 *
 * Monolithic kernel implementation for MeetiX OS
 */

#![no_std]
#![no_main]
#![feature(asm,
           global_asm,
           panic_info_message,
           const_fn_trait_bound,
           step_trait,
           alloc_error_handler,
           const_btree_new,
           array_methods,
           once_cell)]
#![allow(dead_code)]

#[macro_use]
extern crate alloc;

use symbols::code_symbols::CodeSymbols;

use crate::{
    boot_info::BootInfo,
    cpu::Cpu,
    dbg_print::{
        dbg_print_init,
        DbgLevel
    },
    dev::DevManager,
    heap::kernel_heap_init_eternal_pool,
    version::KERNEL_VERSION,
    vm::mem_manager::MemManager
};

mod addr;
mod arch;
mod boot_info;
mod cpu;
mod dbg_print;
mod dev;
mod heap;
mod panic;
mod version;
mod vm;
mod filesystem;
mod info;
mod mem;

/**
 * Rust entry-point.
 *
 * Here is where the kernel starts his execution when
 * `Kernel/arch/<arch_name>/kernel_start.S` transfers the control to the
 * Rust code
 */
#[no_mangle]
pub extern "C" fn kernel_rust_start(raw_boot_info_ptr: *const u8) -> ! {
    /* initialize the kernel heap since the BootInfo & DevManager could use it */
    kernel_heap_init_eternal_pool();

    /* initialize the global instance of the boot boot structure */
    BootInfo::init_instance(raw_boot_info_ptr);

    /* initializes the fundamental drivers, like the serial for debug printing */
    DevManager::early_init();

    /* initialize debug printing and print the header */
    dbg_print_init();
    dbg_println!(DbgLevel::Info, "MeetiX Kernel v{} is Booting...", KERNEL_VERSION);
    dbg_println!(DbgLevel::Info, "An Open Source OS Project written in Rust");

    /* initialize the kernel symbols */
    dbg_println!(DbgLevel::Trace, "Initializing Kernel Symbols...");
    CodeSymbols::init_instance();

    /* initialize the CPU management for the bootstrap CPU */
    dbg_println!(DbgLevel::Trace, "Initializing CPU Management...");
    Cpu::early_init();

    /* initialize the memory manager */
    dbg_println!(DbgLevel::Trace, "Initializing Memory Management...");
    MemManager::init_instance();

    /* initialize the interrupts for this CPU */
    dbg_println!(DbgLevel::Trace, "Initializing Interrupts Management...");
    Cpu::init_interrupts_for_this();

    /* FIXME debug printing to remove */
    {
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
                         "BootMemArea({}..{})",
                         boot_mem_area.start,
                         boot_mem_area.end);
        }

        dbg_println!(DbgLevel::Trace,
                     "Cpu: Base Frequency: {}MHz, Max Frequency: {}MHz, Bus Frequency: \
                      {}MHz",
                     Cpu::current().base_frequency(),
                     Cpu::current().max_frequency(),
                     Cpu::current().bus_frequency());
        dbg_println!(DbgLevel::Trace,
                     "Interrupts are enabled: {}",
                     Cpu::current().are_interrupts_enabled());
    }
    panic!("TODO implement the remaining code");
}
