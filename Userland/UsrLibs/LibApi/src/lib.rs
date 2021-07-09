/*! # Kernel API library
 *
 * Implements an object oriented interface to interact with the MeetiX's
 * Kernel
 */

#![no_std]
#![feature(asm, in_band_lifetimes, min_specialization)]

extern crate alloc;

pub mod arch;
pub mod config_mode;
pub mod entity;
pub mod instant;
pub mod kern_handle;
pub mod object;
pub mod path;
pub mod task;
