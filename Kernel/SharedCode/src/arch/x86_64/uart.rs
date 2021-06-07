/*! x86_64 UART implementation */

use core::fmt;

use uart_16550::SerialPort;

use crate::uart::HwUartBase;

/**
 * The x86_64 implementation of the UART module uses the COM1 for
 * communication.
 *
 * Change this value to the other COMx base to change the Kernel's serial
 * output port
 */
const SERIAL_COM1_PORT_BASE: u16 = 0x3F8;

/**
 * x86_64 `HwUartBase` implementation based on `uart_16550::SerialPort`
 */
pub struct HwUart {
    m_serial_port: SerialPort
}

impl fmt::Write for HwUart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.m_serial_port.write_str(s)
    }
}

impl HwUartBase for HwUart {
    fn new() -> Self {
        unsafe { Self { m_serial_port: SerialPort::new(SERIAL_COM1_PORT_BASE) } }
    }

    fn init_hw(&mut self) -> bool {
        self.m_serial_port.init();
        true
    }
}
