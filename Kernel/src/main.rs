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
           const_mut_refs,
           step_trait,
           alloc_error_handler,
           const_btree_new,
           array_methods,
           new_uninit)]
#![allow(dead_code)]

#[macro_use]
extern crate alloc;

use symbols::code_symbols::CodeSymbols;

use crate::{
    boot_info::BootInfo,
    dbg_print::{
        dbg_print_init,
        DbgLevel
    },
    dev::DevManager,
    heap::kernel_heap_init_eternal_pool,
    processor::Processor,
    task::scheduler::Scheduler,
    version::KERNEL_VERSION,
    vm::mem_manager::MemManager
};

mod addr;
mod arch;
mod boot_info;
mod dbg_print;
mod dev;
mod heap;
mod panic;
mod processor;
mod task;
mod version;
mod vm;

/**
 * Rust entry-point.
 *
 * Here is where the kernel starts his execution when
 * `Kernel/arch/<arch_name>/kernel_start.S` transfers the control to the
 * Rust code.
 *
 * This function is responsible to initialize all the sub-systems of the
 * kernel, discover and start the SMP if available and run the first process
 * to jump into the user-space
 */
#[no_mangle]
pub extern "C" fn bsp_rust_start(raw_boot_info_ptr: *const u8) -> ! {
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
    dbg_println!(DbgLevel::Info, "Initializing Kernel Symbols...");
    CodeSymbols::init_instance();

    /* initialize the CPU management for the bootstrap CPU */
    dbg_println!(DbgLevel::Info, "Initializing CPU Management...");
    Processor::init_instance();

    /* initialize the memory manager */
    dbg_println!(DbgLevel::Info, "Initializing Memory Management...");
    MemManager::init_instance();

    /* initialize the interrupts for this CPU */
    dbg_println!(DbgLevel::Info, "Initializing Interrupts Management...");
    Processor::instance_mut().init_interrupts_for_bsp();

    /* starting Symmetric Multi Processor */
    if Processor::instance().cores_count() > 1 {
        dbg_println!(DbgLevel::Info,
                     "Starting Other {} SMP APs (total cores count: {})...",
                     Processor::instance().cores_count() - 1,
                     Processor::instance().cores_count());
        //Processor::instance().start_smp();
    }

    /* initialize the task scheduler */
    dbg_println!(DbgLevel::Info, "Initializing Task Scheduler...");
    Scheduler::init_instance();

    /* FIXME debug printing to remove */
    {
        dbg_println!(DbgLevel::Debug,
                     "raw_info_ptr: {:#018x}, bsp_rust_start: {:#018x}",
                     raw_boot_info_ptr as usize,
                     bsp_rust_start as usize);
        dbg_println!(DbgLevel::Debug,
                     "boot_loader_name: '{}'",
                     BootInfo::instance().boot_loader_name());
        dbg_println!(DbgLevel::Debug,
                     "cmd_line_args:    '{}'",
                     BootInfo::instance().cmd_line_args());
        for boot_mem_area in BootInfo::instance().phys_mem_ranges().iter() {
            dbg_println!(DbgLevel::Debug,
                         "PhysMemRange {{ {}..{} }}",
                         boot_mem_area.start,
                         boot_mem_area.end);
        }

        dbg_println!(DbgLevel::Trace,
                     "Max Frequency: {}MHz, Bus Frequency: {}MHz",
                     Processor::instance().cores_max_frequency() / 1000000,
                     Processor::instance().cores_bus_frequency() / 1000000);
        dbg_println!(DbgLevel::Trace,
                     "Interrupts are enabled: {}",
                     Processor::instance().this_core().are_interrupts_enabled());
    }

    Processor::instance().this_core().enable_interrupts();
    dbg_println!(DbgLevel::Trace,
                 "Interrupts are enabled: {}",
                 Processor::instance().this_core().are_interrupts_enabled());
    unsafe {
        asm!("int 35");
    }
    panic!("TODO implement the remaining code");
}

pub extern "C" fn ap_rust_start() {
    let this_core_id = Processor::instance().this_core().id();

    /* initialize the CPU management for this AP */
    dbg_println!(DbgLevel::Info,
                 "Initializing AP{} Processor Management...",
                 this_core_id);
    Processor::instance_mut().init_this_ap();

    /* initialize the CPU management for this AP */
    dbg_println!(DbgLevel::Info,
                 "Initializing AP{} Interrupt Management...",
                 this_core_id);
    Processor::instance_mut().init_interrupts_for_this_ap();
}
