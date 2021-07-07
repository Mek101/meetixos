/*! Kernel debug utilities */

pub mod display;
pub mod print;

/**
 * Kibibyte multiplier
 */
pub const C_KIB: usize = 1024;

/**
 * Mebibyte Byte multiplier
 */
pub const C_MIB: usize = C_KIB * 1024;

/**
 * Gibibyte Byte multiplier
 */
pub const C_GIB: usize = C_MIB * 1024;

/**
 * Tebibyte multiplier
 */
pub const C_TIB: usize = C_GIB * 1024;
