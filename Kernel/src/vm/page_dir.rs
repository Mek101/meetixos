/*! Page directory */

use core::{
    fmt,
    fmt::Debug
};

use crate::{
    addr::{
        phys_addr::PhysAddr,
        virt_addr::VirtAddr,
        TAddress
    },
    arch::vm::hw_page_dir::HwPageDir,
    vm::{
        mem_manager::MemManager,
        page_table::{
            PageTable,
            PageTableIndex,
            PageTableLevel
        },
        page_table_entry::PageTableEntry,
        Page4KiB,
        TPageSize
    }
};

pub struct PageDir {
    m_hw_page_dir: HwPageDir,
    m_phys_mem_offset: VirtAddr
}

impl PageDir /* Constructors */ {
    pub fn current() -> Self {
        Self { m_hw_page_dir: HwPageDir::current(),
               m_phys_mem_offset: MemManager::instance().layout_manager()
                                                        .phys_mem_mapping_range()
                                                        .start }
    }

    pub fn pre_phys_mapping() -> Self {
        Self { m_hw_page_dir: HwPageDir::current(),
               m_phys_mem_offset: VirtAddr::null() }
    }
}

impl PageDir /* Methods */ {
    pub unsafe fn activate(&self) {
        self.m_hw_page_dir.activate();
    }

    /**
     * Returns the mapping `PageTableEntry` for the given `VirtAddr`
     */
    pub fn ensure_page_table_entry<S>(&self,
                                      virt_addr: VirtAddr)
                                      -> Option<&mut PageTableEntry>
        where S: TPageSize {
        if virt_addr.is_aligned(S::SIZE) {
            let l4_page_table = self.root_page_table();

            /* obtain the Level3 page-table */
            let l3_page_table =
                self.ensure_next_page_table_from_level(virt_addr,
                                                       l4_page_table,
                                                       PageTableLevel::Root)?;

            /* obtain the Level2 page-table */
            let l2_page_table =
                self.ensure_next_page_table_from_level(virt_addr,
                                                       l3_page_table,
                                                       PageTableLevel::OneGiB)?;

            /* obtain the last mapping page-table level */
            let map_page_table = if S::SIZE == Page4KiB::SIZE {
                /* if a <Page4KiB> mapping is requested go a level deeper */
                self.ensure_next_page_table_from_level(virt_addr,
                                                       l2_page_table,
                                                       PageTableLevel::TwoMiB)?
            } else {
                l2_page_table
            };

            /* extract the <PageTableEntry> from the mapping level */
            Some(&mut map_page_table[virt_addr.page_table_index(S::PAGE_TABLE_LEVEL)])
        } else {
            None
        }
    }

    pub unsafe fn next_page_table(&self,
                                  page_table_entry: &PageTableEntry)
                                  -> &mut PageTable {
        assert!(page_table_entry.is_present(),
                "Tried to obtain a non-present `&mut PageTable`");

        self.frame_to_next_page_table(page_table_entry.phys_frame().unwrap())
    }
}

impl PageDir /* Getters */ {
    pub fn root_phys_frame(&self) -> PhysAddr {
        self.m_hw_page_dir.root_phys_frame()
    }

    pub fn root_page_table(&self) -> &mut PageTable {
        unsafe { self.frame_to_next_page_table(self.root_phys_frame()) }
    }
}

