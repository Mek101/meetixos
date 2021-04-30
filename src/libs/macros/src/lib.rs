/*! # Macros Library
 *
 * Implements a little collection of utility macros used across the MeetiX
 * system
 */

#![no_std]

pub use bitflags::bitflags;
/* TODO this dependency for now must be imported for each
 *      project that uses it, because it depends on std
 */
pub use num_enum;
pub use paste::paste;

pub mod count_reps;
pub mod enums;
pub mod ext_bitflags;
