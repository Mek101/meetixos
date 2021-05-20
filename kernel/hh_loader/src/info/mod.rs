/*! Boot information management */

use crate::{
    arch::info::HwBootInfo,
    info::info::{
        BootInfo,
        HwBootInfoBase
    }
};

pub mod info;
pub mod mem_area;

/* None until <boot_info_init()> is called */
static mut BOOT_INFORMATION: Option<BootInfo> = None;

/**
 * Initializes the global `BootInfo` descriptor from the bootloader
 */
pub fn boot_info_init(raw_boot_info_ptr: *const u8) {
    /* demand the work to the architecture dependent implementation */
    let boot_info = HwBootInfo::obtain_from_arch_info(raw_boot_info_ptr);

    /* store the value into the global value */
    unsafe {
        BOOT_INFORMATION = Some(boot_info);
    }
}

/**
 * Returns the global reference to the `BootInfo` structure
 */
pub fn boot_info() -> &'static BootInfo {
    assert!(unsafe { BOOT_INFORMATION.is_some() });

    unsafe { BOOT_INFORMATION.as_ref().unwrap() }
}
