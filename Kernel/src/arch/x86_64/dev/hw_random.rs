/*! x86_64 random support */

use core::{
    arch::x86_64::{
        __cpuid,
        _rdrand16_step,
        _rdrand32_step,
        _rdrand64_step
    },
    hint::spin_loop
};

use crate::dev::random::THwRandom;

/**
 * x86_64 `HwRandomBase` implementation
 */
pub struct HwRandom;

impl THwRandom for HwRandom {
    fn new() -> Self {
        let cpuid_res = unsafe { __cpuid(0x1) };
        if cpuid_res.ecx & (1 << 30) != 0 {
            Self
        } else {
            panic!("CPU doesn't support RDRAND instruction");
        }
    }

    fn generate_u16(&self) -> u16 {
        let mut rdrand_res = 0;
        while unsafe { _rdrand16_step(&mut rdrand_res) } != 1 {
            spin_loop();
        }

        return rdrand_res;
    }

    fn generate_u32(&self) -> u32 {
        let mut rdrand_res = 0;
        while unsafe { _rdrand32_step(&mut rdrand_res) } != 1 {
            spin_loop();
        }

        return rdrand_res;
    }

    fn generate_u64(&self) -> u64 {
        let mut rdrand_res = 0;
        while unsafe { _rdrand64_step(&mut rdrand_res) } != 1 {
            spin_loop();
        }

        return rdrand_res;
    }
}
