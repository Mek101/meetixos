/*! # Low Level OS Library
 *
 * Implements the architecture dependent code to perform kernel calls, the
 * OS limits and the identifiers for the system call classes and routines.
 *
 * Refer to the `api` crate to right use the kernel's system call
 * interface, this crate is not intended for direct uses
 */

#![no_std]
#![feature(asm)]

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64.rs"]
pub mod arch;
#[cfg(target_arch = "riscv")]
#[path = "arch/riscv.rs"]
pub mod arch;
#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64.rs"]
pub mod arch;

pub mod limits;
pub mod str_utils;
pub mod sysc;
