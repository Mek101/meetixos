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
    BitFindMode,
    TBitArray,
    TBitFields
};
use helps::dbg::TDisplaySizePretty;
use sync::SpinMutex;

use crate::{
    addr::{
        phys_addr::PhysAddr,
        TAddress
    },
    boot_info::BootInfo,
    dbg_print::DbgLevel,
    dbg_println,
    vm::{
        layout_manager::LayoutManager,
        page_dir::PageDir,
        page_table::PageTableIndex,
        Page2MiB,
        Page4KiB,
        TPageSize
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
    m_phys_frames_bitmap: SpinMutex<&'static mut [u8]>,
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
        let last_phys_mem_addr =
            boot_info.boot_mem_areas().iter().last().expect("Missing memory maps").end;
        if last_phys_mem_addr > PhysAddr::MAX {
            dbg_println!(DbgLevel::Warn, "Exceeded physical memory limit")
        }

        /* construct the LayoutManager */
        let layout_manager = if boot_info.cmd_line_arg_exists("-plain-vm-layout") {
            dbg_println!(DbgLevel::Warn, "Disabled kernel layout randomization");
            LayoutManager::new_plain(*last_phys_mem_addr)
        } else {
            LayoutManager::new_randomized(*last_phys_mem_addr)
        };

        /* allocate the physical frames bitmap filled with zeroes */
        let phys_frames_bitmap_requested_pages =
            ((*last_phys_mem_addr / Page4KiB::SIZE / u8::BIT_LEN) + Page4KiB::MASK) >> 12;
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
        let mm_inst = unsafe {
            SM_MEM_MANAGER =
                Some(Self { m_layout_manager: layout_manager,
                            m_phys_frames_bitmap:
                                SpinMutex::const_new(phys_frames_bitmap.leak()),
                            m_mem_manager_stats: mem_manager_stats,
                            m_kernel_page_dir: PageDir::pre_phys_mapping() });
            SM_MEM_MANAGER.as_mut().unwrap()
        };

        /* unmap lower-half, map the physical memory and protect the kernel image */
        mm_inst.map_physical_memory(last_phys_mem_addr);
        mm_inst.update_kernel_page_dir_after_phys_mapping();
        mm_inst.unmap_kernel_lower_half();
        mm_inst.protect_kernel_image();
    }
}

impl MemManager /* Methods */ {
    /**
     * Allocate a physical memory frame from the kernel pool
     */
    pub fn allocate_kernel_phys_frame(&self) -> Option<PhysAddr> {
        self.allocate_phys_frame(BitFindMode::Regular)
    }
}

impl MemManager /* Getters */ {
    /**
     * Returns the global `MemManager` instance
     */
    pub fn instance() -> &'static Self {
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

    /**
     * Returns the reference to the kernel `PageDir`
     */
    pub fn kernel_page_dir(&self) -> &PageDir {
        &self.m_kernel_page_dir
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
     * Unmaps the kernel lower-half mapping
     */
    fn unmap_kernel_lower_half(&self) {
        /* the page-table index 0 is used to map the L4 & L3 page-table */
        let index_zero = PageTableIndex::from(0usize);

        /* obtain the L4 & L3 page-table */
        let l4_page_table = self.kernel_page_dir().root_page_table();
        let l3_page_table = unsafe {
            self.kernel_page_dir().next_page_table(&mut l4_page_table[index_zero])
        };

        /* unmap the first entry for each table */
        l3_page_table[index_zero].set_unused();
        l4_page_table[index_zero].set_unused();
    }

    /**
     * Maps all the physical memory into the
     * `LayoutManager::phys_mem_mapping_range()`
     */
    fn map_physical_memory(&self, last_phys_mem_addr: PhysAddr) {
        /* iterate all the available frames as 2MiB frames to reduce intermediate
         * page-tables granularity and physical memory allocations.
         * In this stage, when this method is called, the <m_kernel_page_dir> doesn't
         * use the real mapped offset, because the memory is not mapped yet
         */
        for phys_addr in (PhysAddr::null()..last_phys_mem_addr).step_by(Page2MiB::SIZE) {
            let virt_addr = self.layout_manager().phys_addr_to_virt_addr(phys_addr);

            /* obtain the page-table-entry for the current virtual-address */
            let mut page_table_mapping =
                self.kernel_page_dir()
                    .ensure_page_table_entry::<Page2MiB>(virt_addr)
                    .expect("Failed to map physical memory");

            /* fill the bitflags */
            page_table_mapping.set_phys_frame(phys_addr)
                              .set_huge_page(true)
                              .set_present(true)
                              .set_readable(true)
                              .set_writeable(true)
                              .set_global(true)
                              .set_user(false);
        }
    }

    /**
     * Updates the kernel `PageDir` with same instance which resolves the
     * virtual -> physical addresses using the memory mapping
     */
    fn update_kernel_page_dir_after_phys_mapping(&mut self) {
        self.m_kernel_page_dir = PageDir::current();
    }

    /**
     * Protects the kernel image with proper protection
     */
    fn protect_kernel_image(&self) {
        dbg_println!(DbgLevel::Warn, "TODO protected kernel image!");
        dbg_println!(DbgLevel::Debug, "PageDir:\n{:?}", self.kernel_page_dir());
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
