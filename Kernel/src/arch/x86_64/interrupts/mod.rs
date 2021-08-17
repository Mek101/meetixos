/*! x86_64 Interrupts management */

pub mod apic_manager;
pub mod intr_desc_table;
pub mod intr_handler;
pub mod intr_stack_frame;

global_asm!(include_str!("intr_service_routines.S"), options(att_syntax));
