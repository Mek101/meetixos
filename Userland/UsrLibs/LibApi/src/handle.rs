/*! Base kernel resource handle */

use api_data::sys::{
    codes::KernHandleFnId,
    fn_path::KernFnPath,
    RawKernHandle,
    INVALID_KERN_HANDLE
};

use crate::caller::KernCaller;

#[repr(transparent)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
pub struct KernHandle {
    m_raw_handle: RawKernHandle
}

impl KernHandle {
    pub(crate) fn from_raw(raw_handle: usize) -> Self {
        Self { m_raw_handle: raw_handle as RawKernHandle }
    }

    pub fn is_valid(&self) -> bool {
        self.m_raw_handle != INVALID_KERN_HANDLE
        && self.inst_kern_call_0(KernFnPath::KernHandle(KernHandleFnId::IsValid))
               .map(|is_valid_handle| is_valid_handle != 0)
               .expect("Failed to check KernHandle validity")
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

impl KernCaller for KernHandle {
    fn raw_handle(&self) -> RawKernHandle {
        self.m_raw_handle
    }
}
