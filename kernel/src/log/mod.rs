/*! Kernel logger implementation */

use core::{
    num::NonZeroUsize,
    str::FromStr
};

use shared::{
    infos::info::BootInfos,
    logger::{
        logger::LevelFilter,
        writers::UartWriter
    }
};
use sync::RawSpinMutex;

use crate::log::locked::LockedBufferedLogger;

mod locked;
mod writer;

/**
 * Global kernel logger instance, is initialized by the `init_logger()`
 * function called by `kern_start()`
 */
static mut KERN_LOGGER: LockedBufferedLogger<RawSpinMutex, UartWriter> =
    LockedBufferedLogger::new_uninitialized();

/**
 * Default logging level, used as fallback when no valid filters are given
 * via kernel's command line
 */
const DEFAULT_LOGGING_LEVEL: LevelFilter = LevelFilter::Debug;

/**
 * Default buffer logging size in bytes, used as fallback when no valid
 * values are given via kernel's command line
 */
const DEFAULT_BUFFER_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(512) };

/**
 * Initializes the global logger instance
 */
pub fn init_logger() {
    /* enable the global logger as global for the log crate too */
    unsafe {
        KERN_LOGGER.enable_as_global().unwrap();
    }

    /* search for `-log-level` key into the kernel's command line */
    let level_filter = {
        let infos = BootInfos::obtain();
        infos.cmdline_args()
            .find_key("-log-level")
            .map_or(DEFAULT_LOGGING_LEVEL, |arg| {
                LevelFilter::from_str(arg.value()).unwrap_or(DEFAULT_LOGGING_LEVEL)
            })
    };

    /* hide all the logs above the given filter level */
    log_set_max_level(level_filter);
}

/**
 * Hides all the logs above the given filter
 */
pub fn log_set_max_level(level_filter: LevelFilter) {
    /* hide all the logs above the given filter level */
    unsafe {
        KERN_LOGGER.set_max_logging_level(level_filter);
    }
}

/**  
 * Enables the logging line-buffering
 */
pub fn log_enable_buffering(use_previous_buffer_if_any: bool) {
    /* search for `-log-buffer-size` key into the kernel's command line if
     * <truncate_to_init_size> is true, otherwise use 0, to tell to the buffer
     * manager to simply re-use the previously allocated buffer
     */
    let buffer_size =
        BootInfos::obtain().cmdline_args()
                           .find_key("-log-buffer-size")
                           .map_or(DEFAULT_BUFFER_SIZE, |value| {
                               if let Ok(value) = usize::from_str(value.as_str()) {
                                   if let Some(value) = NonZeroUsize::new(value) {
                                       value
                                   } else {
                                       DEFAULT_BUFFER_SIZE
                                   }
                               } else {
                                   DEFAULT_BUFFER_SIZE
                               }
                           });

    unsafe {
        KERN_LOGGER.enable_buffering(use_previous_buffer_if_any, buffer_size);
    }
}

/**  
 * Disables the logging line-buffering.
 *
 * `keep_buffer` tells to the underling buffer manager whether the memory
 * must be kept or freed
 */
pub fn log_disable_buffering(keep_buffer: bool) {
    unsafe {
        KERN_LOGGER.disable_buffering(keep_buffer);
    }
}
