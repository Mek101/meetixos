/*! Kernel memory manager */

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
        paging::{
            Page4KiB,
            PageSize
        }
    }
};

static mut SM_MEM_MANAGER: Option<MemManager> = None;

pub struct MemManager {
    m_layout_manager: LayoutManager,
    m_phys_frames_bitmap: &'static mut [u8]
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
        for phys_addr in
            boot_info.boot_mem_areas()
                     .iter()
                     .flat_map(|phys_mem_range| {
                         phys_mem_range.clone().step_by(Page4KiB::SIZE)
                     })
        {
            phys_frames_bitmap.set_bit(*phys_addr / Page4KiB::SIZE, true);
        }

        /* initialize the global instance */
        unsafe {
            SM_MEM_MANAGER = Some(Self { m_layout_manager: layout_manager,
                                         m_phys_frames_bitmap:
                                             phys_frames_bitmap.leak() })
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
