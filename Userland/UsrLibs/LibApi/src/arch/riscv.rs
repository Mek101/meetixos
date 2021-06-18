/*! riscv Kernel function call */

use api_data::sys::{
    id::SysCallId,
    SysCallPayload
};

/**
 * Performs the necessary instructions to switch to the kernel mode with the
 * given `SysCallPayload` and execute the requested kernel function
 */
#[inline(always)]
pub(crate) fn do_syscall(payload: &mut SysCallPayload) {
    asm!("ecall", in("a0") payload.as_syscall_ptr());
}
