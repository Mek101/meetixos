/*! x86_64 implementation code */

pub mod addr;
pub mod dev;

global_asm!(include_str!("kernel_start.S"), options(att_syntax));
