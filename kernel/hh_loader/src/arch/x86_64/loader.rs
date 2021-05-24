/*! x86_64 loader implementation */

use shared::addr::{
    virt::VirtAddr,
    Address
};

/**
 * x86_64 implementation for the loader context switch
 */
pub unsafe fn arch_loader_switch_to_kernel(stack_top: VirtAddr,
                                           loader_info_ptr: VirtAddr,
                                           core_entry_point: VirtAddr) {
    asm!("mov rsp, {}; mov rbp, rsp; push 0; jmp {}",
         in(reg) stack_top.as_usize(),
         in(reg) core_entry_point.as_usize(),
         in("rdi") loader_info_ptr.as_usize());
}
