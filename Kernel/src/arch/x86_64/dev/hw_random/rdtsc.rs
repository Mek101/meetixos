/*! RDTSC random driver */

use alloc::string::String;
use core::arch::x86_64::_rdtsc;

use api_data::object::device::{
    DeviceId,
    DeviceIdClass,
    DeviceIdType
};

use crate::dev::{
    random::TRandomDevice,
    TDevice
};

/**
 * x86_64 `TRandomDevice` implementation, uses RDTSC support
 */
pub struct X64RdTscRandom {
    m_device_id: DeviceId
}

impl X64RdTscRandom /* Constructors */ {
    /**
     * Constructs an uninitialized `X64RdTscRandom`
     */
    pub const fn new(serial_value: u32) -> Self {
        Self { m_device_id: DeviceId::new(DeviceIdType::Character,
                                          DeviceIdClass::Random,
                                          serial_value) }
    }
}

impl TDevice for X64RdTscRandom {
    fn device_id(&self) -> DeviceId {
        self.m_device_id
    }

    fn device_name(&self) -> String {
        format!("random_{}", self.m_device_id.serial_value())
    }

    fn init_hw(&self) -> bool {
        true
    }

    fn as_random(&self) -> Option<&dyn TRandomDevice> {
        Some(self)
    }
}

impl TRandomDevice for X64RdTscRandom {
    fn random_u16(&self) -> u16 {
        unsafe { _rdtsc() as u16 }
    }

    fn random_u32(&self) -> u32 {
        unsafe { _rdtsc() as u32 }
    }

    fn random_u64(&self) -> u64 {
        unsafe { _rdtsc() as u64 }
    }
}
