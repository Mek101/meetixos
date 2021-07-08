/*! x86_64 low level CPU descriptors */

use crate::addr::virt::VirtAddr;

pub mod gdt;
pub mod tss;

#[repr(C, packed)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct DescTablePtr {
    m_size_limit: u16,
    m_base_ptr: VirtAddr
}

impl DescTablePtr {
    pub fn new(limit: u16, base: VirtAddr) -> Self {
        Self { m_size_limit: limit,
               m_base_ptr: base }
    }

    pub fn as_ptr(&self) -> usize {
        self as *const Self as *const usize as usize
    }
}

#[repr(u8)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
pub enum CpuRingMode {
    Ring0,
    Ring1,
    Ring2,
    Ring3
}

impl From<u16> for CpuRingMode {
    fn from(raw_value: u16) -> Self {
        match raw_value {
            0 => Self::Ring0,
            1 => Self::Ring1,
            2 => Self::Ring2,
            3 => Self::Ring3,
            _ => panic!("CpuRingMode: Invalid ring")
        }
    }
}
