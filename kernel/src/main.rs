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
    logger::info
};

use crate::{
    boot_info::boot_info_init,
    log::log_init,
    version::KERN_VERSION
};

mod boot_info;
mod log;
mod mem;
mod panic;
mod version;

/**
 * Rust kernel entry point, here is where the kernel core starts his
 * execution
 */
#[no_mangle]
pub unsafe extern "C" fn kern_start(loader_info_ptr: *const LoaderInfo) {
    /* initialize the given raw information pointer */
    boot_info_init(loader_info_ptr);

    /* initialize the logger, to be able to print in a formatted way */
    log_init();

    /* print the kernel header */
    info!("MeetiX Kernel v{}", KERN_VERSION);
    loop {}
}
