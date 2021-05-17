/*! # Higher Half Loader
 *
 * Implements the kernel stage after the bootloader but before the kernel
 * core.
 *
 * This binary is responsible to initialize the architecture, randomize the
 * kernel layout, initialize the physical memory bitmap, load and map the
 * kernel core into the higher half and jump into it.
 *
 * The kernel core is statically linked into the `.data` section of the
 * loader and loaded as ELF executable by the loader
 */

#![no_std]
#![no_main]
#![feature(global_asm,
           iter_advance_by,
           panic_info_message,
           min_type_alias_impl_trait,
           array_methods)]

use shared::{
    info::info::BootInfo,
    logger::info
};

use crate::{
    loader::{
        loader_load_core,
        loader_preload_core
    },
    log::log_init,
    mem::{
        paging::paging_current_page_dir,
        phys::{
            phys_init,
            phys_pre_init
        },
        vm_layout::vml_randomize_core_layout
    },
    version::HHL_VERSION
};

mod arch;
mod loader;
mod log;
mod mem;
mod panic;
mod version;

/**
 * Rust entrypoint, here is where the 64bit rust code starts his execution
 */
#[no_mangle]
pub unsafe extern "C" fn hhl_rust_entry(raw_info_ptr: *const u8) -> ! {
    /* interpret the raw pointer given to fill the <BootInfo> */
    let _ = BootInfo::from_raw(raw_info_ptr);

    /* initialize the logger, to be able to print in a formatted way */
    log_init();

    /* print the hh_loader's header */
    info!("MeetiX Kernel Loader v{}", HHL_VERSION);

    /* load the ELF file of the kernel's core */
    info!("Pre-loading Kernel's Core");
    loader_preload_core();

    /* pre initialize physical memory, obtain how many bitmap pages are necessary */
    info!("Pre-initializing PhysMem Manager");
    let necessary_bitmap_pages = phys_pre_init();

    /* organize the VM layout for the kernel */
    info!("Randomizing Kernel Core's VM Layout...");
    vml_randomize_core_layout(necessary_bitmap_pages);

    /* initialize the physical memory allocator */
    info!("Initializing PhysMem Manager");
    phys_init();

    /* load the kernel core now */
    info!("Loading Kernel's Core");
    loader_load_core();

    let page_dir = paging_current_page_dir();
    info!("Current PageDir composition:\n{:?}", page_dir);

    loop { /* loop forever here */ }
}
