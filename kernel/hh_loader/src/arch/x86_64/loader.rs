/*! x86_64 loader implementation */

use shared::{
    addr::{
        virt::VirtAddr,
        Address
    },
    info::descriptor::LoaderInfo,
    logger::debug
};

/**
 * x86_64 implementation for the loader context switch
 */
pub unsafe fn arch_loader_switch_to_kernel(stack_top: VirtAddr,
                                           loader_info_ptr: *const LoaderInfo,
                                           core_entry_point: VirtAddr) {
    debug!("Switching to kernel: {:x} : {:x}...", core_entry_point, stack_top);
    debug!("\n{:?}", crate::mem::paging::paging_current_page_dir());
    asm!("mov       rsp, {};\
          mov       rbp, rsp;\
          jmp       {}",
         in(reg) stack_top.as_usize(),
         in(reg) core_entry_point.as_usize(),
         in("rdi") loader_info_ptr as usize);
}
