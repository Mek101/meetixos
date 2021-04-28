/*! # x86_64 Logging Implementation
 *
 * Implements a serial COM1 uart writer
 */

use core::fmt;

use uart_16550::SerialPort;

pub use X64Uart as HwUart;

use crate::hal::uart::HwUartBase;

/** The x86_64 implementation of the UART module uses the COM1 for
 * communication.
 *
 * Change this value to the other COMx base to change the kernel's serial
 * output port
 */
const SERIAL_COM1_PORT_BASE: u16 = 0x3F8;

/** # x86_64 UART Hardware
 *
 * Simple wrapper around the [`uart_16550::SerialPort`] to implement the
 * [`UartBase`] trait
 *
 * [`uart_16550::SerialPort`]: https://docs.rs/uart_16550/0.2.12/uart_16550/struct.SerialPort.html
 * [`UartBase`]: /hal/uart/trait.UartBase.html
 */
pub struct X64Uart {
    m_serial_port: SerialPort
}

impl fmt::Write for X64Uart {
    /** Writes a string slice into this writer, returning whether the write
     * succeeded
     */
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.m_serial_port.write_str(s)
    }
}

impl HwUartBase for X64Uart {
    /** Constructs an uninitialized `X64UartHw`
     */
    fn new() -> Self {
        unsafe { Self { m_serial_port: SerialPort::new(SERIAL_COM1_PORT_BASE) } }
    }

    /** Initializes the underling serial COM port
     */
    fn init_hw(&mut self) -> bool {
        self.m_serial_port.init();
        true
    }
}
