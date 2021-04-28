/*! # HAL Page Directory Management
 *
 * Implements an high level abstraction for the page directory management
 */

use core::{
    fmt,
    fmt::{Debug, Formatter}
};

use crate::hal::{
    addr::{Address, VirtAddr},
    arch::paging::HwPageDirSupport,
    paging::{
        FrameAllocator, MapFlush, MapFlusher, MapRangeFlush, PTFlags, Page1GiB, Page2MiB,
        Page4KiB, PageSize, PageTable, PageTableEntry, PageTableEntryErr, PageTableIndex,
        PageTableLevel, PhysFrame, VirtFrame, VirtFrameRange
    }
};

/** # Page Directory
 *
 * Implements a "middle" level address space manager.
 *
 * Middle level because allows the user to abstract from the complications
 * of the raw [`PageTable`]s and [`PageTableEntries`] and the inner flags,
 * but must be used with consciousness because allows too to map reserved
 * addresses, remap stuffs with undefined consequences.
 *
 * So this is only an hardware abstraction layer to the paging manager, but
 * must be managed by an high level virtual memory manager into the kernel
 *
 * [`PageTable`]: /hal/paging/struct.PageTable.html
 * [`PageTableEntries`]: /hal/paging/struct.PageTableEntries.html
 */
pub struct PageDir {
    m_root_frame: PhysFrame<Page4KiB>,
    m_phys_offset: PhysOffset
}

impl PageDir {
    /** # Construct a `PageDir`
     *
     * The returned instance is able to perform mapping and unmapping
     */
    pub fn new(root_phys_frame: PhysFrame<Page4KiB>, phys_offset: VirtAddr) -> Self {
        Self { m_root_frame: root_phys_frame,
               m_phys_offset: PhysOffset::new(phys_offset) }
    }

    /** # Maps a single `VirtFrame`
     *
     * Creates a new mapping into this page directory at the specified
     * [`VirtFrame`] address.
     *
     * This method also creates all the missing intermediate [`PageTable`]s
     * for the mapping using [`FrameAllocator::alloc_page_table()`].
     *
     * Bigger are the requested [`PageSize`] less intermediate page tables
     * must be created/used, from 1 for [`Page1GiB`] to 3 for [`Page4KiB`].
     *
     * The mapping frame (the [`PhysFrame`] that maps the [`VirtFrame`]
     * given) is not allocated when the given [`PTFlags`] not contains
     * [`PTFlags::PRESENT`] (useful for demand paging)
     *
     * [`VirtFrame`]: /hal/paging/type.VirtFrame.html
     * [`PageTable`]: /hal/paging/struct.PageTable.html
     * [`FrameAllocator::alloc_page_table()`]:
     * /hal/paging/trait.FrameAllocator.html#method.alloc_page_table
     * [`PageSize`]: /hal/paging/trait.PageSize.html
     * [`Page1GiB`]: /hal/paging/struct.Page1GiB.html
     * [`Page4KiB`]: /hal/paging/struct.Page4KiB.html
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     * [`PTFlags`]: /hal/paging/struct.PTFlags.html
     * [`PTFlags::PRESENT`]:
     * /hal/paging/struct.PTFlags.html#associatedconstant.PRESENT
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
            /* in DEBUG mode ensure that the caller have not made mistakes with the
             * flags and the page mapping sizes
             */
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

