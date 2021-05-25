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
    logger::log_info
};

use crate::{
    log::log_init,
    version::KERN_VERSION
};

mod log;
mod mem;
mod panic;
mod version;

/**
 * Rust kernel entry point, here is where the kernel core starts his
 * execution
 */
#[no_mangle]
pub unsafe extern "C" fn kern_start(_loader_info_ptr: *const LoaderInfo) {
    /* initialize the logger, to be able to print in a formatted way */
    log_init();

    /* print the kernel header */
    log_info!("MeetiX Kernel v{}", KERN_VERSION);
    loop {}
}
