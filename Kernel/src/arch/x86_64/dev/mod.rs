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
        let rdrand_device_driver =
            X64RdRandRandom::try_new(0).expect("Unsupported RDRAND instruction");
        let serial_com1_device_driver = X64Serial16550Uart::new(0x3f8);
        let serial_com2_device_driver = X64Serial16550Uart::new(0x2e8);
        let serial_com3_device_driver = X64Serial16550Uart::new(0x2f8);
        let serial_com4_device_driver = X64Serial16550Uart::new(0x3e8);

        assert!(self.register_device(rdrand_device_driver),
                "Failed to register RDRAND driver");
        assert!(self.register_device(serial_com1_device_driver),
                "Failed to register Serial COM1 driver");
        assert!(self.register_device(serial_com2_device_driver),
                "Failed to register Serial COM2 driver");
        assert!(self.register_device(serial_com3_device_driver),
                "Failed to register Serial COM3 driver");
        assert!(self.register_device(serial_com4_device_driver),
                "Failed to register Serial COM4 driver");
    }
}
