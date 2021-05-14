/*! Virtual memory layout randomizer */

use shared::{
    addr::{
        virt::VirtAddr,
        Address
    },
    dbg::dbg_display_size,
    infos::vm_layout::{
        VMLayout,
        VMLayoutArea
    },
    logger::info,
    mem::paging::{
        Page4KiB,
        PageSize
    }
};

use crate::loader::loader_core_preload_cache;

/* kernel space begin..end address (192TiB..256TiB) */
const KERNEL_SPACE_BEGIN: usize = 0x0000_c000_0000_0000;
const KERNEL_SPACE_END: usize = 0x0000_ffff_ffff_ffff;

/* global layout returned by <vml_core_layout()> */
static mut KERNEL_VM_LAYOUT: Option<VMLayout> = None;

/**
 * Randomizes the virtual memory layout for the kernel's core
 */
pub fn vml_randomize_core_layout(necessary_bitmap_pages: usize) {
    assert!(unsafe { KERNEL_VM_LAYOUT.is_none() });

    /* construct the VM area for the kernel text. This is not relocatable */
    let kern_text_area = {
        let preload_cache = loader_core_preload_cache();

        VMLayoutArea::new(preload_cache.load_address(), preload_cache.load_size())
    };

    let phys_mem_bitmap_area = {
        let start_addr =
            kern_text_area.start_addr() - necessary_bitmap_pages * Page4KiB::SIZE;
        let area_size = necessary_bitmap_pages * Page4KiB::SIZE;

        VMLayoutArea::new(start_addr, area_size)
    };

    let vm_layout = VMLayout::new(kern_text_area,
                                  VMLayoutArea::new_zero(),
                                  phys_mem_bitmap_area,
                                  VMLayoutArea::new_zero(),
                                  VMLayoutArea::new_zero(),
                                  VMLayoutArea::new_zero());

    info!("Kernel Space: {:x}..{:x} ({})",
          VirtAddr::new(KERNEL_SPACE_BEGIN),
          VirtAddr::new(KERNEL_SPACE_END),
          dbg_display_size(KERNEL_SPACE_END - KERNEL_SPACE_BEGIN));
    info!("\n{}", vm_layout);

    unsafe {
        KERNEL_VM_LAYOUT = Some(vm_layout);
    }
}

pub fn vml_core_layout() -> &'static VMLayout {
    assert!(unsafe { KERNEL_VM_LAYOUT.is_some() });

    unsafe { KERNEL_VM_LAYOUT.as_ref().unwrap() }
}
