/*! Kernel memory manager */

use core::{
    fmt,
    fmt::Debug,
    sync::atomic::{
        AtomicUsize,
        Ordering
    }
};

use bits::bit_fields::{
    BitArray,
    BitFields,
    BitFindMode
};
use helps::dbg::DisplaySizePretty;
use sync::mutex::{
    spin_mutex::RawSpinMutex,
    Mutex
};

use crate::{
    addr::{
        phys_addr::PhysAddr,
        virt_addr::VirtAddr,
        Address
    },
    boot_info::BootInfo,
    dbg_print::DbgLevel,
    dbg_println,
    vm::{
        layout_manager::LayoutManager,
        page_dir::PageDir,
        page_table::{
            PageTable,
            PageTableLevel
        },
        page_table_entry::PageTableEntry,
        Page2MiB,
        Page4KiB,
        PageSize
    }
};

/* <None> until <MemManager::init_instance()> is called */
static mut SM_MEM_MANAGER: Option<MemManager> = None;

/**
 * Kernel centralized memory manager.
 *
 * This singleton is responsible to manage at physical and virtual regions
 * exposing an high-level interface to the memory management for all the
 * kernel code
 */
pub struct MemManager {
    m_layout_manager: LayoutManager,
    m_phys_frames_bitmap: Mutex<RawSpinMutex, &'static mut [u8]>,
    m_mem_manager_stats: MemManagerStats,
    m_kernel_page_dir: PageDir
}

impl MemManager /* Constructors */ {
    /**
     * Initializes the global instance which can be obtained with
     * `MemManager::instance()`
     */
    pub fn init_instance() {
        let boot_info = BootInfo::instance();

        /* obtain the last PhysAddr to know the real size of the memory */
        let last_phy_mem_addr =
            boot_info.boot_mem_areas().iter().last().expect("Missing memory maps").end;
        if last_phy_mem_addr > PhysAddr::MAX {
            dbg_println!(DbgLevel::Warn, "Exceeded physical memory limit")
        }

        /* construct the LayoutManager */
        let layout_manager =
            if let Some(_) = boot_info.cmd_line_find_arg("-plain-vm-layout") {
                dbg_println!(DbgLevel::Warn, "Disabled kernel layout randomization");
                LayoutManager::new_plain(*last_phy_mem_addr)
            } else {
                LayoutManager::new_randomized(*last_phy_mem_addr)
            };

        /* allocate the physical frames bitmap filled with zeroes */
        let phys_frames_bitmap_requested_pages =
            ((*last_phy_mem_addr / Page4KiB::SIZE / u8::BIT_LEN) + Page4KiB::MASK) >> 12;
        let mut phys_frames_bitmap =
            vec![0; phys_frames_bitmap_requested_pages * Page4KiB::SIZE];

        /* mark the available frames into the bitmap */
        let mem_manager_stats = MemManagerStats::new();
        for phys_addr in
            boot_info.boot_mem_areas()
                     .iter()
                     .flat_map(|phys_mem_range| {
                         phys_mem_range.clone().step_by(Page4KiB::SIZE)
                     })
        {
            /* mark as available only the frames which not store the kernel text */
            if !layout_manager.kern_text_phys_range().contains(&phys_addr) {
                phys_frames_bitmap.set_bit(phys_addr.as_page_index::<Page4KiB>(), true);
                mem_manager_stats.m_free_phys_frames.fetch_add(1, Ordering::Relaxed);
            } else {
                mem_manager_stats.m_allocated_phys_frames.fetch_add(1, Ordering::Relaxed);
            }
        }

        dbg_println!(DbgLevel::Trace,
                     "kern_text_phys_range: {:?}",
                     layout_manager.kern_text_phys_range());
        dbg_println!(DbgLevel::Trace, "{:?}", mem_manager_stats);

        /* initialize the global instance */
        unsafe {
            SM_MEM_MANAGER = Some(Self { m_layout_manager: layout_manager,
                                         m_phys_frames_bitmap:
                                             Mutex::const_new(phys_frames_bitmap.leak()),
                                         m_mem_manager_stats: mem_manager_stats,
                                         m_kernel_page_dir: PageDir::current() });
        }

        /* map all the memory into the layout
         * NOTE this operation is performed after the global instance initialization
         * because many part of the virtual address calculation requires the global
         * instance
         */
        Self::instance().map_physical_memory(last_phy_mem_addr);
    }
}

