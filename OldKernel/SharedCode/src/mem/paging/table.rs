/*! Raw paging structures */

use core::{
    fmt,
    fmt::Debug,
    ops::{
        Index,
        IndexMut
    }
};

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use crate::{
    addr::{
        phys::PhysAddr,
        Address
    },
    arch::mem::paging::dir::HwPageDirSupport,
    mem::paging::{
        dir::HwPageDirSupportBase,
        frame::PhysFrame,
        PageSize
    }
};
use bits::bit_flags::{
    BitFlags,
    BitFlagsValues
};

/**
 * Raw page table structure.
 *
 * It is ensured by the compiler that this object is always page aligned.
 *
 * Internally contains `HwPageDirSupport::PT_ENTRIES_COUNT` entries of
 * `PageTableEntry` structures that they could point to the next page
 * table level or contain the mapping of the physical frame
 */
#[repr(C)]
#[repr(align(4096))]
pub struct PageTable {
    m_entries: [PageTableEntry; HwPageDirSupport::PT_ENTRIES_COUNT]
}

impl PageTable {
    /**
     * Constructs a black `PageTable`
     */
    pub const fn new() -> Self {
        const CLEAN_ENTRY: PageTableEntry = PageTableEntry::new();
        Self { m_entries: [CLEAN_ENTRY; HwPageDirSupport::PT_ENTRIES_COUNT] }
    }

    /**  
     * Wipes out this `PageTable`
     */
    pub fn clear(&mut self) {
        for entry in self.iter_mut() {
            entry.clear();
        }
    }

    /**
     * Returns an `Iterator` of immutable `PageTableEntry` references
     */
    pub fn iter(&self) -> impl Iterator<Item = &PageTableEntry> {
        self.m_entries.iter()
    }

    /**
     * Returns an `Iterator` of mutable `PageTableEntry` references
     */
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PageTableEntry> {
        self.m_entries.iter_mut()
    }

    /**
     * Returns whether this `PageTable` contains all unused entries
     */
    pub fn is_empty(&self) -> bool {
        for entry in self.iter() {
            if !entry.is_unused() {
                return false;
            }
        }
        true
    }
}

impl<T> Index<T> for PageTable where T: Into<usize> {
    type Output = PageTableEntry;

    fn index(&self, index: T) -> &Self::Output {
        self.m_entries.index(index.into())
    }
}

impl<T> IndexMut<T> for PageTable where T: Into<usize> {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        self.m_entries.index_mut(index.into())
    }
}

/**
 * Raw page table entry structure.
 *
 * Internally contains the data and the flags to the next level page table
 * or the physical frame mapped for the mapping
 */
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PageTableEntry {
    m_entry_data: usize
}

impl PageTableEntry {
    /**  
     * Constructs a clean `PageTableEntry`
     */
    pub const fn new() -> Self {
        Self { m_entry_data: 0 }
    }

    /**  
     * Safe Returns the `PageTableEntry`'s `PhysFrame`
     */
    pub fn phys_frame<S>(&self) -> Result<PhysFrame<S>, PageTableEntryErr>
        where S: PageSize {
        /* since this method is primarily called by the PageDir object to obtain
         * physical frame to the next PageTable here is important the order on which
         * the controls are performed.
         * First of all must be checked whether the requested physical frame size is
         * big or not (since when this method is called to obtain the <4KiB>
         * physical frame to the next page table level) and whether this
         * entry is already used for a big frame, that is perfectly
         * legal to be NOT present due to demand paging, then the PRESENT flag
         * presence is tested after because of this
         */
        if !S::IS_BIG && self.flags().is_enabled(PTFlagsBits::HugePage) {
            Err(PageTableEntryErr::InUseForBigFrame)
        } else if !self.flags().is_enabled(PTFlagsBits::Present) {
            Err(PageTableEntryErr::PhysFrameNotPresent)
        } else {
            Ok(self.address().containing_frame())
        }
    }

    /**
     * Updates the `PageTableEntry` data with the given `PhysFrame` and
     * the given `PTFlags`
     */
    pub fn set_mapping<S>(&mut self,
                          phys_frame: PhysFrame<S>,
                          pt_flags: BitFlags<usize, PTFlagsBits>)
        where S: PageSize {
        self.m_entry_data = phys_frame.start_addr().as_usize() | pt_flags.raw_bits();
    }

    /**
     * The containing data is cleared and substituted with zeros.
     *
     * be sure to gain back the mapped frame if any to avoid waste of
     * physical memory
     */
    pub fn clear(&mut self) {
        self.m_entry_data = 0;
    }

    /**
     * Returns the mapped `PhysAddr` without any check
     */
    pub fn address(&self) -> PhysAddr {
        PhysAddr::new(self.m_entry_data & HwPageDirSupport::PTE_ADDR_MASK)
    }

    /**
     * Returns the `PTFlags` for this `PageTableEntry`
     */
    pub fn flags(&self) -> BitFlags<usize, PTFlagsBits> {
        BitFlags::from_raw_truncate(self.m_entry_data)
    }

    /**
     * Updates only the `PTFlags` of this `PageTableEntry`
     */
    pub fn set_flags(&mut self, flags: BitFlags<usize, PTFlagsBits>) {
        self.m_entry_data = self.address().as_usize() | flags.raw_bits();
    }

