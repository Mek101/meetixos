/*! x86_64 Kernel function call */

use api_data::sys::SysCallPayload;

/**
 * Performs the `syscall` instruction to switch to the kernel with the given
 * payload
 */
#[inline(always)]
pub(crate) fn do_syscall(syscall_payload: &mut SysCallPayload) {
    asm!("syscall", in("rax") syscall_payload.as_syscall_ptr(), options(nostack));
}
