/*! # HH Loader Logging
 *
 * Implements a simple logging sub-system used by the loader
 */

use core::cell::Cell;

use hal::uart::Uart;
use log::{Log, Metadata, Record};

/** # Loader Logger
 *
 * Implements a simple logger that writes into the UART
 */
struct Logger {
    m_uart_writer: Cell<Uart>
}

impl Logger {
    /** # Constructs a `Logger`
     *
     * The returned instance is already initialized and ready to write
     */
    fn new() -> Self {
        let mut uart = Uart::new();
        uart.init();
        Self { m_uart_writer: Cell::new(uart) }
    }

    /** Returns the mutable reference to the [`Uart`] writer
     *
     * [`Uart`]: /hal/uart/struct.Uart.html
     */
    fn writer(&self) -> &mut Uart {
        unsafe { &mut *self.m_uart_writer.as_ptr() }
    }
}

impl Log for Logger {
    /** Determines if a log message with the specified metadata would be
     * logged
     */
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true /* the logging level is already managed by the log crate */
    }

    /** Logs the [`Record`]
     *
     * [`Record`]: https://docs.rs/log/0.4.14/log/struct.Record.html
     */
    fn log(&self, record: &Record) {
        write!(self.writer(),
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
