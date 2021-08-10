/*! x86_64 implementation code */

pub mod acpi_manager;
pub mod addr;
pub mod apic_manager;
pub mod desc_table;
pub mod dev;
pub mod global_desc_table;
pub mod hw_boot_info;
pub mod hw_cpu;
pub mod intr_desc_table;
pub mod intr_handler;
pub mod intr_stack_frame;
pub mod ms_register;
pub mod tss;
pub mod vm;
pub mod x64_port;

global_asm!(include_str!("kernel_start.S"), options(att_syntax));
global_asm!(include_str!("intr_service_routines.S"), options(att_syntax));
