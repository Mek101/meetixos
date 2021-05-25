/*! HH_Loader logging */

use shared::logger::{
    logger::{
        LevelFilter,
        Logger
    },
    writers::UartWriter
};

use crate::info::boot_info;

/* global logger */
static mut HHL_LOGGER: Logger<UartWriter> = Logger::new_uninitialized();

/**
 * Initializes the global logger instance
 */
pub fn log_init() {
    /* enable the global logger as global for the log crate too */
    unsafe {
        HHL_LOGGER.init();
        HHL_LOGGER.enable_as_global().unwrap();

        /* set the max logging level reading it from the cmdline */
        HHL_LOGGER.set_max_logging_level_from_cmdline(boot_info().cmdline_args(),
                                                      LevelFilter::Debug);
    }
}
