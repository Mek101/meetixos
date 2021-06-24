/*! riscv Kernel function call */

use api_data::sys::SysCallPayload;

/**
 * Performs the `ecall` instruction to switch to the kernel with the given
 * payload
 */
#[inline(always)]
pub(crate) fn do_syscall(payload: &mut SysCallPayload) {
    asm!("ecall", in("a0") payload.as_syscall_ptr());
}
