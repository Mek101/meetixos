/*! Kernel logger implementation */

use shared::logger::{
    logger::LevelFilter,
    writers::UartWriter
};
use sync::RawSpinMutex;

use crate::log::locked::LockedBufferedLogger;

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
        KERN_LOGGER.enable_as_global().unwrap();
    }

    /* obtain from the from the bootloader information the command-line
     * arguments and search for the `-log-level` key, if provided (and have a
     * valid value) use it, otherwise fallback to the `DEFAULT_LOGGING_LEVEL`
     */
    /*let level_filter =
    boot_info().cmdline_args()
               .find_key("-log-level")
               .map(|cmdline_arg| cmdline_arg.value())
               .map(|arg_value| {
                   LevelFilter::from_str(arg_value).expect("Invalid '-log-level' \
                                                            value")
               })
               .unwrap_or(LevelFilter::Debug);*/

    /* hide all the logs above the given filter level */
    unsafe {
        KERN_LOGGER.set_max_logging_level(LevelFilter::Trace);
    }
}