impl PageDir /* Privates */ {
    /**
     * Ensures the next level `PageTable` for the given `VirtAddr` into the
     * given `PageDir`.
     *
     * Allocates the missing `PageTable` if necessary
     */
    fn ensure_next_page_table_from_level(&self,
                                         virt_addr: VirtAddr,
                                         prev_table: &mut PageTable,
                                         page_table_level: PageTableLevel)
                                         -> Option<&mut PageTable> {
        /* obtain the <PageTableEntry> from the previous table */
        let page_table_entry =
            &mut prev_table[virt_addr.page_table_index(page_table_level)];

        /* allocate the next page-table if missing */
        let new_table_created = if page_table_entry.is_unused() {
            let phys_frame = MemManager::instance().allocate_kernel_phys_frame()?;

            page_table_entry.set_phys_frame(phys_frame);
            page_table_entry.set_present(true);
            page_table_entry.set_readable(true);
            page_table_entry.set_writeable(true);

            true
        } else {
            false
        };

        /* obtain the next page-table from the current entry */
        let next_page_table = unsafe { &mut *self.next_page_table(page_table_entry) };

        /* clear it if it is new */
        if new_table_created {
            next_page_table.clear();
        }
        Some(next_page_table)
    }

    unsafe fn frame_to_next_page_table(&self, phys_frame: PhysAddr) -> &mut PageTable {
        let page_table_virt_addr: VirtAddr =
            (*phys_frame + *self.m_phys_mem_offset).into();

        page_table_virt_addr.as_ref_mut()
    }
}

impl Debug for PageDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,
                 "PageDir {{ m_phys_frame: {}, m_phys_offset: {} }}",
                 self.root_phys_frame(),
                 self.m_phys_mem_offset)?;

        for (l4_index, l4_page_table_entry) in self.root_page_table().iter().enumerate() {
            if l4_page_table_entry.is_unused() {
                continue;
            }

            writeln!(f, "\t{:03} L4{:?}", l4_index, l4_page_table_entry)?;

            if l4_page_table_entry.is_present() {
                for (l3_index, l3_page_table_entry) in
                    unsafe { self.next_page_table(l4_page_table_entry) }.iter()
                                                                        .enumerate()
                {
                    if l3_page_table_entry.is_unused() {
                        continue;
                    }

                    writeln!(f, "\t\t{:03} L3{:?}", l3_index, l3_page_table_entry)?;

                    if l3_page_table_entry.is_present() {
                        for (l2_index,l2_page_table_entry) in
                            unsafe { self.next_page_table(l3_page_table_entry) }.iter().enumerate()
                        {
                            if l2_page_table_entry.is_unused() {
                                continue;
                            }

                            write!(f, "\t\t\t{:03} L2{:?}", l2_index,l2_page_table_entry)?;

                            if l2_page_table_entry.is_huge_page() {
                                let virt_frame = VirtAddr::from_2mib_indexes(PageTableIndex::from(l4_index),
                                                                             PageTableIndex::from(l3_index),
                                                                             PageTableIndex::from(l2_index));
                                writeln!(f, " VirtFrame<2MiB>({}) (L4 {}, L3 {}, L2 {})", virt_frame, l4_index,l3_index,l2_index)?;
                                continue;
                            } else {
                                writeln!(f)?;
                            }

                            if l2_page_table_entry.is_present() && !l2_page_table_entry.is_huge_page() {
                                for (l1_index,l1_page_table_entry) in
                                    unsafe{ self.next_page_table(l2_page_table_entry) }.iter()
                                                                                       .enumerate() {
                                    if l1_page_table_entry.is_unused() {
                                        continue;
                                    }

                                    let virt_frame = VirtAddr::from_4kib_indexes(PageTableIndex::from(l4_index),
                                                                                 PageTableIndex::from(l3_index),
                                                                                 PageTableIndex::from(l2_index),
                                                                                 PageTableIndex::from(l1_index));
                                    writeln!(f, "\t\t\t\t{:03} L1{:?} VirtFrame<4KiB>({}) (L4 {}, L3 {}, L2 {}, L1 {})", l1_index, l1_page_table_entry, virt_frame,l4_index, l3_index, l2_index, l1_index)?;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

pub trait THwPageDir: Debug {
    fn from_phys_frame(phys_frame: PhysAddr) -> Self;

    fn current() -> Self;

    unsafe fn activate(&self);

    fn root_phys_frame(&self) -> PhysAddr;
}
