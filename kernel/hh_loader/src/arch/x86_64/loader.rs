/*! x86_64 loader implementation */

use shared::{
    addr::virt::VirtAddr,
    info::descriptor::LoaderInfo
};

pub unsafe fn arch_loader_switch_to_kernel(_stack_top: VirtAddr,
                                           _loader_info_ptr: *const LoaderInfo,
                                           _core_entry_point: VirtAddr)
                                           -> ! {
    shared::logger::debug!("Switching to kernel...");
    loop {}
}