    /**
     * Returns whether this `PageTableEntry` is clear
     */
    pub fn is_unused(&self) -> bool {
        self.m_entry_data == 0
    }
}

impl Debug for PageTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PageTableEntry")
         .field("m_phys_frame", &self.address())
         .field("m_flags", &self.flags())
         .finish()
    }
}

/** # Page Table Index
 *
 * 9-bit index inside a `PageTable` to select one of the 512
 * `PageTableEntries`.
 *
 * It is ensured that it doesn't contain values bigger than
 * `HwPageDirSupport::PT_ENTRIES_COUNT`
 */
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PageTableIndex {
    m_index: u16
}

impl PageTableIndex {
    /**
     * Constructs a `PageTableIndex` wiping out the bits after
     * `HwPageDirSupport::PT_ENTRIES_COUNT`
     */
    pub const fn new(index: u16) -> Self {
        Self { m_index: index % HwPageDirSupport::PT_ENTRIES_COUNT as u16 }
    }
}

impl Into<usize> for PageTableIndex {
    fn into(self) -> usize {
        self.m_index as usize
    }
}

impl Into<u16> for PageTableIndex {
    fn into(self) -> u16 {
        self.m_index
    }
}

/**
 * Exposes an architecture independent set of `PageTable`'s flags
 * primarily used by the `PageDir` object.
 *
 * Not all the following flags have meaning in all the supported
 * architectures, but are supported and used to have a common layer
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(TryFromPrimitive, IntoPrimitive)]
pub enum PTFlagsBits {
    /**
     * Tells whether the `PageTableEntry` contains a valid `PhysFrame`
     */
    Present   = HwPageDirSupport::PTE_PRESENT,

    /**
     * Tells whether the `PageTable` or the `PageTableEntry` is readable
     */
    Readable  = HwPageDirSupport::PTE_READABLE,

    /**
     * Tells whether the `PageTable` or the `PageTableEntry` is writeable
     */
    Writeable = HwPageDirSupport::PTE_WRITEABLE,

    /**
     * Tells whether the `PageTableEntry` is a global page
     */
    Global    = HwPageDirSupport::PTE_GLOBAL,

    /**
     * Tells whether the `PageTableEntry`'s references a big `PhysFrame`
     */
    HugePage  = HwPageDirSupport::PTE_HUGE,

    /**
     * Tells whether the `PageTable` or the `PageTableEntry` was accessed
     * (i.e read)
     * Note that this flag is applied by the CPU
     */
    Accessed  = HwPageDirSupport::PTE_ACCESSED,

    /**
     * Tells whether the `PageTable` or the `PageTableEntry` is dirty
     * (i.e written)
     * Note that this flag is applied by the CPU
     */
    Dirty     = HwPageDirSupport::PTE_DIRTY,

    /**
     * Tells whether the `PageTableEntry` allows the execution of code
     */
    NoExecute = HwPageDirSupport::PTE_NO_EXECUTE,

    /**
     * Tells whether the `PageTable` or the `PageTableEntry` is accessible
     * by the userspace
     */
    User      = HwPageDirSupport::PTE_USER
}

impl BitFlagsValues for PTFlagsBits {
}

/**
 * Lists the errors that could occur when call
 * `PageTableEntry::phys_frame()`
 */
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum PageTableEntryErr {
    /**
     * The `PhysFrame` is not present (i.e the entry haven't the
     * `PTFlags::PRESENT` flag active)
     */
    PhysFrameNotPresent,

    /**
     * Requested the `PhysFrame` of a smallest `PageSize` of the currently
     * stored (i.e requested a `Page4KiB` frame but the entry contains a
     * `Page2MiB` or a `Page1GiB`)
     */
    InUseForBigFrame
}

/**
 * Lists the 4 level of page tables supported by the 64 bit architectures.
 *
 * This simple enum is used by the `PageDir` structure to iterate and
 * construct intermediate page table levels
 */
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum PageTableLevel {
    Level4,
    Level3,
    Level2,
    Level1
}

impl PageTableLevel {
    /**
     * Returns the next variant if any
     */
    pub fn next_level(&self) -> Option<PageTableLevel> {
        match self {
            Self::Level4 => Some(Self::Level3),
            Self::Level3 => Some(Self::Level2),
            Self::Level2 => Some(Self::Level1),
            Self::Level1 => None
        }
    }

    /**
     * Returns an `Iterator` implementation which sequentially iterates the
     * variants from the begin to the one before `self`
     */
    pub fn iter_until_this(&self) -> impl Iterator<Item = PageTableLevel> {
        struct PageTableLevelIter {
            m_current: Option<PageTableLevel>,
            m_after_last: PageTableLevel
        }

        impl Iterator for PageTableLevelIter {
            type Item = PageTableLevel;

            fn next(&mut self) -> Option<Self::Item> {
                let current = self.m_current;
                if let Some(current) = current {
                    /* update the next-current value */
                    self.m_current = if let Some(next_level) = current.next_level() {
                        if next_level < self.m_after_last {
                            Some(next_level)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                current
            }
        }

        PageTableLevelIter { m_current: Some(PageTableLevel::Level4),
                             m_after_last: self.clone() }
    }

    /**
     * Returns the variant as `usize` without consuming it
     */
    pub fn as_usize(&self) -> usize {
        self.clone() as usize
    }
}
