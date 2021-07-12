/*! Base kernel resource handle */

use api_data::{
    error::OsError,
    sys::{
        codes::KernHandleFnId,
        fn_path::KernFnPath,
        RawKernHandle,
        SysCallPayload,
        INVALID_KERN_HANDLE
    }
};

use crate::arch::do_syscall;

/**
 * Exports the custom result type used across all the LibApi library
 */
pub type Result<T> = core::result::Result<T, OsError>;

/**
 * Generic reference to a kernel resource
 */
#[repr(transparent)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct KernHandle {
    m_raw_handle: RawKernHandle
}

impl KernHandle /* Constructors */ {
    /**
     * Constructs a `KernHandle` from the `raw_handle` value given
     */
    pub(crate) fn from_raw(raw_handle: usize) -> Self {
        Self { m_raw_handle: raw_handle as RawKernHandle }
    }
}

impl KernHandle /* Methods */ {
    /**
     * Returns whether this `KernHandle` references a valid kernel resource
     */
    pub fn is_valid(&self) -> bool {
        self.m_raw_handle != INVALID_KERN_HANDLE
        && self.inst_kern_call_0(KernFnPath::KernHandle(KernHandleFnId::IsValid))
               .map(|is_valid_handle| is_valid_handle != 0)
               .expect("Failed to check KernHandle validity")
    }
}

#[allow(dead_code)]
impl KernHandle /* Privates */ {
    /**
     * Returns the underling `RawKernHandle`
     */
    pub(crate) fn raw_handle(&self) -> RawKernHandle {
        self.m_raw_handle
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * without additional parameters
     */
    #[inline]
    pub(crate) fn inst_kern_call_0(&self, kern_fn_path: KernFnPath) -> Result<usize> {
        self.inst_kern_call_6(kern_fn_path, 0, 0, 0, 0, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with one additional parameter
     */
    #[inline]
    pub(crate) fn inst_kern_call_1(&self,
                                   kern_fn_path: KernFnPath,
                                   arg0: usize)
                                   -> Result<usize> {
        self.inst_kern_call_6(kern_fn_path, arg0, 0, 0, 0, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with two additional parameters
     */
    #[inline]
    pub(crate) fn inst_kern_call_2(&self,
                                   kern_fn_path: KernFnPath,
                                   arg0: usize,
                                   arg1: usize)
                                   -> Result<usize> {
        self.inst_kern_call_6(kern_fn_path, arg0, arg1, 0, 0, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with three additional parameters
     */
    #[inline]
    pub(crate) fn inst_kern_call_3(&self,
                                   kern_fn_path: KernFnPath,
                                   arg0: usize,
                                   arg1: usize,
                                   arg2: usize)
                                   -> Result<usize> {
        self.inst_kern_call_6(kern_fn_path, arg0, arg1, arg2, 0, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with four additional parameters
     */
    #[inline]
    pub(crate) fn inst_kern_call_4(&self,
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
     * with five additional parameters
     */
    #[inline]
    pub(crate) fn inst_kern_call_5(&self,
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
     * with six additional parameters
     */
    #[inline]
    pub(crate) fn inst_kern_call_6(&self,
                                   kern_fn_path: KernFnPath,
                                   arg0: usize,
                                   arg1: usize,
                                   arg2: usize,
                                   arg3: usize,
                                   arg4: usize,
                                   arg5: usize)
                                   -> Result<usize> {
        Self::do_kern_call(kern_fn_path,
                           Some(self.m_raw_handle),
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
    pub(crate) fn kern_call_0(kern_fn_path: KernFnPath) -> Result<usize> {
        Self::kern_call_6(kern_fn_path, 0, 0, 0, 0, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with one additional parameter
     */
    #[inline]
    pub(crate) fn kern_call_1(kern_fn_path: KernFnPath, arg0: usize) -> Result<usize> {
        Self::kern_call_6(kern_fn_path, arg0, 0, 0, 0, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with two additional parameters
     */
    #[inline]
    pub(crate) fn kern_call_2(kern_fn_path: KernFnPath,
                              arg0: usize,
                              arg1: usize)
                              -> Result<usize> {
        Self::kern_call_6(kern_fn_path, arg0, arg1, 0, 0, 0, 0)
    }

    /**
     * Performs the call to the kernel service described by `KernFnPath`
     * with three additional parameters
     */
    #[inline]
    pub(crate) fn kern_call_3(kern_fn_path: KernFnPath,
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
    pub(crate) fn kern_call_4(kern_fn_path: KernFnPath,
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
    pub(crate) fn kern_call_5(kern_fn_path: KernFnPath,
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
    pub(crate) fn kern_call_6(kern_fn_path: KernFnPath,
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
                    raw_handle: Option<RawKernHandle>,
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

impl Default for KernHandle {
    fn default() -> Self {
        Self { m_raw_handle: INVALID_KERN_HANDLE }
    }
}

impl Clone for KernHandle {
    fn clone(&self) -> Self {
        self.inst_kern_call_0(KernFnPath::KernHandle(KernHandleFnId::Clone))
            .map(|cloned_handle| Self::from_raw(cloned_handle))
            .expect("Failed to clone KernHandle")
    }
}

impl Drop for KernHandle {
    fn drop(&mut self) {
        self.inst_kern_call_0(KernFnPath::KernHandle(KernHandleFnId::Drop))
            .expect("Failed to drop handle");
    }
}
