/*! # Raw Page Table Structures
 *
 * Implements the concrete and raw paging structures (page tables and page
 * table entry)
 */

use core::{
    fmt,
    fmt::{Debug, Formatter},
    ops::{Index, IndexMut}
};

use crate::hal::{
    addr::{Address, PhysAddr},
    arch::paging::HwPageDirSupport,
    paging::{HwPageDirSupportBase, PageSize, PhysFrame}
};

/** # Page Table
 *
 * Implements the raw page table structure, it is ensured by the compiler
 * that this object is always page aligned.
 *
 * Internally contains [`HwPageDirSupport::PT_ENTRIES_COUNT`] entries of
 * [`PageTableEntry`] structures that they could point to the next page
 * table level or contain the mapping of the physical frame
 *
 * [`HwPageDirSupport::PT_ENTRIES_COUNT`]:
 * /hal/paging/trait.HwPageDirConstsBase.html#associatedconstant.
 * PT_ENTRIES_COUNT
 * [`PageTableEntry`]:
 * /hal/paging/struct.PageTableEntry.html
 */
#[repr(C)]
#[repr(align(4096))]
pub struct PageTable {
    m_entries: [PageTableEntry; HwPageDirSupport::PT_ENTRIES_COUNT]
}

impl PageTable {
    /** # Constructs a `PageTable`
     *
     * The returned instance is completely blank
     */
    pub fn new() -> Self {
        Self { m_entries: [PageTableEntry::new(); HwPageDirSupport::PT_ENTRIES_COUNT] }
    }

    /** # Wipes out the `PageTable`
     *
     * This page table is completely made blank again as newly constructed
     */
    pub fn clear(&mut self) {
        for entry in self.iter_mut() {
            entry.clear();
        }
    }

    /** Returns an [`Iterator`] of immutable [`PageTableEntry`] references
     *
     * [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
     * [`PageTableEntry`]: /hal/paging/struct.PageTableEntry.html
     */
    pub fn iter(&self) -> impl Iterator<Item = &PageTableEntry> {
        self.m_entries.iter()
    }

    /** Returns an [`Iterator`] of mutable [`PageTableEntry`] references
     *
     * [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
     * [`PageTableEntry`]: /hal/paging/struct.PageTableEntry.html
     */
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PageTableEntry> {
        self.m_entries.iter_mut()
    }

    /** Returns whether this `PageTable` contains all unused entries
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
    /** The returned type after indexing.
     */
    type Output = PageTableEntry;

    /** Performs the indexing (`container[index]`) operation.
     */
    fn index(&self, index: T) -> &Self::Output {
        self.m_entries.index(index.into())
    }
}

impl<T> IndexMut<T> for PageTable where T: Into<usize> {
    /** Performs the mutable indexing (`container[index]`) operation
     */
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        self.m_entries.index_mut(index.into())
    }
}

