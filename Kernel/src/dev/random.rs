/*! Kernel random support */

use crate::dev::TDevice;

/**
 * Hardware accelerated random number generator device driver interface
 */
pub trait TRandomDevice: TDevice {
    /**
     * Generates a random `u16` value
     */
    fn random_u16(&self) -> u16;

    /**
     * Generates a random `u32` value
     */
    fn random_u32(&self) -> u32;

    /**
     * Generates a random `u64` value
     */
    fn random_u64(&self) -> u64;
}
