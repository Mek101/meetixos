/*! Hardware randomization support */

use crate::arch::random::HwRandomGenerator;

/**
 * Random number generator which relies on hardware support to generate
 * numbers of various sizes
 */
pub struct Random {
    m_hw_random: HwRandomGenerator
}

impl Random {
    /**
     * Constructs a `Random`
     */
    pub fn new() -> Self {
        Self { m_hw_random: HwRandomGenerator::new() }
    }

    /**
     * Returns an `usize` random generated number
     */
    pub fn randomize_usize(&self) -> usize {
        self.m_hw_random.randomize_u64() as usize
    }

    /**
     * Returns an `u64` random generated number
     */
    pub fn randomize_u64(&self) -> u64 {
        self.m_hw_random.randomize_u64()
    }

    /**
     * Returns an `u32` random generated number
     */
    pub fn randomize_u32(&self) -> u32 {
        self.m_hw_random.randomize_u32()
    }

    /**
     * Returns an `u16` random generated number
     */
    pub fn randomize_u16(&self) -> u16 {
        self.m_hw_random.randomize_u16()
    }

    /**
     * Returns a `bool` random generated value
     */
    pub fn randomize_bool(&self) -> bool {
        self.m_hw_random.randomize_u16() % 3 == 0
    }
}

/**
 * Interface on which `Random` relies to access native random instructions
 */
pub(crate) trait HwRandomGeneratorBase {
    /*
     * Constructs a `HwRandomGeneratorBase` based object
     */
    fn new() -> Self;

    /**
     * Returns an `u64` random generated number
     */
    fn randomize_u64(&self) -> u64;

    /**
     * Returns an `u32` random generated number
     */
    fn randomize_u32(&self) -> u32 {
        self.randomize_u64() as u32
    }

    /**
     * Returns an `u16` random generated number
     */
    fn randomize_u16(&self) -> u16 {
        self.randomize_u64() as u16
    }
}
