/*! # MeetiX Userspace Runtime Library
 *
 * Implements the main entry point, the panic manager and the heap
 * management
 */

#![no_std]
#![feature(lang_items, alloc_error_handler)]

extern crate alloc;

pub use alloc::*;

pub mod heap;
pub mod panic;
pub mod start;
