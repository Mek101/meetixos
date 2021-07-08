/*! x86_64 Global Descriptor Table */

use core::mem::size_of;

use bits::{
    fields::BitFields,
    flags::{
        BitFlags,
        BitFlagsValues
    }
};

use crate::{
    addr::virt::VirtAddr,
    arch::x86_64::descs::{
        tss::TaskStateSegment,
        CpuRingMode,
        DescTablePtr
    }
};

pub struct GlobalDescTable {
    m_desc_table: [usize; 8],
    m_next_free_desc: usize
}

impl GlobalDescTable {
    pub const fn new() -> Self {
        Self { m_desc_table: [0; 8],
               m_next_free_desc: 0 }
    }

    pub fn add_entry(&mut self, entry_descriptor: Descriptor) -> SegmentSelector {
        let insert_index = match entry_descriptor {
            Descriptor::UserSegment(raw_value) => self.push(raw_value),
            Descriptor::SystemSegment(raw_low_value, raw_high_value) => {
                let index = self.push(raw_low_value);
                self.push(raw_high_value);

                index
            }
        };

        let cpu_ring_mode = match entry_descriptor {
            Descriptor::UserSegment(raw_value) => {
                let descr_flags = DescriptorFlags::from(raw_value);

                if descr_flags.is_enabled(DescriptorFlagsBits::DplRing3) {
                    CpuRingMode::Ring3
                } else {
                    CpuRingMode::Ring0
                }
            },
            Descriptor::SystemSegment(..) => CpuRingMode::Ring0
        };

        SegmentSelector::new(insert_index as u16, cpu_ring_mode)
    }

    pub fn load(&'static self) {
        unsafe {
            asm!("lgdt [{}]",
                 in(reg) &self.table_ptr(),
                 options(readonly, nostack, preserves_flags));
        }
    }

    pub fn table_ptr(&self) -> DescTablePtr {
        DescTablePtr::new(insert_index as u16, VirtAddr::from(self.m_desc_table.as_ptr()))
    }

    fn push(&mut self, raw_value: usize) -> usize {
        if self.m_next_free_desc < self.m_desc_table.len() {
            let index = self.m_next_free_desc;

            self.m_desc_table[index] = raw_value;
            self.m_next_free_desc += 1;

            index
        } else {
            panic!("GlobalDescTable full")
        }
    }
}

pub struct SegmentSelector {
    m_raw_value: u16
}

impl SegmentSelector {
    pub const fn new(array_index: u16, cpu_ring_mode: CpuRingMode) -> Self {
        Self { m_raw_value: array_index << 3 | cpu_ring_mode as u16 }
    }

    pub fn array_index(&self) -> u16 {
        self.m_raw_value >> 3
    }

    pub fn cpu_ring_mode(&self) -> CpuRingMode {
        CpuRingMode::from(self.m_raw_value & 0b11)
    }

