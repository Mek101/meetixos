/*! Page directory management */

use core::{
    fmt,
    fmt::Debug
};

use crate::{
    addr::{
        virt::VirtAddr,
        Address
    },
    arch::mem::paging::HwPageDirSupport,
    mem::paging::{
        allocator::FrameAllocator,
        flush::{
            MapFlush,
            MapFlusher,
            MapRangeFlush
        },
        frame::{
            PhysFrame,
            VirtFrame,
            VirtFrameRange
        },
        table::{
            PTFlags,
            PageTable,
            PageTableEntry,
            PageTableEntryErr,
            PageTableIndex,
            PageTableLevel
        },
        Page1GiB,
        Page2MiB,
        Page4KiB,
        PageSize
    }
};

/**
 * "Middle" level address space manager.
 *
 * Middle level because allows the user to abstract from the complications
 * of the raw `PageTable`s and `PageTableEntries` and the inner flags,
 * but must be used with consciousness because allows too to map reserved
 * addresses, remap stuffs with undefined consequences.
 *
 * So this is only an hardware abstraction layer to the paging manager, but
 * must be managed by an high level virtual memory manager into the kernel
 */
pub struct PageDir {
    m_root_frame: PhysFrame<Page4KiB>,
    m_phys_offset: PhysOffset
}

impl PageDir {
    /**
     * Construct a `PageDir` which maps physical addresses from the given
     * `phys_offset`
     */
    pub fn new(root_phys_frame: PhysFrame<Page4KiB>, phys_offset: VirtAddr) -> Self {
        Self { m_root_frame: root_phys_frame,
               m_phys_offset: PhysOffset::new(phys_offset) }
    }

    /**
     * Creates a new mapping into this page directory at the specified
     * `VirtFrame` address.
     *
     * This method also creates all the missing intermediate `PageTable`s
     * for the mapping using `FrameAllocator::alloc_page_table()`.
     *
     * Bigger are the requested `PageSize` less intermediate page tables
     * must be created/used, from 1 for `Page1GiB` to 3 for `Page4KiB`.
     *
     * The mapping frame (the `PhysFrame` that maps the `VirtFrame`
     * given) is not allocated when the given `PTFlags` not contains
     * `PTFlags::PRESENT` (useful for demand paging)
     */
    pub fn map_single<S, A>(&mut self,
                            virt_frame: VirtFrame<S>,
                            allocator: &mut A,
                            flags: PTFlags)
                            -> Result<MapFlush<S>, PageDirErr>
        where S: PageSize,
              A: FrameAllocator<S> {
        #[cfg(debug_assertions)]
        {
            /* ensure logical correctness in DEBUG mode */
            if !S::IS_BIG {
                assert!(!flags.is_huge_page());
            }
        }

        /* if the given PTFlags contains the <PRESENT> flags then request to the
         * given FrameAllocator to allocate a page of the requested PageSize.
         * If the page presence was requested then fail with an error, otherwise keep
         * a None; the map_frame() method will simply leave 0 the frame bits of the
         * PageTableEntry
         */
        let phys_frame = if flags.is_present() {
            if let Some(phys_frame) = allocator.alloc_page() {
                Some(phys_frame)
            } else {
                return Err(PageDirErr::PhysAllocFailed);
            }
        } else {
            None
        };

        /* add the <HUGE_PAGE> flag if required and not already present */
        let mut map_flags = flags.clone();
        if S::IS_BIG && !flags.is_huge_page() {
            map_flags |= PTFlags::HUGE_PAGE;
        }

        /* demand the actual mapping operations to the map_frame() method */
        self.map_frame(virt_frame, phys_frame, allocator, map_flags)
            .map(|_| MapFlush::new(virt_frame))
    }

