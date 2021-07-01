/*! # MeetiX Userspace Runtime Library
 *
 * Implements the main entry point, the panic manager and the heap
 * management
 */

#![no_std]

extern crate alloc;

pub use alloc::*;

pub mod heap;
pub mod panic;
pub mod start;
