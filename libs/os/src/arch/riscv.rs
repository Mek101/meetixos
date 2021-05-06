/*! RiscV kernel function call code
 *
 * Implements 64-bit riscv architecture-dependent functions to call the
 * kernel.
 *
 * The following code is all marked as unsafe because it is not intended for
 * direct uses into user code, it is really easy to provide wrong arguments
 * count/types/sizes and cause undefined behaviours into the kernel and into
 * the applications.
 *
 * Refer to the `api` crate to use high-level interfaces to the kernel,
 * the following code is considered internal and may change in the future
 */

use crate::sysc::id::SysCallId;

/**
 * Generates the `asm!` code to perform a `riscv` system call
 */
macro_rules! raw_syscall {
    ($id:expr, $( $reg:tt = $val:expr),*) => {{
        let ret_val;
        let is_err: usize;

        /* load the registers and perform the Environment Call (ecall
         * instruction - supervisor level) to switch into the kernel code.
         *
         * | Register | Usage
         * | a0       | <call_id>/<ret_val>
         * | a1       | first argument
         * | a2       | second argument
         * | a3       | third argument
         * | a4       | fourth argument
         * | a5       | fifth argument
         * | a6       | <err_ptr_value> pointer
         * | a7       | <is_error> flag
         */
        asm!(
            "ecall",
            lateout("a0") ret_val,
            lateout("a7") is_err,
            in("a0") Into::<usize>::into($id) $(, in($reg) $val)*,
            /* free the arguments registers */
            lateout("a1") _,
            lateout("a2") _,
            lateout("a3") _,
            lateout("a4") _,
            lateout("a6") _
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

/**
 * Requests the kernel's service identified by the given `SysCallId` without
 * additional arguments than `err_ptr_value`
 */
#[inline(always)]
pub unsafe fn syscall_0(id: SysCallId, err_ptr_value: *mut usize) -> Result<usize, ()> {
    raw_syscall!(id, "a6" = (err_ptr_value as usize))
}

/**
 * Requests the kernel's service identified by the given `SysCallId` with
 * `1` additional argument than `err_ptr_value`
 */
#[inline(always)]
pub unsafe fn syscall_1(id: SysCallId,
                        a1: usize,
                        err_ptr_value: *mut usize)
                        -> Result<usize, ()> {
    raw_syscall!(id, "a1" = a1, "a6" = (err_ptr_value as usize))
}

/**
 * Requests the kernel's service identified by the given `SysCallId` with
 * `2` additional arguments than `err_ptr_value`
 */
#[inline(always)]
pub unsafe fn syscall_2(id: SysCallId,
                        a1: usize,
                        a2: usize,
                        err_ptr_value: *mut usize)
                        -> Result<usize, ()> {
    raw_syscall!(id, "a1" = a1, "a2" = a2, "a6" = (err_ptr_value as usize))
}

/**
 * Requests the kernel's service identified by the given `SysCallId` with
 * `3` additional arguments than `err_ptr_value`
 */
#[inline(always)]
pub unsafe fn syscall_3(id: SysCallId,
                        a1: usize,
                        a2: usize,
                        a3: usize,
                        err_ptr_value: *mut usize)
                        -> Result<usize, ()> {
    raw_syscall!(id, "a1" = a1, "a2" = a2, "a3" = a3, "a6" = (err_ptr_value as usize))
}

/**
 * Requests the kernel's service identified by the given `SysCallId` with
 * `4` additional arguments than `err_ptr_value`
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
                 "a1" = a1,
                 "a2" = a2,
                 "a3" = a3,
                 "a4" = a4,
                 "a6" = (err_ptr_value as usize))
}

/**
 * Requests the kernel's service identified by the given `SysCallId` with
 * `5` additional arguments than `err_ptr_value`
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
                 "a1" = a1,
                 "a2" = a2,
                 "a3" = a3,
                 "a4" = a4,
                 "r5" = a5,
                 "a6" = (err_ptr_value as usize))
}
