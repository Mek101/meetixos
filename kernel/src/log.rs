/*! Kernel logger implementation */

use core::{
    fmt,
    str::FromStr
};

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

use sync::RawSpinMutex;

/**
 * Global kernel logger instance, is initialized by the `init_logger()`
 * function called by `kern_start()`
 */
static mut KERN_LOGGER: Logger<UartWriter, RawSpinMutex> = Logger::new_uninitialized();

/**
 * Default logging level, used as fallback when no valid filters are given
 * via kernel's command line
 */
const DEFAULT_LOGGING_LEVEL: LevelFilter = LevelFilter::Debug;

/** Default buffer logging size in bytes, used as fallback when no valid
 * values are given via kernel's command line
 */
const DEFAULT_BUFFER_SIZE: usize = 512;

/**
 * Initializes the global logger instance
 */
pub fn init_logger() {
    /* enable the global logger as global for the log crate too */
    unsafe {
        KERN_LOGGER.enable_as_global().unwrap();
    }

    /* search for `-log-level` key into the kernel's command line */
    let filter_level = {
        let infos = BootInfos::obtain();
        infos.cmdline_args()
            .find_key("-log-level")
            .map_or(DEFAULT_LOGGING_LEVEL, |arg| {
                LevelFilter::from_str(arg.value()).unwrap_or(DEFAULT_LOGGING_LEVEL)
            })
    };

    /* hide all the logs above the given filter level */
    unsafe {
        KERN_LOGGER.set_max_logging_level(filter_level);
    }
}

/**  
 * Enables the logging line-buffering
 */
pub fn enable_logger_buffering() {
    unsafe {
        KERN_LOGGER.enable_buffering();
    }
}
