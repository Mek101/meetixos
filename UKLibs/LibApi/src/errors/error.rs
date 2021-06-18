/*! Error descriptor */

use core::fmt;

use helps::str::{
    copy_str_to_u8_buf,
    u8_slice_to_str_slice
};
use os::{
    limits::OS_ERROR_MESSAGE_LEN_MAX,
    sysc::id::SysCallId
};

use crate::{
    errors::class::OsErrorClass,
    tasks::{
        impls::{
            proc::Proc,
            thread::Thread
        },
        task::Task
    }
};

/**
 * Standard way to represent an OS error in MeetiX
 */
#[derive(Debug, Default)]
pub struct OsError {
    m_class: OsErrorClass,
    m_syscall: SysCallId,
    m_message: Option<[u8; OS_ERROR_MESSAGE_LEN_MAX]>
}

impl OsError {
    /**
     * Constructs an `OsError` filled with the given data
     */
    pub fn new(class: OsErrorClass, syscall: SysCallId, message: Option<&str>) -> Self {
        Self { m_class: class,
               m_syscall: syscall,
               m_message: message.map(|str_buf| {
                                     let mut buf = [0; OS_ERROR_MESSAGE_LEN_MAX];
                                     copy_str_to_u8_buf(&mut buf, str_buf);
                                     buf
                                 }) }
    }

    /**
     * Returns the `ErrorClass`
     */
    pub fn class(&self) -> OsErrorClass {
        self.m_class
    }

    /**
     * Returns the `SysCallId`
     */
    pub fn syscall(&self) -> SysCallId {
        self.m_syscall
    }

    /**
     * Returns the formatted message of the error if any
     */
    pub fn message(&self) -> Option<&str> {
        self.m_message.map(|buf| u8_slice_to_str_slice(&buf))
    }

    /**
     * Returns `self` as mutable usize pointer (used by the `KernCaller`)
     */
    pub(crate) fn as_ptr(&mut self) -> *mut usize {
        self as *mut _ as *mut usize
    }
}

impl fmt::Display for OsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pid = Proc::this().id();
        let tid = Thread::this().id();

        /* write the complete error message as follow:
         * [<pid>:<tid>] Error: <Human readable error class>\n
         *       : Reason: <Optional error message from the Kernel>
         */
        writeln!(f, "[{}:{}] Error: {}", pid, tid, self.m_class)?;
        if let Some(message) = self.message() {
            writeln!(f, "\t: Reason: {}", message)?;
        }
        Ok(())
    }
}
