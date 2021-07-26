/*! x86_64 random support */

use alloc::string::String;
use core::{
    arch::x86_64::{
        __cpuid,
        _rdrand16_step,
        _rdrand32_step,
        _rdrand64_step
    },
    hint::spin_loop
};

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
 * x86_64 `TRandomDevice` implementation, uses RDRAND support
 */
pub struct X64RdRandRandom {
    m_device_id: DeviceId
}

impl X64RdRandRandom /* Constructors */ {
    /**
     * Constructs an uninitialized `X64RdRandRandom`
     */
    pub const fn new(serial_value: u32) -> Self {
        Self { m_device_id: DeviceId::new(DeviceIdType::Character,
                                          DeviceIdClass::Random,
                                          serial_value) }
    }
}

impl TDevice for X64RdRandRandom {
    fn device_id(&self) -> DeviceId {
        self.m_device_id
    }

    fn device_name(&self) -> String {
        format!("random_{}", self.m_device_id.serial_value())
    }

    fn init_hw(&self) -> bool {
        /* returns whether the CPU supports the RDRAND instruction */
        unsafe { __cpuid(0x1) }.ecx & (1 << 30) != 0
    }

    fn as_random(&self) -> Option<&dyn TRandomDevice> {
        Some(self)
    }
}

impl TRandomDevice for X64RdRandRandom {
    fn random_u16(&self) -> u16 {
        let mut rdrand_res = 0;
        while unsafe { _rdrand16_step(&mut rdrand_res) } != 1 {
            spin_loop();
        }

        return rdrand_res;
    }

    fn random_u32(&self) -> u32 {
        let mut rdrand_res = 0;
        while unsafe { _rdrand32_step(&mut rdrand_res) } != 1 {
            spin_loop();
        }

        return rdrand_res;
    }

    fn random_u64(&self) -> u64 {
        let mut rdrand_res = 0;
        while unsafe { _rdrand64_step(&mut rdrand_res) } != 1 {
            spin_loop();
        }

        return rdrand_res;
    }
}
