/*! x86_64 implementation code */

pub mod acpi_manager;
pub mod addr;
pub mod desc_table;
pub mod dev;
pub mod global_desc_table;
pub mod hw_boot_info;
pub mod hw_cpu_core;
pub mod interrupts;
pub mod io_port;
pub mod ms_register;
pub mod pit;
pub mod task;
pub mod vm;

global_asm!(include_str!("kernel_start.S"), options(att_syntax));
