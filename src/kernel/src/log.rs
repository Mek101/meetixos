/*! # Kernel Logging
 *
 * Implements a simple logging sub-system used by all the kernel
 */

use core::{fmt::Write, str::FromStr};

pub use log::{debug, error, info, warn};
use log::{
    set_logger, set_max_level, LevelFilter, Log, Metadata, Record, SetLoggerError
};

use hal::{boot::infos::BootInfos, uart::Uart};
use sync::SpinMutex;

/** Global kernel logger instance, is initialized by the [`init_logger()`]
 * function in the kernel bootstrap
 *
 * [`init_logger()`]: /kernel/log/fn.init_logger.html
 */
static mut KERN_LOGGER: Option<Logger> = None;

/** Default logging level, used as fallback when no valid filters are given
 * via kernel's command line
 */
const DEFAULT_LOGGING_LEVEL: LevelFilter = LevelFilter::Debug;

/** # Initializes the logger module
 *
 * Initializes the global kernel logger instance and initializes the
 * external logging framework
 */
pub fn init_logger() -> Result<(), SetLoggerError> {
    unsafe {
        assert!(KERN_LOGGER.is_none(), "Re-initializing global logger");

        /* initialize the logger and the `log` framework providing the global logger
         * instance
         */
        KERN_LOGGER = Some(Logger::new());
        set_logger(KERN_LOGGER.as_ref().unwrap_unchecked())?;
    }

    /* obtain from the from the bootloader informations the command-line
     * arguments and search for the `-loglvl` key, if provided (and have a valid
     * value) use it, otherwise fallback to the `DEFAULT_LOGGING_LEVEL`
     */
    let filter_level = {
        let infos = BootInfos::obtain();
        infos.cmdline_args()
             .find_key("-loglvl")
             .map_or(DEFAULT_LOGGING_LEVEL, |arg| {
                 if let Ok(level) = LevelFilter::from_str(arg.value()) {
                     level
                 } else {
                     DEFAULT_LOGGING_LEVEL
                 }
             })
    };

    /* set the `log`s crate static maximum level, then return `Ok`, the log
     * module is successfully initialized
     */
    set_max_level(filter_level);
    Ok(())
}

/** # Kernel Logger
 *
 * Implements a simple thread safe logger that writes into the UART
 */
struct Logger {
    m_uart_writer: SpinMutex<Uart>
}

impl Logger {
    /** # Constructs a `Logger`
     *
     * The returned instance is already initialized and ready to write
     */
    fn new() -> Self {
        let mut uart = Uart::new();
        uart.init();
        Self { m_uart_writer: SpinMutex::new(uart) }
    }
}

impl Log for Logger {
    /** Determines if a log message with the specified metadata would be
     * logged
     */
    fn enabled(&self, _metadata: &Metadata) -> bool {
        /* TODO a special file/syscall which dynamically allow setup the log level */
        true
    }

    /** Logs the [`Record`]
     *
     * [`Record`]: https://docs.rs/log/0.4.14/log/struct.Record.html
     */
    fn log(&self, record: &Record) {
        let mut writer = self.m_uart_writer.lock();
        write!(writer,
               "[{: >5} <> {: <20}] {}\n",
               record.level(),  /* human readable log-level */
               record.target(), /* path to the rust module relative to the kernel */
               record.args()).unwrap()
    }

    /** Flushes any buffered records
     */
    fn flush(&self) {
        /* No supported buffering */
    }
}
