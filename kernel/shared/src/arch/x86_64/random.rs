/*! x86_64 random implementation */

use x86_64::instructions::random::RdRand;

use crate::random::HwRandomGeneratorBase;

/**
 * x86_64 `HwRandomGeneratorBase` implementation
 */
pub struct HwRandomGenerator {
    m_inner: RdRand
}

impl HwRandomGeneratorBase for HwRandomGenerator {
    fn new() -> Self {
        if let Some(inner) = RdRand::new() {
            Self { m_inner: inner }
        } else {
            panic!("CPU doesn't support RDRAND instruction")
        }
    }

    fn randomize_u64(&self) -> u64 {
        loop {
            if let Some(res) = self.m_inner.get_u64() {
                return res;
            }
        }
    }

    fn randomize_u32(&self) -> u32 {
        loop {
            if let Some(res) = self.m_inner.get_u32() {
                return res;
            }
        }
    }

    fn randomize_u16(&self) -> u16 {
        loop {
            if let Some(res) = self.m_inner.get_u16() {
                return res;
            }
        }
    }
}