/** # Page Table Entry
 *
 * Implements the raw page table entry structure.
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
    /** # Constructs an empty `PageTableEntry`
     *
     * The returned instance is blank and zeroed
     */
    pub const fn new() -> Self {
        Self { m_entry_data: 0 }
    }

    /** # Safe Returns the `PageTableEntry`'s `PhysFrame`
     *
     * Checks whether any frame is mapped and present and whether the size
     * corresponds to the requested one.
     *
     * If these checks pass the mapped [`PhysFrame`] is returned
     *
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
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
        if !S::IS_BIG && self.flags().contains(PTFlags::HUGE_PAGE) {
            Err(PageTableEntryErr::InUseForBigFrame)
        } else if !self.flags().contains(PTFlags::PRESENT) {
            Err(PageTableEntryErr::PhysFrameNotPresent)
        } else {
            Ok(PhysFrame::of_addr(self.address()))
        }
    }

    /** # Updates the mapping
     *
     * Updates the `PageTableEntry` data with the given [`PhysFrame`] and
     * the given [`PTFlags`]
     *
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     * [`PTFLags`]: /hal/paging/struct.PTFlags.html
     */
    pub fn set_mapping<S>(&mut self, phys_frame: PhysFrame<S>, flags: PTFlags)
        where S: PageSize {
        self.m_entry_data = phys_frame.start_addr().as_usize() | flags.bits();
    }

    /** # Wipes out the `PageTableEntry`'s data
     *
     * The containing data is cleared and substituted with zeros, bhe sure
     * to gain back the mapped frame if any to avoid waste of physical
     * memory
     */
    pub fn clear(&mut self) {
        self.m_entry_data = 0;
    }

    /** Returns the mapped [`PhysAddr`] without any check
     *
     * [`PhysAddr`]: /hal/addr/struct.PhysAddr.html
     */
    pub fn address(&self) -> PhysAddr {
        unsafe {
            PhysAddr::new_unchecked(self.m_entry_data & HwPageDirSupport::PTE_ADDR_MASK)
        }
    }

    /** Returns the [`PTFlags`] for this `PageTableEntry`
     *
     * [`PTFLags`]: /hal/paging/struct.PTFlags.html
     */
    pub fn flags(&self) -> PTFlags {
        PTFlags::from_bits_truncate(self.m_entry_data)
    }

    /** Updates only the [`PTFlags`] of this `PageTableEntry`
     *
     * [`PTFLags`]: /hal/paging/struct.PTFlags.html
     */
    pub fn set_flags(&mut self, flags: PTFlags) {
        self.m_entry_data = self.address().as_usize() | flags.bits();
    }

    /** Returns whether this `PageTableEntry` is clear
     */
    pub fn is_unused(&self) -> bool {
        self.m_entry_data == 0
    }
}

impl Debug for PageTableEntry {
    /** Formats the value using the given formatter
     */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("PageTableEntry")
         .field("m_phys_frame", &self.address())
         .field("m_flags", &self.flags())
         .finish()
    }
}

/** # Page Table Index
 *
 * Represents a 9-bit index inside a [`PageTable`] to select one of the 512
 * [`PageTableEntries`].
 *
 * It is ensured that it doesn't contain values bigger than
 * [`HwPageDirSupport::PT_ENTRIES_COUNT`]
 *
 * [`PageTable`]: /hal/paging/struct.PageTable.html
 * [`PageTableEntries`]: /hal/paging/struct.PageTableEntry.html
 * [`HwPageDirSupport::PT_ENTRIES_COUNT`]:
 * /hal/paging/trait.HwPageDirConstsBase.html#associatedconstant.
 * PT_ENTRIES_COUNT
 */
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PageTableIndex {
    m_index: u16
}

impl PageTableIndex {
    /** # Constructs a `PageTableIndex`
     *
     * Cuts bits after value >= [`HwPageDirSupport::PT_ENTRIES_COUNT`]
     *
     * [`HwPageDirSupport::PT_ENTRIES_COUNT`]:
     * /hal/paging/trait.HwPageDirConstsBase.html#associatedconstant.
     * PT_ENTRIES_COUNT
     */
    pub const fn new(index: u16) -> Self {
        Self { m_index: index % HwPageDirSupport::PT_ENTRIES_COUNT as u16 }
    }
}

impl Into<usize> for PageTableIndex {
    /** Performs the conversion
     */
    fn into(self) -> usize {
        self.m_index as usize
    }
}

impl Into<u16> for PageTableIndex {
    /** Performs the conversion
     */
    fn into(self) -> u16 {
        self.m_index
    }
}

