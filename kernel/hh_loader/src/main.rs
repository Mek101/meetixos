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
#![feature(global_asm, iter_advance_by, panic_info_message, min_type_alias_impl_trait)]

use shared::{
    addr::virt::VirtAddr,
    dbg::dbg_display_size,
    infos::info::BootInfos,
    logger::info
};

use crate::{
    loader::{
        loader_kernel_core_size,
        loader_load_core
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
    /* interpret the raw pointer given to fill the <BootInfos> */
    let boot_info = BootInfos::from_raw(raw_info_ptr);

    /* initialize the logger, to be able to print in a formatted way */
    log_init();

    /* print the hh_loader's header */
    info!("MeetiX Kernel Loader v{}", HHL_VERSION);
    info!("\tKernel size: {}", dbg_display_size(loader_kernel_core_size()));

    /* pre initialize physical memory, obtain how many bitmap pages are necessary */
    let necessary_bitmap_pages = phys_pre_init();

    /* organize the VM layout for the kernel */
    info!("Randomizing Kernel Core's VM Layout...");
    vml_randomize_core_layout(necessary_bitmap_pages);

    /* initialize the physical memory allocator */
    phys_init();

    /* load the kernel core now */
    loader_load_core();

    info!("Raw info ptr: {:#x}", VirtAddr::from(raw_info_ptr));
    boot_info.cmdline_args().iter().for_each(|arg| info!("Arg: {}", arg.as_str()));
    boot_info.mem_areas().iter().for_each(|mem_area| {
                                    info!("{:?}, {}",
                                          mem_area,
                                          dbg_display_size(mem_area.size()))
                                });

    let page_dir = paging_current_page_dir();
    info!("\n{:?}", page_dir);

    loop { /* loop forever here */ }
}
