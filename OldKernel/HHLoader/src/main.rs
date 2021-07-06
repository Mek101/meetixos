/*! # Higher Half Loader
 *
 * Implements the OldKernel stage after the bootloader but before the
 * OldKernel core.
 *
 * This binary is responsible to initialize the architecture, randomize the
 * OldKernel layout, initialize the physical memory bitmap, load and map the
 * OldKernel core into the higher half and jump into it.
 *
 * The OldKernel core is statically linked into the `.data` section of the
 * loader and loaded as ELF executable by the loader
 */

#![no_std]
#![no_main]
#![feature(global_asm, iter_advance_by, panic_info_message, array_methods, asm)]

use shared::logger::info;

use crate::{
    info::info_init_boot_info,
    loader::{
        loader_init_core_cache,
        loader_load_core
    },
    log::log_init,
    mem::{
        paging::paging_map_phys_mem,
        phys::{
            phys_init,
            phys_pre_init
        },
        vm_layout::vml_randomize_core_layout
    },
    version::HHL_VERSION
};

mod arch;
mod info;
mod loader;
mod log;
mod mem;
mod panic;
mod symbols;
mod version;

/**
 * Rust entrypoint, here is where the 64bit rust code starts his execution
 */
#[no_mangle]
pub unsafe extern "C" fn hhl_rust_entry(raw_info_ptr: *const u8) -> ! {
    /* initialize the given raw information pointer */
    info_init_boot_info(raw_info_ptr);

    /* initialize the logger, to be able to print in a formatted way */
    log_init();

    /* print the HHLoader's header */
    info!("MeetiX OldKernel Loader v{}", HHL_VERSION);

    /* load the ELF file of the OldKernel's core */
    info!("Initializing OldKernel's Core Cache...");
    loader_init_core_cache();

    /* pre initialize physical memory, obtain how many bitmap pages are necessary */
    info!("Initializing Physical Memory Management...");
    let necessary_bitmap_pages = phys_pre_init();

    /* organize the VM layout for the OldKernel */
    info!("Initializing OldKernel's Core VM Layout...");
    vml_randomize_core_layout(necessary_bitmap_pages);

    /* initialize the physical memory allocator */
    info!("Initializing Physical Memory Management...");
    phys_init();

    /* map the physical memory at the right area */
    info!("Initializing Paging...");
    paging_map_phys_mem();

    /* load the OldKernel core now */
    info!("Loading OldKernel's Core");
    loader_load_core();

    panic!("OldKernel Core loader returned");
}
