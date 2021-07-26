/*! Kernel random support */

use crate::dev::TDevice;

pub trait TRandomDevice: TDevice {
    fn random_u16(&self) -> u16;
    fn random_u32(&self) -> u32;
    fn random_u64(&self) -> u64;
}
