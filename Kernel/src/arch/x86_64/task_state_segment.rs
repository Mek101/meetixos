/*! x86_64 Task State Segment */

use crate::addr::{
    virt_addr::VirtAddr,
    TAddress
};

/**
 * x86_64 stack state segment descriptor
 */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TaskStateSegment {
    _reserved_field_1: u32,
    pub m_stacks_per_privilege: [VirtAddr; 3],
    _reserved_field_2: u32,
    pub m_full_intr_stack_table: [VirtAddr; 7],
    _reserved_field_3: u32,
    _reserved_field_4: u32,
    _reserved_field_5: u16,
    pub m_io_map_base: u16
}

impl TaskStateSegment /* Constructors */ {
    /**
     * Constructs an empty `TaskStateSegment`
     */
    pub fn new() -> Self {
        Self { _reserved_field_1: 0,
               m_stacks_per_privilege: [VirtAddr::null(); 3],
               _reserved_field_2: 0,
               m_full_intr_stack_table: [VirtAddr::null(); 7],
               _reserved_field_3: 0,
               _reserved_field_4: 0,
               _reserved_field_5: 0,
               m_io_map_base: 0 }
    }
}
