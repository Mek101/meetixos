/*! Kernel memory manager */

use core::{
    fmt,
    fmt::Debug
};

use bits::bit_fields::{
    BitArray,
    BitFields
};

use crate::{
    addr::{
        phys_addr::PhysAddr,
        Address
    },
    boot_info::BootInfo,
    dbg_print::DbgLevel,
    dbg_println,
    vm::{
        layout_manager::LayoutManager,
        page_dir::PageDir,
        Page4KiB,
        PageSize
    }
};
use helps::dbg::DisplaySizePretty;

static mut SM_MEM_MANAGER: Option<MemManager> = None;

pub struct MemManager {
    m_layout_manager: LayoutManager,
    m_phys_frames_bitmap: &'static mut [u8],
    m_mem_manager_stats: MemManagerStats,
    m_kernel_page_dir: PageDir
}

impl MemManager /* Constructors */ {
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
        let kern_text_phys_range = layout_manager.kern_text_phys_range();
        dbg_println!(DbgLevel::Trace, "kern_text_phys_range: {:?}", kern_text_phys_range);

        let mut mem_manager_stats = MemManagerStats::new();
        for phys_addr in
            boot_info.boot_mem_areas()
                     .iter()
                     .flat_map(|phys_mem_range| {
                         phys_mem_range.clone().step_by(Page4KiB::SIZE)
                     })
        {
            /* mark as available only the frames which not store the kernel text */
            if !kern_text_phys_range.contains(&phys_addr) {
                phys_frames_bitmap.set_bit(phys_addr.as_page_index::<Page4KiB>(), true);
                mem_manager_stats.m_free_phys_frames += 1;
            } else {
                mem_manager_stats.m_allocated_phys_frames += 1;
            }
        }
        dbg_println!(DbgLevel::Trace, "{:?}", mem_manager_stats);

        /* initialize the global instance */
        unsafe {
            SM_MEM_MANAGER = Some(Self { m_layout_manager: layout_manager,
                                         m_phys_frames_bitmap:
                                             phys_frames_bitmap.leak(),
                                         m_mem_manager_stats: mem_manager_stats,
                                         m_kernel_page_dir: PageDir::active() })
        }
    }
}

impl MemManager /* Getters */ {
    pub fn instance() -> &'static MemManager {
        unsafe {
            SM_MEM_MANAGER.as_ref().expect("Tried to obtain MemManager instance before \
                                            initialization")
        }
    }

    pub fn layout_manager(&self) -> &LayoutManager {
        &self.m_layout_manager
    }
}

pub struct MemManagerStats {
    m_allocated_phys_frames: usize,
    m_free_phys_frames: usize
}

impl MemManagerStats /* Constructors */ {
    fn new() -> Self {
        Self { m_allocated_phys_frames: 0,
               m_free_phys_frames: 0 }
    }
}

impl MemManagerStats /* Getters */ {
    pub fn allocated_phys_frames(&self) -> usize {
        self.m_allocated_phys_frames
    }

    pub fn free_phys_frames(&self) -> usize {
        self.m_free_phys_frames
    }
}

impl Debug for MemManagerStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "MemManagerStats {{ m_allocated_phys_frames: {} ({}), \
                m_free_phys_frames: {} ({}) }}",
               self.m_allocated_phys_frames,
               (self.m_allocated_phys_frames * Page4KiB::SIZE).display_pretty(),
               self.m_free_phys_frames,
               (self.m_free_phys_frames * Page4KiB::SIZE).display_pretty())
    }
}
