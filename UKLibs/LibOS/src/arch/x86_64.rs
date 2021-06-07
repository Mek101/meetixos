/*! x86_64 Kernel function call code
 *
 * Implements 64-bit x86_64 architecture-dependent functions to call the
 * Kernel.
 *
 * The following code is all marked as unsafe because it is not intended for
 * direct uses into user code, it is really easy to provide wrong arguments
 * count/types/sizes and cause undefined behaviours into the Kernel and into
 * the applications.
 *
 * Refer to the `LibApi` crate to use high-level interfaces to the Kernel,
 * the following code is considered internal and may change in the future
 */

use crate::sysc::id::SysCallId;

/**
 * Generates the `asm!` code to perform an `x86_64` system call
 */
macro_rules! raw_syscall {
    ($id:expr, $( $reg:tt = $val:expr),*) => {{
        let ret_val;
        let is_err: usize;

        /* load the registers and perform the System Call (syscall
         * instruction - RING0 level) to switch into the Kernel code.
         *
         * | Register | Usage
         * | rax      | <call_id>/<ret_val>
         * | rdx      | first argument
         * | rsi      | second argument
         * | rdi      | third argument
         * | r8       | fourth argument
         * | r9       | fifth argument
         * | r10      | <err_ptr_value> pointer
         * | r11      | <is_error> flag
         */
        asm!(
            "syscall",
            lateout("rax") ret_val,
            lateout("r11") is_err,
            in("rax") Into::<usize>::into($id) as usize $(, in($reg) $val)*,
            /* free the arguments registers */
            lateout("rcx") _,
            lateout("rdx") _,
            lateout("rsi") _,
            lateout("rdi") _,
            lateout("r8")  _,
            lateout("r9")  _,
            lateout("r10") _,
            options(nostack)
        );

        /* if the is_err contains a value greater than zero means that the
         * system call have returned an error, otherwise return the raw integer
         * value, it will be evaluated and mapped by the LibApi wrapper
         */
        if is_err > 0 {
            Err(())
        } else {
            Ok(ret_val)
        }
    }};
}

/**
 * Requests the Kernel's service identified by the given `SysCallId` without
 * additional arguments than `err_ptr_value`
 */
#[inline(always)]
pub unsafe fn syscall_0(id: SysCallId, err_ptr_value: *mut usize) -> Result<usize, ()> {
    raw_syscall!(id, "r10" = (err_ptr_value as usize))
}

/**
 * Requests the Kernel's service identified by the given `SysCallId` with
 * `1` additional argument than `err_ptr_value`
 */
#[inline(always)]
pub unsafe fn syscall_1(id: SysCallId,
                        a1: usize,
                        err_ptr_value: *mut usize)
                        -> Result<usize, ()> {
    raw_syscall!(id, "rdx" = a1, "r10" = (err_ptr_value as usize))
}

/**
 * Requests the Kernel's service identified by the given `SysCallId` with
 * `2` additional arguments than `err_ptr_value`
 */
#[inline(always)]
pub unsafe fn syscall_2(id: SysCallId,
                        a1: usize,
                        a2: usize,
                        err_ptr_value: *mut usize)
                        -> Result<usize, ()> {
    raw_syscall!(id, "rdx" = a1, "rsi" = a2, "r10" = (err_ptr_value as usize))
}

/**
 * Requests the Kernel's service identified by the given `SysCallId` with
 * `3` additional arguments than `err_ptr_value`
 */
#[inline(always)]
pub unsafe fn syscall_3(id: SysCallId,
                        a1: usize,
                        a2: usize,
                        a3: usize,
                        err_ptr_value: *mut usize)
                        -> Result<usize, ()> {
    raw_syscall!(id, "rdx" = a1, "rsi" = a2, "rdi" = a3, "r10" = (err_ptr_value as usize))
}

/**
 * Requests the Kernel's service identified by the given `SysCallId` with
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
                 "rdx" = a1,
                 "rsi" = a2,
                 "rdi" = a3,
                 "r8" = a4,
                 "r10" = (err_ptr_value as usize))
}

/**
 * Requests the Kernel's service identified by the given `SysCallId` with
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
                 "rdx" = a1,
                 "rsi" = a2,
                 "rdi" = a3,
                 "r8" = a4,
                 "r9" = a5,
                 "r10" = (err_ptr_value as usize))
}
