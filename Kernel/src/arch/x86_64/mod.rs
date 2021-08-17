/*! x86_64 implementation code */

pub mod acpi_manager;
pub mod addr;
pub mod desc_table;
pub mod dev;
pub mod global_desc_table;
pub mod hw_boot_info;
pub mod hw_cpu_core;
pub mod interrupts;
pub mod ms_register;
pub mod task_state_segment;
pub mod vm;
pub mod x64_port;

global_asm!(include_str!("kernel_start.S"), options(att_syntax));
