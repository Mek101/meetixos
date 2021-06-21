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
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
pub struct KernHandle {
    m_raw_handle: RawKernHandle
}

impl KernHandle {
    pub(crate) fn from_raw(raw_handle: RawKernHandle) -> Self {
        Self { m_raw_handle: raw_handle }
    }

    pub fn is_valid(&self) -> bool {
        self.m_raw_handle != INVALID_KERN_HANDLE
        && self.inst_kern_call_0(KernFnPath::KernHandle(KernHandleFnId::IsValid))
               .map(|is_valid_handle| {
                   is_valid_handle as RawKernHandle != INVALID_KERN_HANDLE
               })
               .expect("Failed to check KernHandle validity")
    }
}

impl KernCaller for KernHandle {
    fn raw_handle(&self) -> RawKernHandle {
        self.m_raw_handle
    }
}