    /** # Maps a `Range` of `VirtFrame`s
     *
     * Creates a new range of mappings into this page directory starting
     * from the first [`VirtFrame`] address of the given [`VirtFrameRange`].
     *
     * Like [`PageDir::map_single()`], this method also creates all the
     * missing intermediate [`PageTable`]s for all the mappings using
     * [`FrameAllocator::alloc_page_table()`].
     *
     * As [`PageDir::map_single()`] do, this method will not allocate
     * mapping [`PhysFrame`]s if the given [`PTFlags`] not contain
     * [`PTFlags::PRESENT`]
     *
     * [`VirtFrame`]: /hal/paging/type.VirtFrame.html
     * [`VirtFrameRange`]: /hal/paging/type.VirtFrameRange.html
     * [`PageDir::map_single()`]:
     * /hal/paging/struct.PageDir.html#method.map_single
     * [`PageTable`]: /hal/paging/struct.PageTable.html
     * [`FrameAllocator::alloc_page_table()`]:
     * /hal/paging/trait.FrameAllocator.html#method.alloc_page_table
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     * [`PTFlags`]: /hal/paging/struct.PTFlags.html
     * [`PTFlags::PRESENT`]:
     * /hal/paging/struct.PTFlags.html#associatedconstant.PRESENT
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

    /** # Unmaps a single `VirtFrame`
     *
     * Removes the mapping for the given [`VirtFrame`] inside this page
     * directory and frees the mapping's [`PhysFrame`] if present.
     *
     * It could also collect empty intermediate [`PageTable`]s if
     * `collect_empty_page_tables = true` using
     * [`FrameAllocator::free_page_table()`].
     *
     * Please keep in mind that the same not collected intermediate
     * [`PageTable`]s because of `collect_empty_page_tables = false` could
     * be collected by a following `collect_empty_page_tables = true` call
     * that unmaps a [`VirtFrame`] that resides into the same
     * [`PageTable`]s, so if you want to keep allocated a set of page tables
     * be sure to NOT unmap near [`VirtFrame`]s with
     * `collect_empty_page_tables = true`
     *
     * [`VirtFrame`]: /hal/paging/type.VirtFrame.html
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     * [`PageTable`]: /hal/paging/struct.PageTable.html
     * [`PageTableEntry`]: /hal/paging/struct.PageTableEntry.html
     * [`FrameAllocator::free_page_table()`]:
     * /hal/paging/trait.FrameAllocator.html#method.free_page_table
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

    /** # Unmaps a `Range` of `VirtFrame`s
     *
     * Removes a existing range of mappings into this page directory
     * starting from the first [`VirtFrame`] address of the given
     * [`VirtFrameRange`] and frees all the present mapping's [`PhysFrame`].
     *
     * Like [`PageDir::unmap_single()`], this method could also collect
     * empty intermediate [`PageTable`]s if
     * `collect_empty_page_tables = true` using
     * [`FrameAllocator::free_page_table()`].
     *
     * [`VirtFrame`]: /hal/paging/type.VirtFrame.html
     * [`VirtFrameRange`]: /hal/paging/type.VirtFrameRange.html
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     * [`PageDir::unmap_single()`]:
     * /hal/paging/struct.PageDir.html#method.unmap_single
     * [`PageTable`]: /hal/paging/struct.PageTable.html
     * [`FrameAllocator::free_page_table()`]:
     * /hal/paging/trait.FrameAllocator.html#method.free_page_table
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

    /** Returns the [`PTFlags`] of the given [`VirtFrame`]
     *
     * [`PTFlags`]: /hal/paging.struct.PTFlags.html
     * [`VirtFrame`]: /hal/paging.struct.VirtFrame.html
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

    /** Makes this `PageDir` the active one
     */
    pub unsafe fn activate(&self) {
        HwPageDirSupport::activate_page_dir(self.m_root_frame);
    }

    /** Returns the root [`PhysFrame`] of this `PageDir`
     *
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     */
    pub fn root_phys_frame(&self) -> PhysFrame<Page4KiB> {
        self.m_root_frame
    }

    /** Returns the virtual to physical memory offset [`VirtAddr`]
     *
     * [`VirtAddr`]: /hal/addr/struct.VirtAddr.html
     */
    pub fn phys_mem_offset(&self) -> VirtAddr {
        self.m_phys_offset.m_offset
    }

    /** Returns the active page directory
     */
    pub unsafe fn active_page_dir(phys_offset: usize) -> PageDir {
        PageDir::new(HwPageDirSupport::active_page_dir_frame(),
                     VirtAddr::new_unchecked(phys_offset))
    }

    /** # Actually maps a `VirtFrame`
     *
     * Performs the operations to actually map the given [`VirtFrame`] to
     * this page directory according to the given [`PTFlags`]
     *
     * [`VirtFrame`]: /hal/paging/type.VirtFrame.html
     * [`PTFlags`]: /hal/paging/struct.PTFlags.html
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

    /** # Actually unmaps a `VirtFrame`
     *
     * Performs the operations to actually unmap the given [`VirtFrame`]
     * from this page directory
     *
     * [`VirtFrame`]: /hal/paging/type.VirtFrame.html
     * [`PTFlags`]: /hal/paging/struct.PTFlags.html
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

    /** # Ensures the next `PageTable`
     *
     * Ensures that the [`PageTable`] referenced by the given
     * [`PageTableEntry`] exists (it will be created otherwise) and doesn't
     * contain a mapping to a bigger physical frame
     *
     * [`PageTable`]: /hal/paging/struct.PageTable.html
     * [`PageTableEntry`]: /hal/paging/struct.PageTableEntry.html
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

    /** # Calculates the virtual address to the next `PageTable`
     *
     * Calculates the reference to the next [`PageTable`] from the given
     * [`PageTableEntry`]
     *
     * [`PageTable`]: /hal/paging/struct.PageTable.html
     * [`PageTableEntry`]: /hal/paging/struct.PageTableEntry.html
     */
    fn next_page_table<'b, S>(&self,
                              entry: &'b mut PageTableEntry)
                              -> Result<&'b mut PageTable, PageDirErr>
        where S: PageSize {
        Ok(unsafe {
            &mut *self.m_phys_offset.next_table_pointer::<S>(entry.phys_frame()?)
        })
    }

