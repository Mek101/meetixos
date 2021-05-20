/*! x86_64 implementation code */

pub mod info;
pub mod loader;

global_asm!(include_str!("loader_start.S"), options(att_syntax));