impl MemManager /* Methods */ {
    /**
     * Allocate a physical memory frame from the kernel pool
     */
    pub fn allocate_kernel_phys_frame(&self) -> Option<PhysAddr> {
        self.allocate_phys_frame(BitFindMode::Regular)
    }

    /**
     * Returns the mapping `PageTableEntry` for the given `VirtAddr`
     */
    pub fn ensure_page_table_entry<'a, S>(&self,
                                          page_dir: &'a PageDir,
                                          virt_addr: VirtAddr)
                                          -> Option<&'a mut PageTableEntry>
        where S: PageSize {
        if virt_addr.is_aligned(S::SIZE) {
            let l4_page_table = page_dir.root_page_table();

            /* obtain the Level3 page-table */
            let l3_page_table =
                self.ensure_next_page_table_from_level(page_dir,
                                                       virt_addr,
                                                       l4_page_table,
                                                       PageTableLevel::Root)?;

            /* obtain the Level2 page-table */
            let l2_page_table =
                self.ensure_next_page_table_from_level(page_dir,
                                                       virt_addr,
                                                       l3_page_table,
                                                       PageTableLevel::OneGiB)?;

            /* obtain the last mapping page-table level */
            let map_page_table = if S::SIZE == Page4KiB::SIZE {
                /* if a <Page4KiB> mapping is requested go a level deeper */
                self.ensure_next_page_table_from_level(page_dir,
                                                       virt_addr,
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
}

impl MemManager /* Getters */ {
    /**
     * Returns the global `MemManager` instance
     */
    pub fn instance() -> &'static MemManager {
        unsafe {
            SM_MEM_MANAGER.as_ref().expect("Tried to obtain MemManager instance before \
                                            initialization")
        }
    }

    /**
     * Returns the `LayoutManager` instance
     */
    pub fn layout_manager(&self) -> &LayoutManager {
        &self.m_layout_manager
    }
}

impl MemManager /* Privates */ {
    /**
     * Allocates the first available physical frame from the physical bitmap
     */
    fn allocate_phys_frame(&self, bit_find_mode: BitFindMode) -> Option<PhysAddr> {
        let mut unlocked_bitmap = self.m_phys_frames_bitmap.lock();

        /* find the first bit as <true>, which means available bit */
        if let Some(bit_index) = unlocked_bitmap.find_bit(true, bit_find_mode) {
            /* mark the bit as no-more available and update the statistics */
            unlocked_bitmap.set_bit(bit_index, false);
            self.m_mem_manager_stats.on_allocated_phys_frame();

            /* construct the <PhysAddr> from the frame number */
            Some(PhysAddr::from(bit_index * Page4KiB::SIZE))
        } else {
            None
        }
    }

    /**
     * Maps all the physical memory into the
     * `LayoutManager::phys_mem_mapping_range()`
     */
    fn map_physical_memory(&self, last_phys_mem_addr: PhysAddr) {
        /* obtain the virtual offset of the physical memory */
        let phys_mem_mapping_range_start =
            self.m_layout_manager.phys_mem_mapping_range().start;

        let kernel_page_dir = PageDir::pre_phys_mapping();

        /* iterate all the available frames as 2MiB frames to reduce intermediate
         * page-tables granularity and physical memory allocations
         */
        for phys_addr in (PhysAddr::null()..last_phys_mem_addr).step_by(Page2MiB::SIZE) {
            let virt_addr: VirtAddr = (*phys_addr + *phys_mem_mapping_range_start).into();

            /* obtain the page-table-entry for the current virtual-address */
            let page_table_entry =
                self.ensure_page_table_entry::<Page2MiB>(&kernel_page_dir, virt_addr)
                    .expect("Failed to map physical memory");

            /* set the flags of the entry */
            page_table_entry.set_phys_frame(phys_addr); /* sets <is_present()> too */
            page_table_entry.set_writeable(true);
            page_table_entry.set_readable(true);
            page_table_entry.set_global(true);

            /* invalidate the TLB */
            unsafe {
                asm!("invlpg [{}]", in(reg) *virt_addr, options(nostack, preserves_flags));
                // page_table_entry.invalidate_in_tlb();
            }
        }
    }

    /**
     * Ensures the next level `PageTable` for the given `VirtAddr` into the
     * given `PageDir`.
     *
     * Allocates the missing `PageTable` if necessary
     */
    fn ensure_next_page_table_from_level<'a>(&self,
                                             page_dir: &'a PageDir,
                                             virt_addr: VirtAddr,
                                             prev_table: &'a mut PageTable,
                                             page_table_level: PageTableLevel)
                                             -> Option<&'a mut PageTable> {
        /* obtain the <PageTableEntry> from the previous table */
        let page_table_entry =
            &mut prev_table[virt_addr.page_table_index(page_table_level)];

        /* allocate the next page-table if missing */
        let new_table_created = if page_table_entry.is_unused() {
            page_table_entry.set_phys_frame(self.allocate_kernel_phys_frame()?);
            page_table_entry.set_readable(true);
            page_table_entry.set_writeable(true);

            true
        } else {
            false
        };

        /* obtain the next page-table from the current entry */
        let next_page_table = unsafe { &mut *page_dir.next_page_table(page_table_entry) };

        /* clear it if it is new */
        if new_table_created {
            next_page_table.clear();
        }
        Some(next_page_table)
    }
}

