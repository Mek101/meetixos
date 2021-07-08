/*! x86_64 Global Descriptor Table */

use core::mem::size_of;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

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

/**
 * x86_64 GDT
 */
pub struct GlobalDescTable {
    m_desc_table: [usize; 8],
    m_next_free_desc: usize
}

impl GlobalDescTable {
    /**
     * Constructs a `GlobalDescTable` with already the null-segment
     */
    pub const fn new() -> Self {
        Self { m_desc_table: [0; 8],
               m_next_free_desc: 1 }
    }

    /**
     * Appends a `Segment` descriptor
     */
    pub fn add_entry(&mut self, segment: Segment) -> SegmentSelector {
        /* insert the segment inside the table */
        let insert_index = match segment {
            Segment::Common(raw_value) => {
                /* for common segments use a single table entry */
                self.push(raw_value)
            },
            Segment::System(raw_low_value, raw_high_value) => {
                /* for system segments (TSS) use two table entries */
                let first_index = self.push(raw_low_value);
                self.push(raw_high_value);

                first_index
            }
        };

        /* select the <CpuRingMode> */
        let cpu_ring_mode = match segment {
            Segment::Common(raw_value) => {
                let descr_flags = SegmentFlags::from(raw_value);

                /* common segments could be for userland or only for kernel */
                if descr_flags.is_enabled(SegmentFlagsBits::DplRing3) {
                    CpuRingMode::Ring3
                } else {
                    CpuRingMode::Ring0
                }
            },
            Segment::System(..) => {
                /* system segments are only for kernel mode */
                CpuRingMode::Ring0
            }
        };

        /* return the selector */
        SegmentSelector::new(insert_index as u16, cpu_ring_mode)
    }

    /**
     * Loads into the current CPU this GDT
     */
    pub fn load(&'static self) {
        unsafe {
            asm!("lgdt [{}]",
                 in(reg) &self.table_ptr(),
                 options(readonly, nostack, preserves_flags));
        }
    }

    /**
     * Returns the `DescTablePtr` for this GDT
     */
    pub fn table_ptr(&self) -> DescTablePtr {
        DescTablePtr::new((self.m_next_free_desc * size_of::<usize>() - 1) as u16,
                          VirtAddr::from(self.m_desc_table.as_ptr()))
    }

    /**
     * Pushes a new entry inside the table, panics if the limit is reached
     */
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

/**
 * x86_64 segment selector
 */
#[repr(transparent)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct SegmentSelector {
    m_raw_value: u16
}

impl SegmentSelector {
    /**
     * Constructs a `SegmentSelector` from the given parameters
     */
    pub const fn new(array_index: u16, cpu_ring_mode: CpuRingMode) -> Self {
        Self { m_raw_value: array_index << 3 | cpu_ring_mode as u16 }
    }

    /**
     * Returns the index of the table where the CPU must load the `Segment`
     */
    pub fn array_index(&self) -> u16 {
        self.m_raw_value >> 3
    }

    /**
     * Returns the `CpuRingMode` to use for `Segment` loading
     */
    pub fn cpu_ring_mode(&self) -> CpuRingMode {
        CpuRingMode::from(self.m_raw_value & 0b11)
    }

    /**
     * Sets the `CpuRingMode` to load the segment
     */
    pub fn set_cpu_ring_mode(&mut self, cpu_ring_mode: CpuRingMode) {
        self.m_raw_value.set_bits(0..2, cpu_ring_mode as u16);
    }
}

