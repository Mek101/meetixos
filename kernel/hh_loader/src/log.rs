/*! HH_Loader logging */

use core::str::FromStr;

use shared::{
    infos::info::BootInfos,
    logger::{
        logger::{
            LevelFilter,
            Logger
        },
        writers::UartWriter
    }
};

/* global logger */
static mut HHL_LOGGER: Logger<UartWriter> = Logger::new_uninitialized();

/* default logging level when no valid command line filter is given */
const DEFAULT_LOGGING_LEVEL: LevelFilter = LevelFilter::Debug;

/**
 * Initializes the global logger instance
 */
pub fn log_init() {
    /* enable the global logger as global for the log crate too */
    unsafe {
        HHL_LOGGER.enable_as_global().unwrap();
    }

    /* obtain from the from the bootloader informations the command-line
     * arguments and search for the `-log-level` key, if provided (and have a
     * valid value) use it, otherwise fallback to the `DEFAULT_LOGGING_LEVEL`
     */
    let level_filter = {
        let infos = BootInfos::obtain();
        infos.cmdline_args()
            .find_key("-log-level")
            .map_or(DEFAULT_LOGGING_LEVEL, |arg| {
                LevelFilter::from_str(arg.value()).unwrap_or(DEFAULT_LOGGING_LEVEL)
            })
    };

    /* hide all the logs above the given filter level */
    unsafe {
        HHL_LOGGER.set_max_logging_level(level_filter);
    }
}