    /**
     * Creates a new range of mappings into this page directory starting
     * from the first `VirtFrame` address of the given `VirtFrameRange`.
     *
     * Like `PageDir::map_single()`, this method also creates all the
     * missing intermediate `PageTable`s for all the mappings using
     * `FrameAllocator::alloc_page_table()`.
     *
     * As `PageDir::map_single()` do, this method will not allocate
     * mapping `PhysFrame`s if the given `PTFlags` not contain
     * `PTFlags::PRESENT`
     */
    pub fn map_range<S, A>(&mut self,
                           virt_range: VirtFrameRange<S>,
                           allocator: &mut A,
                           flags: PTFlags)
                           -> Result<MapRangeFlush<S>, PageDirErr>
        where S: PageSize,
              A: FrameAllocator<S> {
        /* ensure non-empty range to avoid useless computations */
        if virt_range.is_empty() {
            return Err(PageDirErr::EmptyRange);
        }

        /* iterate each virtual frame into the range and try to map each one */
        let original_range = virt_range.clone();
        for virt_frame in virt_range {
            /* try to map the single current VirtFrame and catch the error to be able
             * to unmap the already mapped range (i.e the failure could happen when
             * many frames was already mapped, we need to rollback)
             */
            if let Err(err) = self.map_single(virt_frame, allocator, flags)
                                  .map(|single_map_flusher| {
                                      /* here the mapper safely ignores the MapFlush
                                       * returned because the TLB cache will be
                                       * invalidated by the MapRangeFlush returned at
                                       * the end of this method when no errors occurs
                                       */
                                      single_map_flusher.ignore()
                                  })
            {
                /* here we can ignore the unmapping errors because we only want to
                 * de-allocate the mapped page tables and the mapped frames (if PTFlags
                 * contains PTFlags::PRESENT) to avoid memory waste in case of partial
                 * mapping success
                 */
                let _ = self.unmap_range(VirtFrame::range_of(original_range.start,
                                                             virt_frame),
                                         allocator,
                                         true)
                            .map(|flusher| {
                                /* when unmapping the frames that are part of a range
                                 * not completely mapped explicitly call
                                 * MapRangeFlush::flush() internally to avoid to return
                                 * another MapRangeFlush object as Err variant of the
                                 * Result, which may be confusing
                                 */
                                flusher.flush()
                            });
                return Err(err);
            }
        }

        /* return the range flusher from the given inclusive range */
        Ok(MapRangeFlush::new(VirtFrame::range_incl_of(original_range.start,
                                                       original_range.end - 1)))
    }

    /**
     * Removes the mapping for the given `VirtFrame` inside this page
     * directory and frees the mapping's `PhysFrame` if present.
     *
     * It could also collect empty intermediate `PageTable`s if
     * `collect_empty_page_tables = true` using
     * `FrameAllocator::free_page_table()`.
     *
     * Please keep in mind that the same not collected intermediate
     * `PageTable`s because of `collect_empty_page_tables = false` could
     * be collected by a following `collect_empty_page_tables = true` call
     * that unmaps a `VirtFrame` that resides into the same `PageTable`s, so
     * if you want to keep allocated a set of page tables be sure to NOT
     * unmap near `VirtFrame`s with `collect_empty_page_tables = true`
     */
    pub fn unmap_single<S, A>(&mut self,
                              virt_frame: VirtFrame<S>,
                              allocator: &mut A,
                              collect_empty_page_tables: bool)
                              -> Result<MapFlush<S>, PageDirErr>
        where S: PageSize,
              A: FrameAllocator<S> {
        self.unmap_frame(virt_frame, allocator, collect_empty_page_tables)
            .map(|phys_frame_opt| {
                /* give back the mapping PhysFrame to the allocator if it was present */
                if let Some(phys_frame) = phys_frame_opt {
                    allocator.free_page(phys_frame)
                }

                /* construct a page flusher for the removed mapping */
                MapFlush::new(virt_frame)
            })
    }

    /**
     * Removes a existing range of mappings into this page directory
     * starting from the first `VirtFrame` address of the given
     * `VirtFrameRange` and frees all the present mapping's `PhysFrame`.
     *
     * Like `PageDir::unmap_single()`, this method could also collect
     * empty intermediate `PageTable`s if `collect_empty_page_tables = true`
     * using `FrameAllocator::free_page_table()`
     */
    pub fn unmap_range<S, A>(&mut self,
                             virt_range: VirtFrameRange<S>,
                             allocator: &mut A,
                             collect_empty_page_tables: bool)
                             -> Result<MapRangeFlush<S>, PageDirErr>
        where S: PageSize,
              A: FrameAllocator<S> {
        /* ensure non-empty range to avoid useless computations */
        if virt_range.is_empty() {
            return Err(PageDirErr::EmptyRange);
        }

        /* iterate each virtual frame into the given range and unmap each one */
        let original_range = virt_range.clone();
        for virt_frame in virt_range {
            /* try to unmap the current VirtFrame, the FrameAllocator will receive the
             * mapped PhysFrame, if any, and the PhysFrames of the empty intermediate
             * PageTables if collect_empty_page_tables is true
             */
            if let Err(err) =
                self.unmap_single(virt_frame, allocator, collect_empty_page_tables)
                    .map(|single_unmap_flusher| {
                        /* here the mapper safely ignores the flusher returned because
                         * the TLB cache will be invalidated by the MapRangeFlush
                         * returned at the end when no errors occurs
                         */
                        single_unmap_flusher.ignore()
                    })
            {
                return Err(err);
            }
        }

        /* return the range flusher from the given inclusive range */
        Ok(MapRangeFlush::new(VirtFrame::range_incl_of(original_range.start,
                                                       original_range.end - 1)))
    }

