/*! # Macros Library
 *
 * Implements a little collection of utility macros used across the MeetiX
 * system
 */

#![no_std]

pub use bitflags::bitflags;
pub use paste::paste;

pub mod count_reps;
pub mod enums;
pub mod ext_bitflags;
