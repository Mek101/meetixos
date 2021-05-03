/*! # Architecture Dependent Switch
 *
 * This module exports at (and only at) the crate level the architecture
 * dependent code, which internally exposes well defined name for the upside
 * layer.
 *
 * Many of these data structures/functions are exported as is, the others
 * are wrapped by an HAL structure/function in the upper level
 */

/* self:: is used to avoid crate name ambiguity */
#[cfg(target_arch = "aarch64")]
pub(crate) use self::aarch64::*;
#[cfg(target_arch = "riscv")]
pub(crate) use self::riscv::*;
#[cfg(target_arch = "x86_64")]
pub(crate) use self::x86_64::*;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "riscv")]
mod riscv;
#[cfg(target_arch = "x86_64")]
mod x86_64;
