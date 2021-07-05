/*! aarch64 Kernel function call */

use api_data::sys::{
    AsSysCallPtr,
    SysCallPayload
};

/**
 * Performs the `svc0` instruction to switch to the kernel with the given
 * payload
 */
#[inline(always)]
pub(crate) fn do_syscall(syscall_payload: &mut SysCallPayload) {
    unsafe {
        asm!("svc #0", in("x0") syscall_payload.as_syscall_ptr());
    }
}
