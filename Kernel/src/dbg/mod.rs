/*! Kernel debug utilities */

pub mod display_pretty;
pub mod print;

/**
 * Kibibyte multiplier
 */
pub const C_KIB: usize = 1024;

/**
 * Mebibyte Byte multiplier
 */
pub const C_MIB: usize = C_KIB * C_KIB;

/**
 * Gibibyte Byte multiplier
 */
pub const C_GIB: usize = C_MIB * C_KIB;

/**
 * Tebibyte multiplier
 */
pub const C_TIB: usize = C_GIB * C_KIB;
