/*! OldKernel logger implementation */

use shared::logger::{
    logger::LevelFilter,
    writers::UartWriter
};
use sync::mutex::spin::RawSpinMutex;

use crate::{
    cmdline::cmdline_info,
    log::locked::LockedBufferedLogger
};

mod locked;
mod writer;

/* global buffered logger, initialized by <log_init()> */
static mut KERN_LOGGER: LockedBufferedLogger<RawSpinMutex, UartWriter> =
    LockedBufferedLogger::new_uninitialized();

/* size of the logger buffer, initialized by <log_init()> */
static mut KERN_LOG_BUFFER_SIZE: usize = 0;

/**
 * Initializes the global logger instance
 */
pub fn log_init() {
    /* enable the global logger as global for the log crate too */
    unsafe {
        KERN_LOGGER.init();
        KERN_LOGGER.enable_as_global().unwrap();

        /* set the max logging level reading it from the cmdline */
        KERN_LOGGER.set_max_logging_level_from_cmdline(cmdline_info().cmdline_args(),
                                                       LevelFilter::Debug);
    }

    /* read from the command line whether a valid buffer size is given */
    let buffer_size = cmdline_info().cmdline_args().value_by_key("-log-buffer-size", 512);
    unsafe {
        KERN_LOG_BUFFER_SIZE = buffer_size;
    }
}

/**
 * Enables the logger line-buffering
 */
pub fn log_enable_buffering(use_previous_buffer_if_any: bool) {
    unsafe {
        KERN_LOGGER.enable_buffering(use_previous_buffer_if_any, KERN_LOG_BUFFER_SIZE)
    }
}
