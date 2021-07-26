/*! x86_64 device drivers implementations */

use crate::{
    arch::x86_64::dev::{
        hw_random::X64RdRandRandom,
        hw_uart::X64Serial16550Uart
    },
    dev::DevManager
};

pub mod hw_random;
pub mod hw_uart;
pub mod io_port;

impl DevManager /* Methods */ {
    pub fn register_early_devices(&self) {
        /* register RDRAND device */
        assert!(self.register_device(X64RdRandRandom::new(0)),
                "Failed to register RDRAND driver");

        /* register COM1 serial device */
        assert!(self.register_device(X64Serial16550Uart::new_com1()),
                "Failed to register Serial COM1 driver");

        /* register COM2 serial device */
        assert!(self.register_device(X64Serial16550Uart::new_com2()),
                "Failed to register Serial COM2 driver");

        /* register COM3 serial device */
        assert!(self.register_device(X64Serial16550Uart::new_com3()),
                "Failed to register Serial COM3 driver");

        /* register COM4 serial device */
        assert!(self.register_device(X64Serial16550Uart::new_com4()),
                "Failed to register Serial COM4 driver");
    }
}
