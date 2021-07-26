/*! Universal asynchronous receiver-transmitter */

use core::fmt;

use crate::dev::TDevice;

/**
 * Universal Asynchronous Receiver-Transmitter driver interface
 */
pub trait TUartDevice: TDevice {
    /**
     * Writes the given `&str` to the output UART.
     *
     * NOTE: the implementation is responsible of thread-synchronization
     */
    fn write_str(&self, str: &str) -> fmt::Result;
}
