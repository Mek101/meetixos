/*! x86_64 Task State Segment */

use crate::addr::{
    virt::VirtAddr,
    Address
};

/**
 * x86_64 stack state segment descriptor
 */
#[repr(C, packed)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct TaskStateSegment {
    _reserved_field_1: u32,
    m_stacks_per_privilege: [VirtAddr; 3],
    _reserved_field_2: u32,
    m_full_intr_stack_table: [VirtAddr; 7],
    _reserved_field_3: u32,
    _reserved_field_4: u32,
    _reserved_field_5: u16,
    m_io_map_base: u16
}

impl TaskStateSegment {
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

    /**
     * Returns the slice of 64bit canonical stack pointers for privilege
     * levels from 0 to 2
     */
    pub fn stacks_per_privilege(&self) -> &[VirtAddr] {
        self.m_stacks_per_privilege.as_slice()
    }

    /**
     * Returns the mutable slice of 64bit canonical stack pointers for
     * privilege levels from 0 to 2
     */
    pub fn stack_per_privilege_mut(&mut self) -> &mut [VirtAddr] {
        self.m_stacks_per_privilege.as_mut_slice()
    }

    /**
     * Returns the slice of 64bit canonical interrupt stack pointers
     */
    pub fn full_intr_stack_table(&self) -> &[VirtAddr] {
        self.m_full_intr_stack_table.as_slice()
    }

    /**
     * Returns the mutable slice of 64bit canonical interrupt stack pointers
     */
    pub fn full_intr_stack_table_mut(&mut self) -> &mut [VirtAddr] {
        self.m_full_intr_stack_table.as_mut_slice()
    }

    /**
     * Returns the base for the x86 I/O map
     */
    pub fn io_map_base(&self) -> u16 {
        self.m_io_map_base
    }
}
