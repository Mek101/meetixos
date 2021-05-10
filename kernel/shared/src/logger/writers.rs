/*! kernel land logger writers implementations */

use core::fmt;

use crate::{
    logger::logger::LoggerWriter,
    uart::Uart
};

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
