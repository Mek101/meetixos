/*! x86_64 implementation code */

pub mod addr;
pub mod desc_table;
pub mod dev;
pub mod gdt;
pub mod hw_boot_info;
pub mod hw_cpu;
pub mod idt;
pub mod tss;
pub mod vm;

global_asm!(include_str!("kernel_start.S"), options(att_syntax));
