/*! Architecture dependent switch */

#[cfg(target_arch = "aarch64")]
pub(crate) use aarch64::*;
#[cfg(target_arch = "riscv")]
pub(crate) use riscv::*;
#[cfg(target_arch = "x86_64")]
pub(crate) use x86_64::*;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "riscv")]
mod riscv;
#[cfg(target_arch = "x86_64")]
mod x86_64;