    /**
     * Returns the `PTFlags` of the given `VirtFrame`
     */
    pub fn flags_of<S>(&self, virt_frame: VirtFrame<S>) -> Result<PTFlags, PageDirErr>
        where S: PageSize {
        /* iterate each level of the page tables to reach the requested one */
        let mut table = self.root_page_table();
        for pt_level in S::MAP_LEVEL.iter_until_this() {
            let next_page_table = &mut table[virt_frame.index_for_level(pt_level)];

            /* in this case the next page table existence is not ensured but fail */
            table = self.next_page_table::<S>(next_page_table)?
        }

        /* returns the flags for the selected entry */
        Ok(table[virt_frame.index_for_level(S::MAP_LEVEL)].flags())
    }

    /**
     * Makes this `PageDir` the active one
     */
    pub unsafe fn activate(&self) {
        HwPageDirSupport::activate_page_dir(self.m_root_frame);
    }

    /**
     * Returns the root `PhysFrame` of this `PageDir`
     */
    pub fn root_phys_frame(&self) -> PhysFrame<Page4KiB> {
        self.m_root_frame
    }

    /**
     * Returns the virtual to physical memory offset `VirtAddr`
     */
    pub fn phys_mem_offset(&self) -> VirtAddr {
        self.m_phys_offset.m_offset
    }

    /**
     * Returns the active page directory
     */
    pub unsafe fn active_page_dir(phys_offset: usize) -> PageDir {
        PageDir::new(HwPageDirSupport::active_page_dir_frame(),
                     VirtAddr::new(phys_offset))
    }

    /**
     * Performs the operations to actually map the given `VirtFrame` to
     * this page directory according to the given `PTFlags`
     */
    fn map_frame<S, A>(&mut self,
                       virt_frame: VirtFrame<S>,
                       phys_frame: Option<PhysFrame<S>>,
                       allocator: &mut A,
                       flags: PTFlags)
                       -> Result<(), PageDirErr>
        where S: PageSize,
              A: FrameAllocator<S> {
        /* iterate from the root page table to the page table level necessary to map
         * the requested VirtFrame.
         * Bigger is the PageSize requested, less intermediate page tables are needed
         * to map the frame (but bigger PhysFrame is requested to the
         * FrameAllocator).
         * The following cycle ensures too that the missing intermediate PageTables
         * are created requesting new page table pages to the FrameAllocator
         */
        let mut table = self.root_page_table();
        for pt_level in S::MAP_LEVEL.iter_until_this() {
            let next_table_entry = &mut table[virt_frame.index_for_level(pt_level)];

            /* Ensure the existence of the next page table and fail without any
             * rollback because caller methods unmaps partially done mappings
             */
            table = self.ensure_next_pt(next_table_entry, allocator, flags)?;
        }

        /* select the map level entry */
        let table_entry = &mut table[virt_frame.index_for_level(S::MAP_LEVEL)];

        /* ensure that the selected entry is unused or without PhysFrame (in case of
         * demand paging)
         */
        if table_entry.is_unused() || !table_entry.flags().is_present() {
            /* we have now to distinguish the two cases here:
             * 1) phys_frame is Some, explicit mapping is requested: we have to fill
             *    the entry with physical frame given and the flags
             * 2) phys_frame is None, demand paging mapping is requested: leave the
             *    frame bits to 0 (the kernel will receive a page fault when the page
             *    will receive the first access) but fill up the flags bits
             *
             * Note that the if clause is
             * `if .. || !table_entry.flags().is_present()`
             * and not
             * `if .. || (!table_entry.flags().is_present() && phys_frame.is_some())`
             * because the map method could be used to change the protection flags of
             * the mapping page table without effectively map a PhysFrame
             */
            if let Some(phys_frame) = phys_frame {
                table_entry.set_mapping(phys_frame, flags);
            } else {
                table_entry.set_flags(flags);
            }
            Ok(())
        } else {
            Err(PageDirErr::PageAlreadyMapped)
        }
    }