    /** # Collects unused `PageTable`s
     *
     * Recursively checks whether the [`PageTable`]s that was used to map
     * the given [`VirtFrame`] are empty, in that case deallocates them
     * returning to the [`FrameAllocator`] given
     *
     * [`PageTable`]: /hal/paging/struct.PageTable.html
     * [`VirtFrame`]: /hal/paging/type.VirtFrame.html
     * [`FrameAllocator`]: /hal/paging/trait.FrameAllocator.html
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
                if let Some(next_table_level) = pt_level.iter_from_this().next() {
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

    /** Returns the mutable reference to the Level4 [`PageTable`]
     *
     * [`PageTable`]: /hal/paging/struct.PageTable.html
     */
    fn root_page_table(&self) -> &mut PageTable {
        unsafe { &mut *self.m_phys_offset.next_table_pointer(self.m_root_frame) }
    }
}

impl Debug for PageDir {
    /** Formats the value using the given formatter
     */
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

/** # Physical Memory Offset
 *
 * Simple object that keeps the value of the virtual address on which the
 * kernel maps the physical memory
 */
struct PhysOffset {
    m_offset: VirtAddr
}

impl PhysOffset {
    /** # Constructs a `PhysOffset`
     *
     * The constructs asserts the well alignment of the given [`VirtAddr`]
     * to the [`Page4KiB`]
     *
     * [`VirtAddr`]: /hal/addr/struct.VirtAddr.html
     * [`Page4KiB`]: /hal/paging/struct.Page4kib.html
     */
    fn new(offset: VirtAddr) -> Self {
        assert!(offset.is_aligned(Page4KiB::SIZE));
        Self { m_offset: offset }
    }

    /** # Calculates next `PageTable`'s virtual address
     *
     * Using the stored virtual offset returns the virtual pointer to the
     * next [`PageTable`] on which the given [`PhysFrame`] points
     *
     * [`PageTable`]: /hal/paging/struct.PageTable.html
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     */
    unsafe fn next_table_pointer<S>(&self, phys_frame: PhysFrame<S>) -> *mut PageTable
        where S: PageSize {
        (self.m_offset + phys_frame.start_addr().as_usize()).as_ptr_mut()
    }
}

/** # Hardware Page Directory Support Base Interface
 *
 * Defines a little amount of constants and methods to support the main
 * [`PageDir`] object to perform architecture specific operations or apply
 * architecture specific values
 */
pub(crate) trait HwPageDirSupportBase {
    /* The following values are assigned with the real page table (entry)
     * flags expected by the hardware architecture, they will be used by the
     * for his published internal flags
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

    /** Returns the current [`PageDir`]'s [`PhysFrame`]
     *
     * [`PageDir`]: /hal/paging/struct.PageDir.html
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     */
    unsafe fn active_page_dir_frame() -> PhysFrame<Page4KiB>;

    /** Activates the given [`PhysFrame`] as current [`PageDir`]
     *
     * [`PhysFrame`]: /hal/paging/type.PhysFrame.html
     * [`PageDir`]: /hal/paging/struct.PageDir.html
     */
    unsafe fn activate_page_dir(phys_frame: PhysFrame<Page4KiB>);
}

c_handy_enum! {
    /** # Page Directory Errors
     *
     * Enumerates the errors that could occur when using the [`PageDir`]
     *
     * [`PageDir`]: /hal/paging/struct.PageDir.html
     */
    pub enum PageDirErr : u8 {
        PageNotMapped        = 0 => "The page requested is not mapped",
        PageAlreadyMapped    = 1 => "Mapping overlaps an already mapped page",
        EmptyRange           = 2 => "Mapping range empty",
        PhysAllocFailed      = 3 => "Physical allocator have no more frames",
        PartialHugePageUnmap = 4 => "Tried to unmap a piece of a huge frame",
    }
}

impl From<PageTableEntryErr> for PageDirErr {
    /** Performs the conversion
     */
    fn from(pte_err: PageTableEntryErr) -> Self {
        match pte_err {
            PageTableEntryErr::PhysFrameNotPresent => Self::PageNotMapped,
            PageTableEntryErr::InUseForBigFrame => Self::PageAlreadyMapped
        }
    }
}
