/*! Universal asynchronous receiver-transmitter */

use core::fmt;

use crate::dev::TDevice;

pub trait TUartDevice: TDevice {
    fn write_str(&self, str: &str) -> fmt::Result;
}
