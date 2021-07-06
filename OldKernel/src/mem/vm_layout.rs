/*! OldKernel virtual memory layout */

use shared::info::{
    descriptor::LoaderInfo,
    vm_layout::VMLayout
};

static mut VM_LAYOUT: Option<VMLayout> = None;

/**
 * Initializes the global `VMLayout` from the given `LoaderInfo`
 */
pub fn vml_init_from_loader_info(loader_info: &LoaderInfo) {
    unsafe {
        VM_LAYOUT = Some(loader_info.vm_layout().clone());
    }
}

/**
 * Returns the reference to the global `VMLayout` instance
 */
pub fn vml_layout() -> &'static VMLayout {
    unsafe {
        assert!(VM_LAYOUT.is_some());

        VM_LAYOUT.as_ref().unwrap()
    }
}
