/*! Virtual memory layout randomizer */

use core::str::FromStr;

use shared::{
    addr::{
        virt::VirtAddr,
        Address
    },
    dbg::dbg_display_size,
    infos::{
        info::BootInfos,
        vm_layout::{
            VMLayout,
            VMLayoutArea
        }
    },
    logger::info,
    mem::paging::{
        Page2MiB,
        Page4KiB,
        PageSize
    },
    random::Random
};

use crate::loader::loader_core_preload_cache;

/* kernel space begin..end address (192TiB..256TiB) */
const KERN_SPACE_BEGIN: usize = 0x0000_c000_0000_0000;
const KERN_SPACE_END: usize = 0x0000_ffff_ffff_ffff;

/* global layout returned by <vml_core_layout()> */
static mut KERNEL_VM_LAYOUT: Option<VMLayout> = None;

/**
 * Randomizes the virtual memory layout for the kernel's core
 */
pub fn vml_randomize_core_layout(necessary_bitmap_pages: usize) {
    assert!(unsafe { KERNEL_VM_LAYOUT.is_none() });

    /* use random generator */
    let random = Random::new();

    /* construct the VM area for the kernel text. This is not relocatable */
    let kern_text_area = {
        let preload_cache = loader_core_preload_cache();

        VMLayoutArea::new(preload_cache.load_address(), preload_cache.load_size())
    };

    /* construct the VM area for the physical memory bitmap */
    let phys_mem_bitmap_area = {
        /* randomize a boolean value to choose whether the loader must place the
         * physical memory bitmap at the beginning of the kernel space or right
         * before the kernel text
         */
        let start_addr = if random.randomize_bool() {
            kern_text_area.start_addr() - necessary_bitmap_pages * Page4KiB::SIZE
        } else {
            VirtAddr::new(KERN_SPACE_BEGIN)
        };

        VMLayoutArea::new(start_addr, necessary_bitmap_pages * Page4KiB::SIZE)
    };

    let kern_stack_area = {
        let stack_size = BootInfos::obtain().cmdline_args()
                                            .find_key("-kern-stack-size")
                                            .map(|cmdline_arg| cmdline_arg.value())
                                            .map(|str_value| {
                                                usize::from_str(str_value)
                                                      .expect("Invalid '-kern-stack-size' value")
                                            })
                                            .unwrap_or(Page2MiB::SIZE);
    };

    let vm_layout = VMLayout::new(kern_text_area,
                                  VMLayoutArea::new_zero(),
                                  VMLayoutArea::new_zero(),
                                  phys_mem_bitmap_area,
                                  VMLayoutArea::new_zero(),
                                  VMLayoutArea::new_zero(),
                                  VMLayoutArea::new_zero());

    info!("Kernel Space: {:x}..{:x} ({})",
          VirtAddr::new(KERN_SPACE_BEGIN),
          VirtAddr::new(KERN_SPACE_END),
          dbg_display_size(KERN_SPACE_END - KERN_SPACE_BEGIN));
    info!("\n{}", vm_layout);

    unsafe {
        KERNEL_VM_LAYOUT = Some(vm_layout);
    }
}

pub fn vml_core_layout() -> &'static VMLayout {
    assert!(unsafe { KERNEL_VM_LAYOUT.is_some() });

    unsafe { KERNEL_VM_LAYOUT.as_ref().unwrap() }
}
