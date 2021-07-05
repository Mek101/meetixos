/*! riscv Kernel function call */

use api_data::sys::{
    AsSysCallPtr,
    SysCallPayload
};

/**
 * Performs the `ecall` instruction to switch to the kernel with the given
 * payload
 */
#[inline(always)]
pub(crate) fn do_syscall(syscall_payload: &mut SysCallPayload) {
    unsafe {
        asm!("ecall", in("a0") syscall_payload.as_syscall_ptr());
    }
}
