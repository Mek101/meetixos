/*! x86_64 low level CPU descriptors */

use crate::addr::virt_addr::VirtAddr;

/**
 * x86_64 segmentation descriptor pointer
 */
#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone)]
pub struct DescTablePtr {
    m_size_limit: u16,
    m_base_ptr: VirtAddr
}

impl DescTablePtr /* Constructors */ {
    /**
     * Constructs a `DescTablePtr` from the given values
     */
    pub fn new(limit: u16, base: VirtAddr) -> Self {
        Self { m_size_limit: limit,
               m_base_ptr: base }
    }
}

/**
 * Lists the x86_64 CPU ring modes
 */
#[repr(u8)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
pub enum CpuRingMode {
    /* used for kernel mode */
    Ring0,
    /* unused */
    Ring1,
    /* unused */
    Ring2,
    /* used for user mode */
    Ring3
}

impl From<u16> for CpuRingMode {
    fn from(raw_value: u16) -> Self {
        match raw_value {
            0 => Self::Ring0,
            1 => Self::Ring1,
            2 => Self::Ring2,
            3 => Self::Ring3,
            _ => panic!("CpuRingMode::from(): Invalid ring value given")
        }
    }
}