/**
 * `MemManager` statistics
 */
pub struct MemManagerStats {
    m_allocated_phys_frames: AtomicUsize,
    m_free_phys_frames: AtomicUsize
}

impl MemManagerStats /* Constructors */ {
    /**
     * Constructs a zero `MemManagerStats`
     */
    fn new() -> Self {
        Self { m_allocated_phys_frames: AtomicUsize::new(0),
               m_free_phys_frames: AtomicUsize::new(0) }
    }
}

impl MemManagerStats /* Getters */ {
    /**
     * Returns the currently allocated physical frames
     */
    pub fn allocated_phys_frames(&self) -> usize {
        self.m_allocated_phys_frames.load(Ordering::Relaxed)
    }

    /**
     * Returns the currently free physical frames
     */
    pub fn free_phys_frames(&self) -> usize {
        self.m_free_phys_frames.load(Ordering::Relaxed)
    }
}

impl MemManagerStats /* Privates */ {
    /**
     * Updates the counters when a physical frame is being allocated
     */
    fn on_allocated_phys_frame(&self) {
        self.m_free_phys_frames.fetch_sub(1, Ordering::SeqCst);
        self.m_allocated_phys_frames.fetch_add(1, Ordering::SeqCst);
    }

    /**
     * Updates the counters when a physical frame is being freed
     */
    fn on_free_phys_frame(&self) {
        self.m_allocated_phys_frames.fetch_sub(1, Ordering::SeqCst);
        self.m_free_phys_frames.fetch_add(1, Ordering::SeqCst);
    }
}

impl Debug for MemManagerStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "MemManagerStats {{ m_allocated_phys_frames: {} ({}), \
                m_free_phys_frames: {} ({}) }}",
               self.allocated_phys_frames(),
               (self.allocated_phys_frames() * Page4KiB::SIZE).display_pretty(),
               self.free_phys_frames(),
               (self.free_phys_frames() * Page4KiB::SIZE).display_pretty())
    }
}
