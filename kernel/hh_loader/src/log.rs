/*! HH_Loader logging */

use core::str::FromStr;

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
    }

    /* obtain from the from the bootloader information the command-line
     * arguments and search for the `-log-level` key, if provided (and have a
     * valid value) use it, otherwise fallback to the `DEFAULT_LOGGING_LEVEL`
     */
    let level_filter =
        boot_info().cmdline_args()
                   .find_key("-log-level")
                   .map(|cmdline_arg| cmdline_arg.value())
                   .map(|arg_value| {
                       LevelFilter::from_str(arg_value).expect("Invalid '-log-level' \
                                                                value")
                   })
                   .unwrap_or(LevelFilter::Debug);

    /* hide all the logs above the given filter level */
    unsafe {
        HHL_LOGGER.set_max_logging_level(level_filter);
    }
}