    /**
     * Performs the operations to actually unmap the given `VirtFrame`
     * from this page directory
     */
    fn unmap_frame<S, A>(&mut self,
                         virt_frame: VirtFrame<S>,
                         allocator: &mut A,
                         collect_empty_page_tables: bool)
                         -> Result<Option<PhysFrame<S>>, PageDirErr>
        where S: PageSize,
              A: FrameAllocator<S> {
        /* iterate each level of the page tables to reach the requested one */
        let mut table = self.root_page_table();
        for pt_level in S::MAP_LEVEL.iter_until_this() {
            let next_page_table = &mut table[virt_frame.index_for_level(pt_level)];

            /* in this case the next page table existence is not ensured but fail */
            table = self.next_page_table::<S>(next_page_table)?
        }

        /* select the map level entry */
        let table_entry = &mut table[virt_frame.index_for_level(S::MAP_LEVEL)];

        /* ensure that the entry is used or immediately return with the error */
        let unmap_res = if !table_entry.is_unused() {
            /* lets obtain the option that could contain the mapping PhysFrame.
             * If not present is not an error, it only means that the caller wants to
             * unmap a demand paging mapping that was not accessed
             */
            let frame_to_unmap = if let Ok(phys_frame) = table_entry.phys_frame() {
                Some(phys_frame)
            } else {
                None
            };

            /* clear the entry and return the frame to unmap */
            table_entry.clear();
            Ok(frame_to_unmap)
        } else {
            Err(PageDirErr::PageNotMapped)
        };

        /* collect the unused page tables if requested by the caller, this operation
         * cannot fail because only unmaps PhysFrames
         */
        if collect_empty_page_tables {
            self.collect_unused_page_tables(virt_frame,
                                            self.root_page_table(),
                                            PageTableLevel::Level4,
                                            allocator);
        }

        unmap_res
    }