    pub fn set_cpu_ring_mode(&mut self, cpu_ring_mode: CpuRingMode) {
        self.m_raw_value.set_bits(0..2, cpu_ring_mode as u16);
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Descriptor {
    UserSegment(usize),
    SystemSegment(usize, usize)
}

impl Descriptor {
    pub fn kernel_code_segment() -> Descriptor {
        let kern_code_flags = DescriptorFlagsBits::common()
                              | DescriptorFlagsBits::Executable
                              | DescriptorFlagsBits::LongMode;

        Self::UserSegment(kern_code_flags.raw_bits())
    }

    pub fn kernel_data_segment() -> Descriptor {
        let kern_data_flags =
            DescriptorFlagsBits::common() | DescriptorFlagsBits::DefaultSize;

        Self::UserSegment(kern_data_flags.raw_bits())
    }

    pub fn user_code_segment() -> Descriptor {
        let user_code_flags = DescriptorFlagsBits::common()
                              | DescriptorFlagsBits::Executable
                              | DescriptorFlagsBits::LongMode
                              | DescriptorFlagsBits::DplRing3;

        Self::UserSegment(user_code_flags.raw_bits())
    }

    pub fn user_data_segment() -> Descriptor {
        let user_data_flags = DescriptorFlagsBits::common()
                              | DescriptorFlagsBits::DefaultSize
                              | DescriptorFlagsBits::DplRing3;

        Self::UserSegment(user_data_flags.raw_bits())
    }

    pub fn tss_segment(task_state_segment: &'static TaskStateSegment) -> Descriptor {
        let task_state_segment_ptr = task_state_segment as *const _ as usize;

        let mut raw_low_bits = {
            let bit_flags = DescriptorFlags::new_zero() | DescriptorFlagsBits::Present;

            bit_flags.raw_bits()
        };
        raw_low_bits.set_bits(0..16, size_of::<TaskStateSegment>() - 1);
        raw_low_bits.set_bits(16..40, task_state_segment_ptr.bits_at(0..24));
        raw_low_bits.set_bits(40..44, 0b1001);
        raw_low_bits.set_bits(56..64, task_state_segment_ptr.bits_at(24..32));

        let mut raw_high_bits = 0;
        raw_high_bits.set_bits(0..32, task_state_segment_ptr.bits_at(32..64));

        Self::SystemSegment(raw_low_bits, raw_high_bits)
    }
}

pub type DescriptorFlags = BitFlags<usize, DescriptorFlagsBits>;

pub enum DescriptorFlagsBits {
    /* ignored bits LIMIT 0..=15 */
    Bit0        = 0,
    Bit1        = 1,
    Bit2        = 2,
    Bit3        = 3,
    Bit4        = 4,
    Bit5        = 5,
    Bit6        = 6,
    Bit7        = 7,
    Bit8        = 8,
    Bit9        = 9,
    Bit10       = 10,
    Bit11       = 11,
    Bit12       = 12,
    Bit13       = 13,
    Bit14       = 14,
    Bit15       = 15,

    /* ignored bits LIMIT 16..=19 */
    Bit16       = 16,
    Bit17       = 17,
    Bit18       = 18,
    Bit19       = 19,

    /* significant bits */
    Accessed    = 40,
    Writeable   = 41,
    Conforming  = 42,
    Executable  = 43,
    UserSegment = 44,
    DplRing3    = 45,
    Present     = 47,
    LongMode    = 53,
    DefaultSize = 54,
    Granularity = 55
}

impl DescriptorFlagsBits {
    fn common() -> DescriptorFlags {
        *DescriptorFlags::new_zero()
        | DescriptorFlagsBits::Bit0
        | DescriptorFlagsBits::Bit1
        | DescriptorFlagsBits::Bit2
        | DescriptorFlagsBits::Bit3
        | DescriptorFlagsBits::Bit4
        | DescriptorFlagsBits::Bit5
        | DescriptorFlagsBits::Bit6
        | DescriptorFlagsBits::Bit7
        | DescriptorFlagsBits::Bit8
        | DescriptorFlagsBits::Bit9
        | DescriptorFlagsBits::Bit10
        | DescriptorFlagsBits::Bit11
        | DescriptorFlagsBits::Bit12
        | DescriptorFlagsBits::Bit13
        | DescriptorFlagsBits::Bit14
        | DescriptorFlagsBits::Bit15
        | DescriptorFlagsBits::Bit16
        | DescriptorFlagsBits::Bit17
        | DescriptorFlagsBits::Bit18
        | DescriptorFlagsBits::Bit19
        | DescriptorFlagsBits::Accessed
        | DescriptorFlagsBits::Writeable
        | DescriptorFlagsBits::UserSegment
        | DescriptorFlagsBits::Present
        | DescriptorFlagsBits::Granularity
    }
}

impl BitFlagsValues for DescriptorFlagsBits {
    /* No methods to implement */
}
