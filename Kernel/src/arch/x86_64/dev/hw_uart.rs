/*! x86_64 UART implementation */

use alloc::string::String;
use core::{
    fmt,
    hint::spin_loop
};

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use api_data::object::device::{
    DeviceId,
    DeviceIdClass,
    DeviceIdType
};
use bits::bit_flags::{
    BitFlags,
    TBitFlagsValues
};
use sync::SpinMutex;

use crate::{
    arch::x86_64::dev::io_port::IOPort,
    dev::{
        uart::TUartDevice,
        TDevice
    }
};

/**
 * x86_64 `TUartDevice` implementation based on Serial16550 hardware
 */
pub struct X64Serial16550Uart {
    m_device_id: DeviceId,
    m_data: IOPort<u8>,
    m_intr_enabled: IOPort<u8>,
    m_fifo_ctrl: IOPort<u8>,
    m_line_ctrl: IOPort<u8>,
    m_modem_ctrl: IOPort<u8>,
    m_line_status: IOPort<u8>,
    m_writer: SpinMutex<X64Serial16550UartWriter>
}

impl X64Serial16550Uart /* Constructors */ {
    /**
     * Constructs an uninitialized `X64Serial16550Uart` which uses the COM1
     * port
     */
    pub fn new_com1() -> Self {
        Self::new(0x3f8, 1)
    }

    /**
     * Constructs an uninitialized `X64Serial16550Uart` which uses the COM2
     * port
     */
    pub fn new_com2() -> Self {
        Self::new(0x2e8, 2)
    }

    /**
     * Constructs an uninitialized `X64Serial16550Uart` which uses the COM3
     * port
     */
    pub fn new_com3() -> Self {
        Self::new(0x2f8, 3)
    }

    /**
     * Constructs an uninitialized `X64Serial16550Uart` which uses the COM4
     * port
     */
    pub fn new_com4() -> Self {
        Self::new(0x3e8, 4)
    }

    /**
     * Constructs an uninitialized `X64Serial16550Uart` with the given
     * parameters
     */
    fn new(serial_base: u16, com_port_number: u32) -> Self {
        Self { m_device_id: DeviceId::new(DeviceIdType::Character,
                                          DeviceIdClass::Uart,
                                          com_port_number),
               m_data: IOPort::new(serial_base),
               m_intr_enabled: IOPort::new(serial_base + 1),
               m_fifo_ctrl: IOPort::new(serial_base + 2),
               m_line_ctrl: IOPort::new(serial_base + 3),
               m_modem_ctrl: IOPort::new(serial_base + 4),
               m_line_status: IOPort::new(serial_base + 5),
               m_writer: SpinMutex::const_new(X64Serial16550UartWriter) }
    }
}

impl X64Serial16550Uart /* Privates */ {
    /**
     * Returns the line-status of the serial port
     */
    fn line_status(&self) -> BitFlags<u8, LineStatusBits> {
        let raw_line_status_value = unsafe { self.m_line_status.read() };

        BitFlags::from_raw_truncate(raw_line_status_value)
    }

    /**
     * Waits in spin-loop until the line-status is empty
     */
    fn wait_for_empty(&self) {
        while self.line_status().is_disabled(LineStatusBits::OutputEmpty) {
            spin_loop();
        }
    }

    /**
     * Writes the given byte to the serial port
     */
    fn send(&self, byte_to_send: u8) {
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

impl TDevice for X64Serial16550Uart {
    fn device_id(&self) -> DeviceId {
        self.m_device_id
    }

    fn device_name(&self) -> String {
        format!("com_{}", self.m_device_id.serial_value())
    }

    fn init_hw(&self) -> bool {
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

    fn as_uart(&self) -> Option<&dyn TUartDevice> {
        Some(self)
    }
}

impl TUartDevice for X64Serial16550Uart {
    fn write_str(&self, str: &str) -> fmt::Result {
        self.m_writer.lock().write_str(self, str);
        Ok(())
    }
}

/**
 * Writer proxy used to write thread-safe to the serial port
 */
struct X64Serial16550UartWriter;

impl X64Serial16550UartWriter /* Methods */ {
    /**
     * Writes the given `&str` to the given `X64Serial16550Uart`
     */
    fn write_str(&self, serial_uart: &X64Serial16550Uart, str: &str) {
        for byte_to_send in str.bytes() {
            serial_uart.send(byte_to_send);
        }
    }
}

/**
 * Interrupts flags
 */
#[repr(usize)]
#[derive(Copy, Clone)]
#[derive(IntoPrimitive, TryFromPrimitive)]
enum IntrEnabledBits {
    Received,
    Sent,
    Errored,
    StatusChange
}

impl TBitFlagsValues for IntrEnabledBits {
}

/**
 * Line-status flags
 */
#[repr(usize)]
#[derive(Copy, Clone)]
#[derive(IntoPrimitive, TryFromPrimitive)]
enum LineStatusBits {
    InputFull,
    OutputEmpty = 5
}

impl TBitFlagsValues for LineStatusBits {
}
