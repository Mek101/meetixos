/*! System call codes and classes */

use crate::{
    error::OsError,
    limit::SYSCALL_ARGS_COUNT_MAX,
    sys::fn_path::KernFnPath
};

pub mod codes;
pub mod fn_path;

/**
 * Invalid kernel handle value
 */
pub const INVALID_KERN_HANDLE: RawKernHandle = RawKernHandle::MAX;

/**
 * Convenience type renaming for kernel handles
 */
pub type RawKernHandle = u32;

/**
 * Fixed collector of system call arguments
 */
#[derive(Debug)]
pub struct SysCallPayload {
    m_kern_fn_path: KernFnPath,
    m_raw_handle: Option<RawKernHandle>, /* TODO tables with inst_required = <> */
    m_raw_args: [usize; SYSCALL_ARGS_COUNT_MAX],
    m_error_modified: bool,
    m_error: OsError,
    m_result: usize
}

impl SysCallPayload {
    /**
     * Constructs a `SysCallArgs` with the given parameters
     */
    #[inline]
    pub fn new(kern_fn_path: KernFnPath,
               raw_handle: Option<RawKernHandle>,
               arg0: usize,
               arg1: usize,
               arg2: usize,
               arg3: usize,
               arg4: usize,
               arg5: usize)
               -> Self {
        Self { m_kern_fn_path: kern_fn_path,
               m_raw_handle: raw_handle,
               m_raw_args: [arg0, arg1, arg2, arg3, arg4, arg5],
               m_error_modified: false,
               m_error: OsError::default(),
               m_result: 0 }
    }

    /**
     * Returns the `KernFnPath` to the kernel function to call
     */
    #[inline]
    pub fn kern_fn_path(&self) -> KernFnPath {
        self.m_kern_fn_path
    }

    /**
     * Returns the kernel handle instance if any
     */
    #[inline]
    pub fn raw_handle(&self) -> Option<RawKernHandle> {
        self.m_raw_handle
    }

    /**
     * Returns the value for the argument at `arg_index` as `usize`
     */
    #[inline]
    pub fn raw_arg(&self, arg_index: usize) -> usize {
        assert!(arg_index < SYSCALL_ARGS_COUNT_MAX, "Requested an argument out of index");

        self.m_raw_args[arg_index]
    }

    /**
     * Returns a copy of the value as `T` at `arg_index`
     */
    #[inline]
    pub fn arg_value<T>(&self, arg_index: usize) -> T
        where T: From<usize> {
        T::from(self.raw_arg(arg_index))
    }

    /**
     * Returns the argument at `arg_index` as immutable reference of `T`
     */
    #[inline]
    pub fn arg_ref<T>(&self, arg_index: usize) -> &T {
        unsafe { &*(self.raw_arg(arg_index) as *const T) }
    }

    /**
     * Returns the argument at `arg_index` as mutable reference of `T`
     */
    #[inline]
    pub fn arg_ref_mut<T>(&self, arg_index: usize) -> &mut T {
        unsafe { &mut *(self.raw_arg(arg_index) as *mut T) }
    }

    /**
     * Returns the reference to the `OsError` instance
     */
    #[inline]
    pub fn error(&self) -> &OsError {
        &self.m_error
    }

    /**
     * Returns the mutable reference to the `OsError` instance
     *
     * NOTE: Sets too the `m_error_modified` field to true
     */
    #[inline]
    pub fn error_mut(&mut self) -> &mut OsError {
        self.m_error_modified = true;
        &mut self.m_error
    }

    /**
     * Returns `self` as `usize` pointer
     */
    #[inline]
    pub fn as_syscall_ptr(&mut self) -> usize {
        self as *mut Self as usize
    }
}

impl Into<Result<usize, OsError>> for SysCallPayload {
    #[inline]
    fn into(self) -> Result<usize, OsError> {
        if !self.m_error_modified {
            Ok(self.m_result)
        } else {
            Err(self.m_error)
        }
    }
}
