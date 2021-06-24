/*! aarch64 Kernel function call */

use api_data::sys::SysCallPayload;

/**
 * Performs the `svc0` instruction to switch to the kernel with the given
 * payload
 */
#[inline(always)]
pub(crate) fn do_syscall(payload: &mut SysCallPayload) {
    asm!("svc #0", in("x0") payload.as_syscall_ptr());
}
