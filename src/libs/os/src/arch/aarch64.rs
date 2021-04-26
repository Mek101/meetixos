/*! # Architecture dependent code for aarch64
 *
 * Implements 64-bit ARM architecture-dependent functions to call the
 * kernel.
 *
 * The following code is all marked as unsafe because it is not intended for
 * direct uses into user code, it is really easy to provide wrong arguments
 * count/types/sizes and cause undefined behaviours into the kernel and into
 * the applications.
 *
 * Refer to the [`api`] crate to use high-level interfaces to the kernel,
 * the following code is considered internal and may change in the future
 *
 * [`api`]: /api/index.html
 */

use crate::sysc::id::SysCallId;

/** # Raw syscall function
 *
 * Generates the `asm!` code to perform a system call
 */
macro_rules! raw_syscall {
    ($id:expr, $( $reg:tt = $val:expr),*) => {{
        let ret_val;
        let is_err: usize;

        /* load the registers and perform the SuperVisor Call (svc
         * instruction - EL1 level) to switch into the kernel code.
         *
         * | Register | Usage
         * | x0       | <call_id>/<ret_val>
         * | x1       | first argument
         * | x2       | second argument
         * | x3       | third argument
         * | x4       | fourth argument
         * | x5       | fifth argument
         * | x6       | <err_ptr_value> pointer
         * | x7       | <is_error> flag
         */
        asm!(
            "svc #0",
            lateout("x0") ret_val,
            lateout("x7") is_err,
            in("x0") Into::<usize>::into($id) $(, in($reg) $val)*,
            /* free the arguments registers */
            lateout("x1") _,
            lateout("x2") _,
            lateout("x3") _,
            lateout("x4") _,
            lateout("x5") _,
            lateout("x6") _
        );

        /* if the is_err contains a value greater than zero means that the
         * system call have returned an error, otherwise return the raw integer
         * value, it will be evaluated and mapped by the api wrapper
         */
        if is_err > 0 {
            Err(())
        } else {
            Ok(ret_val)
        }
    }};
}

/** # 0 argument system call
 *
 * Performs a switch to the kernel code to use a kernel service identified
 * by the given [`SysCallId`].
 *
 * No additional arguments than the [`SysCallId`] and the `err_ptr_value`
 * are given to the kernel
 *
 * [`SysCallId`]: /os/sysc/id/struct.SysCallId.html
 */
#[inline(always)]
pub unsafe fn syscall_0(id: SysCallId, err_ptr_value: *mut usize) -> Result<usize, ()> {
    raw_syscall!(id, "x6" = (err_ptr_value as usize))
}

/** # 1 argument system call
 *
 * Performs a switch to the kernel code to use a kernel service identified
 * by the given [`SysCallId`].
 *
 * One additional argument than the [`SysCallId`] and the `err_ptr_value`
 * is given to the kernel
 *
 * [`SysCallId`]: /os/sysc/id/struct.SysCallId.html
 */
#[inline(always)]
pub unsafe fn syscall_1(id: SysCallId,
                        a1: usize,
                        err_ptr_value: *mut usize)
                        -> Result<usize, ()> {
    raw_syscall!(id, "x1" = a1, "x6" = (err_ptr_value as usize))
}

/** # 2 argument system call
 *
 * Performs a switch to the kernel code to use a kernel service identified
 * by the given [`SysCallId`].
 *
 * Two additional arguments than the [`SysCallId`] and the `err_ptr_value`
 * are given to the kernel
 *
 * [`SysCallId`]: /os/sysc/id/struct.SysCallId.html
 */
#[inline(always)]
pub unsafe fn syscall_2(id: SysCallId,
                        a1: usize,
                        a2: usize,
                        err_ptr_value: *mut usize)
                        -> Result<usize, ()> {
    raw_syscall!(id, "x1" = a1, "x2" = a2, "x6" = (err_ptr_value as usize))
}

/** # 3 argument system call
 *
 * Performs a switch to the kernel code to use a kernel service identified
 * by the given [`SysCallId`].
 *
 * Three additional arguments than the [`SysCallId`] and the `err_ptr_value`
 * are given to the kernel
 *
 * [`SysCallId`]: /os/sysc/id/struct.SysCallId.html
 */
#[inline(always)]
pub unsafe fn syscall_3(id: SysCallId,
                        a1: usize,
                        a2: usize,
                        a3: usize,
                        err_ptr_value: *mut usize)
                        -> Result<usize, ()> {
    raw_syscall!(id, "x1" = a1, "x2" = a2, "x3" = a3, "x6" = (err_ptr_value as usize))
}

/** # 4 argument system call
 *
 * Performs a switch to the kernel code to use a kernel service identified
 * by the given [`SysCallId`].
 *
 * Four additional arguments than the [`SysCallId`] and the `err_ptr_value`
 * are given to the kernel
 *
 * [`SysCallId`]: /os/sysc/id/struct.SysCallId.html
 */
#[inline(always)]
pub unsafe fn syscall_4(id: SysCallId,
                        a1: usize,
                        a2: usize,
                        a3: usize,
                        a4: usize,
                        err_ptr_value: *mut usize)
                        -> Result<usize, ()> {
    raw_syscall!(id,
                 "x1" = a1,
                 "x2" = a2,
                 "x3" = a3,
                 "x4" = a4,
                 "x6" = (err_ptr_value as usize))
}

/** # 5 argument system call
 *
 * Performs a switch to the kernel code to use a kernel service identified
 * by the given [`SysCallId`].
 *
 * Five additional arguments than the [`SysCallId`] and the `err_ptr_value`
 * are given to the kernel
 *
 * [`SysCallId`]: /os/sysc/id/struct.SysCallId.html
 */
#[inline(always)]
pub unsafe fn syscall_5(id: SysCallId,
                        a1: usize,
                        a2: usize,
                        a3: usize,
                        a4: usize,
                        a5: usize,
                        err_ptr_value: *mut usize)
                        -> Result<usize, ()> {
    raw_syscall!(id,
                 "x1" = a1,
                 "x2" = a2,
                 "x3" = a3,
                 "x4" = a4,
                 "x5" = a5,
                 "x6" = (err_ptr_value as usize))
}
