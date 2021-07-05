/*! x86_64 random implementation */

use core::arch::x86_64::{
    _rdrand16_step,
    _rdrand32_step,
    _rdrand64_step
};

use crate::random::HwRandomGeneratorBase;

/**
 * x86_64 `HwRandomGeneratorBase` implementation
 */
#[derive(Debug, Copy, Clone)]
pub struct HwRandomGenerator {
    m_inner: ()
}

impl HwRandomGeneratorBase for HwRandomGenerator {
    fn new() -> Self {
        let cpuid_res = unsafe { core::arch::x86_64::__cpuid(0x1) };
        if cpuid_res.ecx & (1 << 30) != 0 {
            Self { m_inner: () }
        } else {
            panic!("CPU doesn't support RDRAND instruction");
        }
    }

    fn randomize_u64(&self) -> u64 {
        let mut rdrand_res = 0;
        while unsafe { _rdrand64_step(&mut rdrand_res) } != 1 {
            core::hint::spin_loop();
        }

        return rdrand_res;
    }

    fn randomize_u32(&self) -> u32 {
        let mut rdrand_res = 0;
        while unsafe { _rdrand32_step(&mut rdrand_res) } != 1 {
            core::hint::spin_loop();
        }

        return rdrand_res;
    }

    fn randomize_u16(&self) -> u16 {
        let mut rdrand_res = 0;
        while unsafe { _rdrand16_step(&mut rdrand_res) } != 1 {
            core::hint::spin_loop();
        }

        return rdrand_res;
    }
}