ext_bitflags! {
    /** # Page Table Flags
     *
     * Exposes an architecture independent set of [`PageTable`]'s flags
     ** primarily used by the [`PageDir`] object.
     *
     * Not all the following flags have meaning in all the supported
     * architectures, but are supported and used to have a common layer
     *
     * [`PageTable`]: /hal/paging/struct.PageTable.html
     * [`PageDir`]: /hal/paging/struct.PageDir.html
     */
    pub struct PTFlags: usize {
        /** Tells whether the [`PageTableEntry`] contains a valid [`PhysFrame`]
         *
         * [`PageTableEntry`]: /hal/paging/struct.PageTableEntry.html
         * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
         */
        const PRESENT    = HwPageDirSupport::PTE_PRESENT;

        /** Tells whether the [`PageTable`] or the [`PageTableEntry`] is readable
         *
         * [`PageTable`]: /hal/paging/struct.PageTable.html
         * [`PageTableEntry`]: /hal/paging/struct.PageTableEntry.html
         */
        const READABLE   = HwPageDirSupport::PTE_READABLE;

        /** Tells whether the [`PageTable`] or the [`PageTableEntry`] is writeable
         *
         * [`PageTable`]: /hal/paging/struct.PageTable.html
         * [`PageTableEntry`]: /hal/paging/struct.PageTableEntry.html
         */
        const WRITEABLE  = HwPageDirSupport::PTE_WRITEABLE;

        /** Tells whether the [`PageTableEntry`] is a global page
         *
         * [`PageTableEntry`]: /hal/paging/struct.PageTableEntry.html
         */
        const GLOBAL = HwPageDirSupport::PTE_GLOBAL;

        /** Tells whether the [`PageTableEntry`]'s references a big [`PhysFrame`]
         *
         * [`PageTableEntry`]: /hal/paging/struct.PageTableEntry.html
         * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
         */
        const HUGE_PAGE  = HwPageDirSupport::PTE_HUGE;

        /** Tells whether the [`PageTable`] or the [`PageTableEntry`] was accessed
         * (i.e read)
         * Note that this flag is applied by the CPU
         *
         * [`PageTable`]: /hal/paging/struct.PageTable.html
         * [`PageTableEntry`]: /hal/paging/struct.PageTableEntry.html
         */
        const ACCESSED   = HwPageDirSupport::PTE_ACCESSED;

        /** Tells whether the [`PageTable`] or the [`PageTableEntry`] is dirty
         * (i.e written)
         * Note that this flag is applied by the CPU
         *
         * [`PageTable`]: /hal/paging/struct.PageTable.html
         * [`PageTableEntry`]: /hal/paging/struct.PageTableEntry.html
         */
        const DIRTY      = HwPageDirSupport::PTE_DIRTY;

        /** Tells whether the [`PageTableEntry`] allows the execution of code
         *
         * [`PageTableEntry`]: /hal/paging/struct.PageTableEntry.html
         */
        const NO_EXECUTE = HwPageDirSupport::PTE_NO_EXECUTE;

        /** Tells whether the [`PageTable`] or the [`PageTableEntry`] is accessible
         * by the userspace
         *
         * [`PageTable`]: /hal/paging/struct.PageTable.html
         * [`PageTableEntry`]: /hal/paging/struct.PageTableEntry.html
         */
        const USER       = HwPageDirSupport::PTE_USER;
    }
}

c_handy_enum! {
    /** # Page Table Level
     *
     * Enumerates the 4 level of page tables supported by the 64 bit
     * architectures.
     *
     * This simple enum is used by the [`PageDir`] structure to iterate
     * and construct intermediate page table levels
     *
     * [`PageDir`]: /hal/paging/struct.PageDir.html
     */
    pub enum PageTableLevel: u8 {
        Level4 = 0,
        Level3 = 1,
        Level2 = 2,
        Level1 = 3,
    }
}

c_handy_enum! {
    /** # Page Table Entry Errors
     *
     * Enumerates the errors that could occur when call
     * [`PageTableEntry::phys_frame()`]
     *
     * [`PageTableEntry::phys_frame()`]: /hal/paging/struct.PageTableEntry.html#method.phys_frame
     */
    pub enum PageTableEntryErr : u8 {
        /** The [`PhysFrame`] is not present (i.e the entry haven't
         * the [`PTFlags::PRESENT`] flag active)
         *
         * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
         * [`PTFlags::PRESENT`]: /hal/paging/struct.PTFlags.html#associatedconstant.PRESENT
         */
        PhysFrameNotPresent = 0,

        /** Requested the [`PhysFrame`] of a smallest [`PageSize`] of the
         * currently stored (i.e requested a [`Page4KiB`] frame but
         * the entry contains a [`Page2MiB`] or a [`Page1GiB`])
         *
         * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
         * [`PageSize`]: /hal/paging/trait.PageSize.html
         * [`Page4KiB`]: /hal/paging/struct.Page4KiB.html
         * [`Page2MiB`]: /hal/paging/struct.Page2MiB.html
         * [`Page1GiB`]: /hal/paging/struct.Page1GiB.html
         */
        InUseForBigFrame    = 1,
    }
}
