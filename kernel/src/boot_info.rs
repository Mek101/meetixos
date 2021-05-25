/*! Higher half loader information management */

use shared::info::descriptor::LoaderInfo;

static mut BOOT_INFO: Option<LoaderInfo> = None;

pub fn boot_info_init(loader_info_ptr: *const LoaderInfo) {
    unsafe {
        let loader_info = loader_info_ptr.read();

        BOOT_INFO = Some(loader_info);
    }
}

pub fn boot_info() -> &'static LoaderInfo {
    unsafe {
        assert!(BOOT_INFO.is_some());

        BOOT_INFO.as_ref().unwrap()
    }
}
