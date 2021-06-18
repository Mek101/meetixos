/*! Kernel caller trait */

use api_data::{
    error::OsError,
    sys::{
        fn_path::KernFnPath,
        KernHandle,
        SysCallPayload
    }
};

use crate::arch::do_syscall;

/**
 * Exports the custom result type used across all the LibApi library
 */
pub type Result<T> = core::result::Result<T, OsError>;

/**
 * Interface which gives the ability to perform system call for the structs
 * which implement it
 */
pub(crate) trait KernCaller {
    /**
     * Returns the raw underling kernel resource handle
     */
    fn raw_handle(&self) -> KernHandle;

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * without additional parameters.
     *
     * Needs the `KernHandle` of the implementor via
     * `KernCaller::raw_handle()` to execute the requested operation
     * referred to the object instance
     */
    #[inline]
    fn inst_kern_call_0(&self, kern_fn_path: KernFnPath) -> Result<usize> {
        self.inst_kern_call_6(kern_fn_path, 0, 0, 0, 0, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with one additional parameter.
     *
     * Needs the `KernHandle` of the implementor via
     * `KernCaller::raw_handle()` to execute the requested operation
     * referred to the object instance
     */
    #[inline]
    fn inst_kern_call_1(&self, kern_fn_path: KernFnPath, arg0: usize) -> Result<usize> {
        self.inst_kern_call_6(kern_fn_path, arg0, 0, 0, 0, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with two additional parameters.
     *
     * Needs the `KernHandle` of the implementor via
     * `KernCaller::raw_handle()` to execute the requested operation
     * referred to the object instance
     */
    #[inline]
    fn inst_kern_call_2(&self,
                        kern_fn_path: KernFnPath,
                        arg0: usize,
                        arg1: usize)
                        -> Result<usize> {
        self.inst_kern_call_6(kern_fn_path, arg0, arg1, 0, 0, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with three additional parameters.
     *
     * Needs the `KernHandle` of the implementor via
     * `KernCaller::raw_handle()` to execute the requested operation
     * referred to the object instance
     */
    #[inline]
    fn inst_kern_call_3(&self,
                        kern_fn_path: KernFnPath,
                        arg0: usize,
                        arg1: usize,
                        arg2: usize)
                        -> Result<usize> {
        self.inst_kern_call_6(kern_fn_path, arg0, arg1, arg2, 0, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with four additional parameters.
     *
     * Needs the `KernHandle` of the implementor via
     * `KernCaller::raw_handle()` to execute the requested operation
     * referred to the object instance
     */
    #[inline]
    fn inst_kern_call_4(&self,
                        kern_fn_path: KernFnPath,
                        arg0: usize,
                        arg1: usize,
                        arg2: usize,
                        arg3: usize)
                        -> Result<usize> {
        self.inst_kern_call_6(kern_fn_path, arg0, arg1, arg2, arg3, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with five additional parameters.
     *
     * Needs the `KernHandle` of the implementor via
     * `KernCaller::raw_handle()` to execute the requested operation
     * referred to the object instance
     */
    #[inline]
    fn inst_kern_call_5(&self,
                        kern_fn_path: KernFnPath,
                        arg0: usize,
                        arg1: usize,
                        arg2: usize,
                        arg3: usize,
                        arg4: usize)
                        -> Result<usize> {
        self.inst_kern_call_6(kern_fn_path, arg0, arg1, arg2, arg3, arg4, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with six additional parameters.
     *
     * Needs the `KernHandle` of the implementor via
     * `KernCaller::raw_handle()` to execute the requested operation
     * referred to the object instance
     */
    #[inline]
    fn inst_kern_call_6(&self,
                        kern_fn_path: KernFnPath,
                        arg0: usize,
                        arg1: usize,
                        arg2: usize,
                        arg3: usize,
                        arg4: usize,
                        arg5: usize)
                        -> Result<usize> {
        Self::do_kern_call(kern_fn_path,
                           Some(self.raw_handle()),
                           arg0,
                           arg1,
                           arg2,
                           arg3,
                           arg4,
                           arg5)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * without additional parameters
     */
    #[inline]
    fn kern_call_0(kern_fn_path: KernFnPath) -> Result<usize> {
        Self::kern_call_6(kern_fn_path, 0, 0, 0, 0, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with one additional parameter
     */
    #[inline]
    fn kern_call_1(kern_fn_path: KernFnPath, arg0: usize) -> Result<usize> {
        Self::kern_call_6(kern_fn_path, arg0, 0, 0, 0, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with two additional parameters
     */
    #[inline]
    fn kern_call_2(kern_fn_path: KernFnPath, arg0: usize, arg1: usize) -> Result<usize> {
        Self::kern_call_6(kern_fn_path, arg0, arg1, 0, 0, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with three additional parameters
     */
    #[inline]
    fn kern_call_3(kern_fn_path: KernFnPath,
                   arg0: usize,
                   arg1: usize,
                   arg2: usize)
                   -> Result<usize> {
        Self::kern_call_6(kern_fn_path, arg0, arg1, arg2, 0, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with four additional parameters
     */
    #[inline]
    fn kern_call_4(kern_fn_path: KernFnPath,
                   arg0: usize,
                   arg1: usize,
                   arg2: usize,
                   arg3: usize)
                   -> Result<usize> {
        Self::kern_call_6(kern_fn_path, arg0, arg1, arg2, arg3, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with five additional parameters
     */
    #[inline]
    fn kern_call_5(kern_fn_path: KernFnPath,
                   arg0: usize,
                   arg1: usize,
                   arg2: usize,
                   arg3: usize,
                   arg4: usize)
                   -> Result<usize> {
        Self::kern_call_6(kern_fn_path, arg0, arg1, arg2, arg3, arg4, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with six additional parameters
     */
    #[inline]
    fn kern_call_6(kern_fn_path: KernFnPath,
                   arg0: usize,
                   arg1: usize,
                   arg2: usize,
                   arg3: usize,
                   arg4: usize,
                   arg5: usize)
                   -> Result<usize> {
        Self::do_kern_call(kern_fn_path, None, arg0, arg1, arg2, arg3, arg4, arg5)
    }

    /**
     * Effectively performs the system call with the given parameters
     */
    #[inline]
    fn do_kern_call(kern_fn_path: KernFnPath,
                    raw_handle: Option<KernHandle>,
                    arg0: usize,
                    arg1: usize,
                    arg2: usize,
                    arg3: usize,
                    arg4: usize,
                    arg5: usize)
                    -> Result<usize> {
        /* prepare and fill the payload with all the given parameters */
        let mut syscall_payload = SysCallPayload::new(kern_fn_path,
                                                      raw_handle,
                                                      arg0,
                                                      arg1,
                                                      arg2,
                                                      arg3,
                                                      arg4,
                                                      arg5);

        /* perform the switch and let the kernel execute the requested routine */
        do_syscall(&mut syscall_payload);

        /* move the payload to the result */
        syscall_payload.into()
    }
}
