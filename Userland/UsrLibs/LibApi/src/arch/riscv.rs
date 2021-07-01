/*! riscv Kernel function call */

use api_data::sys::SysCallPayload;

/**
 * Performs the `ecall` instruction to switch to the kernel with the given
 * payload
 */
#[inline(always)]
pub(crate) fn do_syscall(syscall_payload: &mut SysCallPayload) {
    asm!("ecall", in("a0") syscall_payload.as_syscall_ptr());
}
