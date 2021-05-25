/*! # MeetiX Kernel
 *
 * Implements the MeetiX kernel core
 */

#![no_std]
#![no_main]
#![feature(panic_info_message, alloc_error_handler, const_fn_trait_bound)]

extern crate alloc;

use shared::{
    info::descriptor::LoaderInfo,
    logger::{
        info,
        trace
    }
};

use crate::{
    cmdline::{
        cmdline_info,
        cmdline_info_init
    },
    log::{
        log_enable_buffering,
        log_init
    },
    mem::{
        heap::heap_init,
        paging::paging_unmap_loader,
        phys::phys_init,
        vm_layout::vml_init_from_loader_info
    },
    version::KERN_VERSION
};

mod cmdline;
mod log;
mod mem;
mod panic;
mod version;

/**
 * Rust kernel entry point, here is where the kernel core starts his
 * execution
 */
#[no_mangle]
pub unsafe extern "C" fn kern_start(loader_info: &LoaderInfo) {
    /* initialize the command line info from the loader info */
    cmdline_info_init(loader_info);

    /* initialize the logger, to be able to print in a formatted way */
    log_init();

    /* print the kernel header */
    trace!("Booted By {}", cmdline_info().bootloader_name());
    info!("MeetiX Kernel v{}", KERN_VERSION);

    /* initialize the kernel <VMLayout> from the LoaderInfo given */
    info!("Initializing Kernel VM Layout...");
    vml_init_from_loader_info(loader_info);

    /* initialize the physical memory allocator */
    info!("Initializing Kernel Physical Memory Management...");
    phys_init(loader_info.bitmap_allocated_bits());

    /* unmap the kernel loader, after this call <loader_info> will be invalid */
    info!("Unmapping Kernel Loader...");
    paging_unmap_loader(loader_info);

    /* initialize the kernel heap */
    info!("Initializing Kernel Heap...");
    heap_init();

    /* enable logging buffering */
    info!("Enabling Kernel Logging Buffering...");
    log_enable_buffering(false);

    trace!("PageDir:\n{:?}", crate::mem::paging::paging_current_page_dir());
    loop {}
}
