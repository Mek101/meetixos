/*! Kernel debug utilities */

pub mod display;
pub mod print;

/**
 * Kibibyte multiplier
 */
pub const KIB: usize = 1024;

/**
 * Mebibyte Byte multiplier
 */
pub const MIB: usize = KIB * 1024;

/**
 * Gibibyte Byte multiplier
 */
pub const GIB: usize = MIB * 1024;

/**
 * Tebibyte multiplier
 */
pub const TIB: usize = GIB * 1024;
