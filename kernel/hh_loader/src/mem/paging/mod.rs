/*! HH_Loader paging management */

use shared::{
    addr::{
        virt::VirtAddr,
        Address
    },
    mem::paging::{
        dir::PageDir,
        flush::MapFlusher,
        table::PTFlags
    }
};

use crate::mem::{
    paging::allocator::LinearAllocator,
    vm_layout::vml_core_layout
};

pub mod allocator;

/* offset of the physical memory into virtual memory */
static mut PHYS_MEM_OFFSET: Option<VirtAddr> = None;

/**
 * Maps the physical memory at the place chosen by the `vm_layout` module
 */
pub fn paging_map_phys_mem() {
    let phys_mem_mapping_area = vml_core_layout().phys_mem_mapping_area();

    /* map all the physical memory into the designed area.
     * Note that here is used mapping with huge 2MiB frames to reduce physical
     * allocation requests for intermediate page tables
     */
    let map_result =
        paging_current_page_dir().map_range(phys_mem_mapping_area.as_frame_range(),
                                            &LinearAllocator::new_zero(),
                                            PTFlags::PRESENT
                                            | PTFlags::READABLE
                                            | PTFlags::WRITEABLE
                                            | PTFlags::GLOBAL
                                            | PTFlags::HUGE_PAGE
                                            | PTFlags::NO_EXECUTE);
    match map_result {
        Ok(map_flusher) => {
            /* not strictly necessary, but just to be sure */
            map_flusher.flush()
        },
        Err(err) => {
            /* cannot continue anymore */
            panic!("Failed to map physical memory: cause: {}", err)
        }
    }

    /* store the virtual address to be accessed by paging_current_page_dir() */
    unsafe {
        PHYS_MEM_OFFSET = Some(phys_mem_mapping_area.start_addr());
    }
}

/**
 * Returns the currently active `PageDir` instance
 */
pub fn paging_current_page_dir() -> PageDir {
    let phys_mem_offset = if let Some(phys_offset) = unsafe { PHYS_MEM_OFFSET } {
        phys_offset
    } else {
        VirtAddr::new_zero()
    };

    unsafe { PageDir::active_page_dir(phys_mem_offset) }
}
