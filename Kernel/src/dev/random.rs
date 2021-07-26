/*! Kernel random support */

use crate::{
    arch::dev::hw_random::HwRandom,
    dev::TDevice
};

pub trait TRandomDevice: TDevice {
    fn random_u16(&self) -> u16;
    fn random_u32(&self) -> u32;
    fn random_u64(&self) -> u64;
}

pub struct Random {
    m_hw_random: HwRandom
}

impl Random /* Constructors */ {
    pub fn new() -> Self {
        Self { m_hw_random: HwRandom::new() }
    }
}

impl Random /* Methods */ {
    /**
     * Generates a new `u16` value
     */
    pub fn generate_u16(&self) -> u16 {
        self.m_hw_random.generate_u16()
    }

    /**
     * Generates a new `u32` value
     */
    pub fn generate_u32(&self) -> u32 {
        self.m_hw_random.generate_u32()
    }

    /**
     * Generates a new `u64` value
     */
    pub fn generate_u64(&self) -> u64 {
        self.m_hw_random.generate_u64()
    }
}

/**
 * Interface on which `Random` relies to perform random number generation
 * using hardware acceleration
 */
pub trait THwRandom {
    /**
     * Constructs an `HwRandom`
     */
    fn new() -> Self;

    /**
     * Generates a new `u16` value
     */
    fn generate_u16(&self) -> u16;

    /**
     * Generates a new `u32` value
     */
    fn generate_u32(&self) -> u32;

    /**
     * Generates a new `u64` value
     */
    fn generate_u64(&self) -> u64;
}
