/*! # Kernel caller trait
 *
 * Implements a trait that makes an object able to perform system calls
 */

use os::{
    sysc::{
        fn_path::KernFnPath,
        id::SysCallId
    },
    syscall_0,
    syscall_1,
    syscall_2,
    syscall_3,
    syscall_4,
    syscall_5
};

use crate::errors::Error;

/** # API `Result` Alias
 *
 * Exports the custom result type used across all the api library
 */
pub type Result<T> = core::result::Result<T, Error>;

/** # Kernel Caller
 *
 * Exposes for the objects that implement it the ability to perform system
 * call to the requested kernel service encapsulating the unsafety because
 * it is ensured that changes in kernel's system call interface are released
 * with internal updated of the [`api`] crate
 *
 * When implemented this trait gives too the ability to perform instance
 * calls which are system calls referred to a particular instance of an
 * object owned by the caller thread/process in user/kernel space
 *
 * [`api`]: /api/index.html
 */
pub(crate) trait KernCaller {
    /** # Caller handle value
     *
     * Returns the upper 32bits of the 64bit identifier of a system call.
     *
     * Normally the data returned is the value of the handle of the object
     * that requests the kernel service, otherwise 0 is returned
     */
    fn caller_handle_bits(&self) -> u32 {
        0
    }

    /** # Composes the `SysCallId`
     *
     * Instantiates a [`SysCallId`] with the given parameter, the value
     * returned by the [`caller_handle_bits()`](HB) and the call class
     *
     * [`SysCallId`]: os::sysc::id::SysCallId
     * [HB]: crate::caller::KernCaller::caller_handle_bits
     */
    fn call_id(&self, id: KernFnPath) -> SysCallId {
        SysCallId::new(id, self.caller_handle_bits()).into()
    }

    /** # 0 argument system call
     *
     * Performs the call to the kernel service identified by the given
     * [`KernFnPath`] without additional arguments.
     *
     * Internally is composed the complete call identifier with
     * [`KernCaller::call_id()`]
     *
     * [`KernFnPath`]: os::sysc::fn_path::KernFnPath
     * [`KernCaller::call_id()`]: crate::caller::KernCaller::call_id
     */
    fn kern_call_0(&self, id: KernFnPath) -> Result<usize> {
        let mut error = Error::default();
        unsafe { syscall_0(self.call_id(id), error.as_ptr()).map_err(|_| error) }
    }

    /** # 1 argument system call
     *
     * Performs the call to the kernel service identified by the given
     * [`KernFnPath`] with 1 additional argument.
     *
     * Internally is composed the complete call identifier with
     * [`KernCaller::call_id()`]
     *
     * [`KernFnPath`]: os::sysc::fn_path::KernFnPath
     * [`KernCaller::call_id()`]: crate::caller::KernCaller::call_id
     */
    fn kern_call_1(&self, id: KernFnPath, a1: usize) -> Result<usize> {
        let mut error = Error::default();
        unsafe { syscall_1(self.call_id(id), a1, error.as_ptr()).map_err(|_| error) }
    }

    /** # 2 arguments system call
     *
     * Performs the call to the kernel service identified by the given
     * [`KernFnPath`] with 2 additional arguments.
     *
     * Internally is composed the complete call identifier with
     * [`KernCaller::call_id()`]
     *
     * [`KernFnPath`]: os::sysc::fn_path::KernFnPath
     * [`KernCaller::call_id()`]: crate::caller::KernCaller::call_id
     */
    fn kern_call_2(&self, id: KernFnPath, a1: usize, a2: usize) -> Result<usize> {
        let mut error = Error::default();
        unsafe { syscall_2(self.call_id(id), a1, a2, error.as_ptr()).map_err(|_| error) }
    }

    /** # 3 arguments system call
     *
     * Performs the call to the kernel service identified by the given
     * [`KernFnPath`] with 3 additional arguments.
     *
     * Internally is composed the complete call identifier with
     * [`KernCaller::call_id()`]
     *
     * [`KernFnPath`]: os::sysc::fn_path::KernFnPath
     * [`KernCaller::call_id()`]: crate::caller::KernCaller::call_id
     */
    fn kern_call_3(&self,
                   id: KernFnPath,
                   a1: usize,
                   a2: usize,
                   a3: usize)
                   -> Result<usize> {
        let mut error = Error::default();
        unsafe {
            syscall_3(self.call_id(id), a1, a2, a3, error.as_ptr()).map_err(|_| error)
        }
    }

    /** # 4 arguments system call
     *
     * Performs the call to the kernel service identified by the given
     * [`KernFnPath`] with 4 additional arguments.
     *
     * Internally is composed the complete call identifier with
     * [`KernCaller::call_id()`]
     *
     * [`KernFnPath`]: os::sysc::fn_path::KernFnPath
     * [`KernCaller::call_id()`]: crate::caller::KernCaller::call_id
     */
    fn kern_call_4(&self,
                   id: KernFnPath,
                   a1: usize,
                   a2: usize,
                   a3: usize,
                   a4: usize)
                   -> Result<usize> {
        let mut error = Error::default();
        unsafe {
            syscall_4(self.call_id(id), a1, a2, a3, a4, error.as_ptr()).map_err(|_| error)
        }
    }

    /** # 5 arguments system call
     *
     * Performs the call to the kernel service identified by the given
     * [`KernFnPath`] with 5 additional arguments.
     *
     * Internally is composed the complete call identifier with
     * [`KernCaller::call_id()`]
     *
     * [`KernFnPath`]: os::sysc::fn_path::KernFnPath
     * [`KernCaller::call_id()`]: crate::caller::KernCaller::call_id
     */
    fn kern_call_5(&self,
                   id: KernFnPath,
                   a1: usize,
                   a2: usize,
                   a3: usize,
                   a4: usize,
                   arg5: usize)
                   -> Result<usize> {
        let mut error = Error::default();
        unsafe {
            syscall_5(self.call_id(id),
                      a1,
                      a2,
                      a3,
                      a4,
                      arg5,
                      error.as_ptr()).map_err(|_| error)
        }
    }
}
