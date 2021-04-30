/*! # Low Level OS Library
 *
 * Implements the architecture dependent code to perform kernel calls, the
 * OS limits and the identifiers for the system call classes and routines.
 *
 * System calls returns a [`Result`] variant with a raw `usize` value both
 * for [`Ok`] and [`Err`]. Those values must be interpreted by the upper
 * wrappers that are not implemented into this library.
 *
 * Refer to the [`api`] crate to right use the kernel's system call
 * interface, this crate is not intended for direct uses
 *
 * [`Result`]: core::result::Result
 * [`Ok`]: core::result::Result::Ok
 * [`Err`]: core::result::Result::Err
 * [`api`]: /api/index.html
 */

#![no_std]
#![feature(asm)]

#[macro_use]
extern crate macros;

pub use arch::*;

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64.rs"]
mod arch;
#[cfg(target_arch = "riscv")]
#[path = "arch/riscv.rs"]
mod arch;
#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64.rs"]
mod arch;

pub mod limits;
pub mod str_utils;
pub mod sysc;