/**
 * x86_64 segmentation descriptor
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Segment {
    Common(usize),
    System(usize, usize)
}

impl Segment {
    /**
     * Returns a `Segment` configured for kernel-code
     */
    pub fn kernel_code_segment() -> Segment {
        let kern_code_flags = SegmentFlagsBits::common()
                              | SegmentFlagsBits::Executable
                              | SegmentFlagsBits::LongMode;

        Self::Common(kern_code_flags.raw_bits())
    }

    /**
     * Returns a `Segment` configured for kernel-data
     */
    pub fn kernel_data_segment() -> Segment {
        let kern_data_flags = SegmentFlagsBits::common() | SegmentFlagsBits::DefaultSize;

        Self::Common(kern_data_flags.raw_bits())
    }

    /**
     * Returns a `Segment` configured for user-code
     */
    pub fn user_code_segment() -> Segment {
        let user_code_flags = SegmentFlagsBits::common()
                              | SegmentFlagsBits::Executable
                              | SegmentFlagsBits::LongMode
                              | SegmentFlagsBits::DplRing3;

        Self::Common(user_code_flags.raw_bits())
    }

    /**
     * Returns a `Segment` configured for user-data
     */
    pub fn user_data_segment() -> Segment {
        let user_data_flags = SegmentFlagsBits::common()
                              | SegmentFlagsBits::DefaultSize
                              | SegmentFlagsBits::DplRing3;

        Self::Common(user_data_flags.raw_bits())
    }

    /**
     * Returns a `Segment` configured for the given `TaskStateSegment`
     */
    pub fn tss_segment(task_state_segment: &'static TaskStateSegment) -> Segment {
        let task_state_segment_ptr = task_state_segment as *const _ as usize;

        let mut raw_low_bits = {
            let bit_flags = SegmentFlags::new_zero() | SegmentFlagsBits::Present;

            bit_flags.raw_bits()
        };
        raw_low_bits.set_bits(0..16, size_of::<TaskStateSegment>() - 1);
        raw_low_bits.set_bits(16..40, task_state_segment_ptr.bits_at(0..24));
        raw_low_bits.set_bits(40..44, 0b1001);
        raw_low_bits.set_bits(56..64, task_state_segment_ptr.bits_at(24..32));

        let mut raw_high_bits = 0;
        raw_high_bits.set_bits(0..32, task_state_segment_ptr.bits_at(32..64));

        Self::System(raw_low_bits, raw_high_bits)
    }
}

/**
 * `Segment` flags `BitFlags`
 */
pub type SegmentFlags = BitFlags<usize, SegmentFlagsBits>;

#[repr(usize)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(TryFromPrimitive, IntoPrimitive)]
pub enum SegmentFlagsBits {
    /* ignored bits LIMIT 0..=15 */
    Limit0      = 0,
    Limit1      = 1,
    Limit2      = 2,
    Limit3      = 3,
    Limit4      = 4,
    Limit5      = 5,
    Limit6      = 6,
    Limit7      = 7,
    Limit8      = 8,
    Limit9      = 9,
    Limit10     = 10,
    Limit11     = 11,
    Limit12     = 12,
    Limit13     = 13,
    Limit14     = 14,
    Limit15     = 15,

    /* significant bits */
    Accessed    = 40,
    Writeable   = 41,
    Conforming  = 42,
    Executable  = 43,
    UserSegment = 44,
    DplRing3    = 45,
    Present     = 47,

    /* ignored bits LIMIT_HI 16..=19 */
    Limit16     = 48,
    Limit17     = 49,
    Limit18     = 50,
    Limit19     = 51,

    LongMode    = 53,
    DefaultSize = 54,
    Granularity = 55
}

impl SegmentFlagsBits {
    /**
     * Returns the common `SegmentFlags`
     */
    fn common() -> SegmentFlags {
        SegmentFlags::new_zero()
        | Self::Limit0
        | Self::Limit1
        | Self::Limit2
        | Self::Limit3
        | Self::Limit4
        | Self::Limit5
        | Self::Limit6
        | Self::Limit7
        | Self::Limit8
        | Self::Limit9
        | Self::Limit10
        | Self::Limit11
        | Self::Limit12
        | Self::Limit13
        | Self::Limit14
        | Self::Limit15
        | Self::Limit16
        | Self::Limit17
        | Self::Limit18
        | Self::Limit19
        | Self::Accessed
        | Self::Writeable
        | Self::UserSegment
        | Self::Present
        | Self::Granularity
    }
}

impl BitFlagsValues for SegmentFlagsBits {
    /* No methods to implement */
}
