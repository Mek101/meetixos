/*! Kernel land logger writers implementations */

use core::{
    fmt,
    fmt::Write
};

use crate::uart::Uart;

/**
 * Implements the `LoggerWriter` for the `Uart` struct
 */
pub struct UartWriter {
    m_uart: Uart
}

impl LoggerWriter for UartWriter {
    fn new() -> Self {
        let mut uart = Uart::new();
        uart.init();
        Self { m_uart: uart }
    }
}

impl fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.m_uart.write_str(s)
    }
}

/**
 * Defines the methods and the markers that each backend writer must
 * implement.
 *
 * This trait is used by the `Logger` to communicate with the real logger
 * storage/hardware (a serial output, the video, or a file)
 */
pub trait LoggerWriter: Write + Send + Sync {
    /**
     * Constructs an initialized `LoggerWriter`
     */
    fn new() -> Self;
}