    /**
     * Ensures that the `PageTable` referenced by the given
     * `PageTableEntry` exists (it will be created otherwise) and doesn't
     * contain a mapping to a bigger physical frame
     */
    fn ensure_next_pt<'b, S, A>(&self,
                                entry: &'b mut PageTableEntry,
                                allocator: &mut A,
                                flags: PTFlags)
                                -> Result<&'b mut PageTable, PageDirErr>
        where S: PageSize,
              A: FrameAllocator<S> {
        let new_table_created;

        /* check for empty page table entry, in that case we have to create it */
        if entry.is_unused() {
            /* request now to the given allocator a new page table physical frame */
            if let Some(table_phys_frame) = allocator.alloc_page_table() {
                /* compose the table flags, here must be distinguished whether the
                 * caller have given the <USER> flags or not, because intermediate
                 * page tables must be accessible as well.
                 * Otherwise map as supervisor pages the page tables to avoid
                 * resolution miss by the MMU, that could start resolve the
                 * virtual addresses and fail with the last level
                 */
                let table_flags = if flags.is_user() {
                    PTFlags::READABLE
                    | PTFlags::WRITEABLE
                    | PTFlags::PRESENT
                    | PTFlags::USER
                } else {
                    PTFlags::READABLE | PTFlags::WRITEABLE | PTFlags::PRESENT
                };

                /* set now the mapping */
                entry.set_mapping(table_phys_frame, table_flags);
                new_table_created = true;
            } else {
                return Err(PageDirErr::PhysAllocFailed);
            }
        } else {
            new_table_created = false;
        }

        /* obtain the reference to the next table starting from the physical address
         * of the page table entry and the physical offset of this page directory.
         * The validity checks about the physical frame usability are performed by
         * the PageTableEntry::phys_frame() method, which ensures that we don't use
         * this entry if maps a HUGE frame or, if contains a not-present mapping (for
         * demand paging)
         */
        let next_page_table = self.next_page_table::<S>(entry)?;

        /* clear the frame if new */
        if new_table_created {
            next_page_table.clear();
        }

        Ok(next_page_table)
    }

    /**
     * Calculates the virtual address to the next `PageTable`
     */
    fn next_page_table<'b, S>(&self,
                              entry: &'b mut PageTableEntry)
                              -> Result<&'b mut PageTable, PageDirErr>
        where S: PageSize {
        Ok(unsafe {
            &mut *self.m_phys_offset.next_table_pointer::<S>(entry.phys_frame()?)
        })
    }

    /**
     * Recursively checks whether the `PageTable`s that was used to map
     * the given `VirtFrame` are empty, in that case deallocates them
     * returning to the `FrameAllocator` given
     */
    fn collect_unused_page_tables<S, A>(&self,
                                        virt_frame: VirtFrame<S>,
                                        page_table: &mut PageTable,
                                        pt_level: PageTableLevel,
                                        allocator: &mut A)
        where S: PageSize,
              A: FrameAllocator<S> {
        /* recurse the page table level before the map level for the PageSize given.
         * The level before the map level because doing this we could easily access
         * the next level and use the current one as previous
         */
        if pt_level.as_usize() + 1 < S::MAP_LEVEL.as_usize() {
            let page_table_entry = &mut page_table[virt_frame.index_for_level(pt_level)];
            let next_table_res = self.next_page_table::<S>(page_table_entry);

            /* recurse to the next level page table */
            if let Ok(next_page_table) = next_table_res {
                if let Some(next_table_level) = pt_level.next_level() {
                    self.collect_unused_page_tables(virt_frame,
                                                    next_page_table,
                                                    next_table_level,
                                                    allocator)
                }
            }
        }

        /* obtain the reference to the next page table entry */
        let next_page_table_entry = &mut page_table[virt_frame.index_for_level(pt_level)];

        /* obtain the reference to the next page table */
        if let Ok(next_page_table) = self.next_page_table::<S>(next_page_table_entry) {
            /* well if the next level page table is empty (doesn't contains non-zero
             * page table entries) is possible to free the PhysFrame of the entry that
             * references the `next_page_table`
             */
            if next_page_table.is_empty() {
                /* after this call undefined behaviours could occur if
                 * `next_page_table` is used
                 */
                allocator.free_page_table(next_page_table_entry.phys_frame().unwrap());

                /* after this call any reference to the `next_page_table` is lost */
                next_page_table_entry.clear();
            }
        }
    }

    /**
     * Returns the mutable reference to the Level4 `PageTable`
     */
    fn root_page_table(&self) -> &mut PageTable {
        unsafe { &mut *self.m_phys_offset.next_table_pointer(self.m_root_frame) }
    }
}

impl Debug for PageDir {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (l4_index, l4_entry) in self.root_page_table().iter_mut().enumerate() {
            if l4_entry.is_unused() {
                continue;
            }

            if let Err(err) = writeln!(f, "\tL4{:?}", l4_entry) {
                return Err(err);
            }

