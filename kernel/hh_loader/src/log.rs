/*! # HH Loader Logging
 *
 * Implements a simple logging sub-system used by the loader
 */

use core::{fmt, str::FromStr};

use hal::{boot_infos::BootInfos, uart::Uart};
use logger::{LevelFilter, Logger, LoggerWriter};
use sync::RawSpinMutex;

/** Global kernel loader logger instance, is initialized by the
 * [`init_logger()`] function called by [`hhl_rust_entry()`]
 *
 * [`init_logger()`]: fn.init_logger.html
 * [`hhl_rust_entry()`]: fn.hhl_rust_entry.html
 */
static mut HHL_LOGGER: Logger<UartWriter, RawSpinMutex> = Logger::new_uninitialized();

/** Default logging level, used as fallback when no valid filters are given
 * via kernel's command line
 */
const DEFAULT_LOGGING_LEVEL: LevelFilter = LevelFilter::Debug;

/** # Initializes the logger
 *
 * Initializes the global logger instance
 */
pub fn init_logger() {
    /* enable the global logger as global for the log crate too */
    unsafe {
        HHL_LOGGER.enable_as_global().unwrap();
    }

    /* obtain from the from the bootloader informations the command-line
     * arguments and search for the `-log-level` key, if provided (and have a
     * valid value) use it, otherwise fallback to the `DEFAULT_LOGGING_LEVEL`
     */
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
        HHL_LOGGER.set_max_logging_level(filter_level);
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
