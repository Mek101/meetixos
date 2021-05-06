/*! # Kernel Logging
 *
 * Implements a simple UART logging sub-system used by all the kernel.
 *
 * TODO for now the implementation uses the same backend writer of the
 *      hh_loader, implement a `FileWriter` when filesystem become reality
 */

use core::{
    fmt,
    str::FromStr
};

use hal::{
    boot_infos::BootInfos,
    uart::Uart
};
use logger::{
    LevelFilter,
    Logger,
    LoggerWriter
};
use sync::RawSpinMutex;

/** Global kernel logger instance, is initialized by the
 * [`init_logger()`] function called by [`hhl_rust_entry()`]
 *
 * [`init_logger()`]: fn.init_logger.html
 * [`hhl_rust_entry()`]: fn.hhl_rust_entry.html
 */
static mut KERN_LOGGER: Logger<UartWriter, RawSpinMutex> = Logger::new_uninitialized();

/** Default logging level, used as fallback when no valid filters are given
 * via kernel's command line
 */
const DEFAULT_LOGGING_LEVEL: LevelFilter = LevelFilter::Debug;

/** Default buffer logging size in bytes, used as fallback when no valid
 * values are given via kernel's command line
 */
const DEFAULT_BUFFER_SIZE: usize = 512;

/** # Initializes the logger
 *
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

/** # Enables the logging line-buffering
 *
 * Searches into the kernel's command line for `-log-buffer-size` and uses
 * the value as buffer size
 */
pub fn enable_logger_buffering() {
    /* search for `-log-buffer-size` key into the kernel's command line */
    let buffer_size = {
        let infos = BootInfos::obtain();
        infos.cmdline_args()
            .find_key("-log-buffer-size")
            .map_or(DEFAULT_BUFFER_SIZE, |value| {
                if let Ok(value) = usize::from_str(value.as_str()) {
                    value
                } else {
                    DEFAULT_BUFFER_SIZE
                }
            })
    };

    unsafe {
        KERN_LOGGER.enable_buffering(buffer_size);
    }
}

/** # UART LoggerWriter
 *
 * Implements the [`LoggerWriter`] for the [`Uart`] struct
 *
 * [`LoggerWriter`]: trait.LoggerWriter.html
 * [`Uart`]: struct.Uart.html
 */
pub struct UartWriter {
    m_uart: Uart
}

impl LoggerWriter for UartWriter {
    /** Constructs an initialized `LoggerWriter`
     */
    fn new() -> Self {
        let mut uart = Uart::new();
        uart.init();
        Self { m_uart: uart }
    }
}

impl fmt::Write for UartWriter {
    /** Writes a string slice into this writer, returning whether the write
     * succeeded
     */
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.m_uart.write_str(s)
    }
}