            if l4_entry.flags().is_present() {
                if let Ok(l3_page_table) = self.next_page_table::<Page4KiB>(l4_entry) {
                    for (l3_index, l3_entry) in l3_page_table.iter_mut().enumerate() {
                        if l3_entry.is_unused() {
                            continue;
                        }

                        if let Err(err) = write!(f, "\t\tL3{:?}", l3_entry) {
                            return Err(err);
                        }
                        if l3_entry.flags().is_huge_page() {
                            let frame = VirtFrame::<Page1GiB>::from_table_indexes(PageTableIndex::new(l4_index as u16),
                                                                                  PageTableIndex::new(l3_index as u16));
                            if let Err(err) = writeln!(f, " VirtFrame<1GiB>({:?})", frame.start_addr()) {
                                return Err(err);
                            } else {
                                continue;
                            }
                        } else {
                            let _ = write!(f, "\n");
                        }

                        if l3_entry.flags().is_present() && !l3_entry.flags().is_huge_page() {
                            if let Ok(l2_page_table) = self.next_page_table::<Page4KiB>(l3_entry) {
                                for (l2_index, l2_entry) in l2_page_table.iter_mut().enumerate() {
                                    if l2_entry.is_unused() {
                                        continue;
                                    }

                                    if let Err(err) = write!(f, "\t\t\tL2{:?}", l2_entry) {
                                        return Err(err);
                                    }
                                    if l2_entry.flags().is_huge_page() {
                                        let frame = VirtFrame::<Page2MiB>::from_table_indexes(PageTableIndex::new(l4_index as u16),
                                                                                              PageTableIndex::new(l3_index as u16),
                                                                                              PageTableIndex::new(l2_index as u16));
                                        if let Err(err) = writeln!(f, " VirtFrame<2MiB>({:?})", frame.start_addr()) {
                                            return Err(err);
                                        } else {
                                            continue;
                                        }
                                    } else {
                                        let _ = write!(f, "\n");
                                    }

                                    if l2_entry.flags().is_present() && !l2_entry.flags().is_huge_page() {
                                        if let Ok(l1_page_table) = self.next_page_table::<Page4KiB>(l2_entry) {
                                            for (l1_index, l1_entry) in l1_page_table.iter_mut().enumerate() {
                                                if l1_entry.is_unused() {
                                                    continue;
                                                }

                                                let frame = VirtFrame::<Page4KiB>::from_table_indexes(PageTableIndex::new(l4_index as u16),
                                                                                                      PageTableIndex::new(l3_index as u16),
                                                                                                      PageTableIndex::new(l2_index as u16),
                                                                                                      PageTableIndex::new(l1_index as u16));

                                                if let Err(err) = writeln!(f, "\t\t\t\tL1{:?} VirtFrame<4KiB>({:?})", l1_entry, frame.start_addr()) {
                                                    return Err(err);
                                                }
                                            }
                                        }
                                    }
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

/**
 * Wrapper for the virtual memory offset of the `PageDir`.
 *
 * It is responsible to calculate the pointers to the next level page tables
 */
struct PhysOffset {
    m_offset: VirtAddr
}

impl PhysOffset {
    /**
     * Constructs a `PhysOffset` which asserts the well alignment of the
     * given `VirtAddr` to the `Page4KiB`
     */
    fn new(offset: VirtAddr) -> Self {
        assert!(offset.is_aligned(Page4KiB::SIZE));
        Self { m_offset: offset }
    }

    /**
     * Using the stored virtual offset returns the virtual pointer to the
     * next `PageTable` on which the given `PhysFrame` points
     */
    unsafe fn next_table_pointer<S>(&self, phys_frame: PhysFrame<S>) -> *mut PageTable
        where S: PageSize {
        (self.m_offset + phys_frame.start_addr().as_usize()).as_ptr_mut()
    }
}

/**
 * Interface of utilities, implemented by the architecture dependent code to
 * support the `PageDir` implementation
 */
pub(crate) trait HwPageDirSupportBase {
    /* The following values are assigned with the real page table (entry)
     * flags expected by the hardware architecture, they will be used by the
     * `PageDir` for his published internal flags
     */
    const PTE_PRESENT: usize;
    const PTE_READABLE: usize;
    const PTE_WRITEABLE: usize;
    const PTE_GLOBAL: usize;
    const PTE_HUGE: usize;
    const PTE_ACCESSED: usize;
    const PTE_DIRTY: usize;
    const PTE_NO_EXECUTE: usize;
    const PTE_USER: usize;

    /* Page Table Entry physical address mask */
    const PTE_ADDR_MASK: usize;

    /* Page Table entries count */
    const PT_ENTRIES_COUNT: usize;

    /* Page Table levels */
    const PT_LEVEL_PGDIR: PageTableLevel;
    const PT_LEVEL_1GB: PageTableLevel;
    const PT_LEVEL_2MB: PageTableLevel;
    const PT_LEVEL_4KB: PageTableLevel;

    /**
     * Returns the current `PageDir`'s `PhysFrame`
     */
    unsafe fn active_page_dir_frame() -> PhysFrame<Page4KiB>;

    /**
     * Activates the given `PhysFrame` as current `PageDir`
     */
    unsafe fn activate_page_dir(phys_frame: PhysFrame<Page4KiB>);
}

/**
 * Lists the errors that could occur when using the `PageDir`
 */
pub enum PageDirErr {
    PageNotMapped,
    PageAlreadyMapped,
    EmptyRange,
    PhysAllocFailed,
    PartialHugePageUnmap
}

impl From<PageTableEntryErr> for PageDirErr {
    fn from(pte_err: PageTableEntryErr) -> Self {
        match pte_err {
            PageTableEntryErr::PhysFrameNotPresent => Self::PageNotMapped,
            PageTableEntryErr::InUseForBigFrame => Self::PageAlreadyMapped
        }
    }
}
