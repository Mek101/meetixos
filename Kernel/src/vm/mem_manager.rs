/*! Kernel memory manager */

use crate::{
    addr::{
        phys_addr::PhysAddr,
        Address
    },
    boot_info::BootInfo,
    dbg::print::DbgLevel,
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
    m_phys_frames_bitmap: &'static [u8]
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
            if let Some(_) = boot_info.cmd_line_find_arg("--unordered-vm-layout") {
                LayoutManager::new_randomized(*last_phy_mem_addr)
            } else {
                LayoutManager::new_plain(*last_phy_mem_addr)
            };
    }
}
