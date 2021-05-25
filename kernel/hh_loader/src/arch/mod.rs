#[cfg(target_arch = "aarch64")]
pub use aarch64::*;
#[cfg(target_arch = "riscv")]
pub use riscv::*;

#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "riscv")]
mod riscv;
#[cfg(target_arch = "x86_64")]
mod x86_64;
