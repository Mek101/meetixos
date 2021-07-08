/*! Boot information management */

use shared::{
    info::descriptor::LoaderInfo,
    mem::paging::{
        Page4KiB,
        PageSize
    }
};

use crate::{
    arch::info::HwBootInfo,
    info::info::{
        BootInfo,
        HwBootInfoBase
    },
    mem::{
        phys::phys_allocated_memory,
        vm_layout::vml_core_layout
    },
    symbols::kernel_symbols
};

pub mod info;
pub mod mem_area;

/* None until <boot_info_init()> is called */
static mut BOOT_INFO: Option<BootInfo> = None;

/* None until <info_prepare_loader_info()> is called */
static mut LOADER_INFO: Option<LoaderInfo> = None;

/**
 * Initializes the global `BootInfo` descriptor from the bootloader
 */
pub fn info_init_boot_info(raw_boot_info_ptr: *const u8) {
    /* demand the work to the architecture dependent implementation */
    let boot_info = HwBootInfo::obtain_from_arch_info(raw_boot_info_ptr);

    /* store the value into the global value */
    unsafe {
        BOOT_INFO = Some(boot_info);
    }
}

/**
 * Prepares the `LoaderInfo` structure
 */
pub fn info_prepare_loader_info() -> &'static LoaderInfo {
    /* fill the <LoaderInfo> instance from the <BootInfo> data */
    let boot_info = boot_info();
    let loader_info = LoaderInfo::new(boot_info.cmdline_args().clone(),
                                      vml_core_layout().clone(),
                                      phys_allocated_memory() / Page4KiB::SIZE,
                                      boot_info.loader_reserved_range().clone(),
                                      boot_info.loader_mapped_range().clone(),
                                      kernel_symbols(),
                                      boot_info.bootloader_name());

    /* store the loader boot into the global value and return the reference */
    unsafe {
        LOADER_INFO = Some(loader_info);
        LOADER_INFO.as_ref().unwrap()
    }
}

/**
 * Returns the global reference to the `BootInfo` structure
 */
pub fn boot_info() -> &'static BootInfo {
    assert!(unsafe { BOOT_INFO.is_some() });

    unsafe { BOOT_INFO.as_ref().unwrap() }
}
