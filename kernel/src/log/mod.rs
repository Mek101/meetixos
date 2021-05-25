/*! Kernel logger implementation */

use shared::logger::{
    logger::LevelFilter,
    writers::UartWriter
};
use sync::RawSpinMutex;

use crate::{
    boot_info::boot_info,
    log::locked::LockedBufferedLogger
};

mod locked;
mod writer;

/* global buffered logger, initialized by <log_init()> */
static mut KERN_LOGGER: LockedBufferedLogger<RawSpinMutex, UartWriter> =
    LockedBufferedLogger::new_uninitialized();

/**
 * Initializes the global logger instance
 */
pub fn log_init() {
    /* enable the global logger as global for the log crate too */
    unsafe {
        KERN_LOGGER.init();
        KERN_LOGGER.enable_as_global().unwrap();

        /* set the max logging level reading it from the cmdline */
        KERN_LOGGER.set_max_logging_level_from_cmdline(boot_info().cmdline_args(),
                                                       LevelFilter::Debug);
    }
}
