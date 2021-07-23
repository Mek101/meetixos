/*! x86_64 UART implementation */

use core::fmt;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use bits::bit_flags::{
    BitFlags,
    BitFlagsValues
};

use crate::{
    arch::x86_64::dev::io_port::IOPort,
    dev::uart::HwUartBase
};

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
    m_data: IOPort<u8>,
    m_intr_enabled: IOPort<u8>,
    m_fifo_ctrl: IOPort<u8>,
    m_line_ctrl: IOPort<u8>,
    m_modem_ctrl: IOPort<u8>,
    m_line_status: IOPort<u8>
}

impl HwUart /* Privates */ {
    fn line_status(&self) -> BitFlags<u8, LineStatusBits> {
        let raw_line_status_value = unsafe { self.m_line_status.read() };

        BitFlags::from_raw_truncate(raw_line_status_value)
    }

    fn wait_for_empty(&self) {
        while self.line_status().is_disabled(LineStatusBits::OutputEmpty) {
            core::hint::spin_loop();
        }
    }

    fn send(&mut self, byte_to_send: u8) {
        unsafe {
            match byte_to_send {
                8 | 0x7F => {
                    self.wait_for_empty();
                    self.m_data.write(8);
                    self.wait_for_empty();
                    self.m_data.write(b' ');
                    self.wait_for_empty();
                    self.m_data.write(8);
                },
                _ => {
                    self.wait_for_empty();
                    self.m_data.write(byte_to_send);
                }
            }
        }
    }
}

impl HwUartBase for HwUart {
    const CONST_NEW: Self =
        Self { m_data: IOPort::new(SERIAL_COM1_PORT_BASE),
               m_intr_enabled: IOPort::new(SERIAL_COM1_PORT_BASE + 1),
               m_fifo_ctrl: IOPort::new(SERIAL_COM1_PORT_BASE + 2),
               m_line_ctrl: IOPort::new(SERIAL_COM1_PORT_BASE + 3),
               m_modem_ctrl: IOPort::new(SERIAL_COM1_PORT_BASE + 4),
               m_line_status: IOPort::new(SERIAL_COM1_PORT_BASE + 5) };

    fn init_hw(&mut self) -> bool {
        unsafe {
            /* disable interrupts */
            self.m_intr_enabled.write(0x00);

            /* enable DLAB */
            self.m_line_ctrl.write(0x80);

            /* set maximum speed to 38400 bps by configuring DLL and DLM */
            self.m_data.write(0x03);
            self.m_intr_enabled.write(0x00);

            /* disable DLAB and set data word length to 8 bits */
            self.m_line_ctrl.write(0x03);

            /* enable FIFO, clear TX/RX queues and set interrupt watermark at 14 bytes */
            self.m_fifo_ctrl.write(0xC7);

            /* mark data terminal ready, signal request to send and enable auxiliary
             * output #2 (used as interrupt line for CPU)
             */
            self.m_modem_ctrl.write(0x0B);

            /* enable interrupts */
            self.m_intr_enabled.write(0x01);
        }
        true
    }
}

impl fmt::Write for HwUart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte_to_send in s.bytes() {
            self.send(byte_to_send);
        }
        Ok(())
    }
}

#[repr(usize)]
#[derive(Copy, Clone)]
#[derive(IntoPrimitive, TryFromPrimitive)]
enum IntrEnabledBits {
    Received,
    Sent,
    Errored,
    StatusChange
}

impl BitFlagsValues for IntrEnabledBits {
}

#[repr(usize)]
#[derive(Copy, Clone)]
#[derive(IntoPrimitive, TryFromPrimitive)]
enum LineStatusBits {
    InputFull,
    OutputEmpty = 5
}

impl BitFlagsValues for LineStatusBits {
}
