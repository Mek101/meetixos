/*! Virtual memory layout randomizer */

use shared::{
    addr::{
        virt::VirtAddr,
        Address
    },
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

static mut KERNEL_VM_LAYOUT: Option<VMLayout> = None;

pub fn vml_randomize_core_layout(necessary_bitmap_pages: usize) {
    assert!(unsafe { KERNEL_VM_LAYOUT.is_none() });

    let vm_layout = VMLayout::new(VMLayoutArea::new_zero(),
                                  VMLayoutArea::new_zero(),
                                  VMLayoutArea::new(VirtAddr::new(0xC0000000),
                                                    necessary_bitmap_pages
                                                    * Page4KiB::SIZE),
                                  VMLayoutArea::new_zero(),
                                  VMLayoutArea::new_zero(),
                                  VMLayoutArea::new_zero());

    info!("\n{}", vm_layout);

    unsafe {
        KERNEL_VM_LAYOUT = Some(vm_layout);
    }
}

pub fn vml_core_layout() -> &'static VMLayout {
    assert!(unsafe { KERNEL_VM_LAYOUT.is_some() });

    unsafe { KERNEL_VM_LAYOUT.as_ref().unwrap() }
}
