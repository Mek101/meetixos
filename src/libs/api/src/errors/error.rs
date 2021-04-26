/*! # Error Descriptor
 *
 * Implements the error descriptor used by all the system calls
 */

use core::{
    fmt,
    fmt::{Display, Formatter}
};

use os::{limits::ERROR_MESSAGE_LEN_MAX, str_utils, sysc::id::SysCallId};

use crate::{
    errors::ErrorClass,
    tasks::{
        impls::{Proc, Thread},
        Task
    }
};

/** # Kernel Error
 *
 * Represents the standard way to represent an OS error in MeetiX
 */
#[derive(Debug, Default)]
pub struct Error {
    m_class: ErrorClass,
    m_syscall: SysCallId,
    m_message: Option<[u8; ERROR_MESSAGE_LEN_MAX]>
}

impl Error {
    /** # Constructs an `Error`
     *
     * The returned instance is filled with the given data
     */
    pub fn new(class: ErrorClass, syscall: SysCallId, message: Option<&str>) -> Self {
        Self { m_class: class,
               m_syscall: syscall,
               m_message: message.map(|str_buf| {
                                     let mut buf = [0; ERROR_MESSAGE_LEN_MAX];
                                     str_utils::copy_str_to_u8_buf(&mut buf, str_buf);
                                     buf
                                 }) }
    }

    /** Returns the [`ErrorClass`]
     *
     * [`ErrorClass`]: /api/errors/enum.ErrorClass.html
     */
    pub fn class(&self) -> ErrorClass {
        self.m_class
    }

    /** Returns the [`SysCallId`]
     *
     * [`SysCallId`]: /os/sysc/id/struct.SysCallId.html
     */
    pub fn syscall(&self) -> SysCallId {
        self.m_syscall
    }

    /** Returns the formatted message of the error if any
     */
    pub fn message(&self) -> Option<&str> {
        self.m_message.map(|buf| str_utils::u8_slice_to_str_slice(&buf))
    }

    /**
     * Returns `self` as mutable usize pointer (used by the [`KernCaller`])
     *
     * [`KernCaller`]: /api/caller/trait.KernCaller.html
     */
    pub(crate) fn as_ptr(&mut self) -> *mut usize {
        self as *mut _ as *mut usize
    }
}

impl Display for Error {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let pid = Proc::this().id();
        let tid = Thread::this().id();

        /* write the complete error message as follow:
         * [<pid>:<tid>] Error: <Human readable error class>\n
         *       : Reason: <Optional error message from the kernel>
         */
        let mut res = writeln!(f, "[{}:{}] Error: {}", pid, tid, self.m_class);
        if res.is_ok() && self.m_message.is_some() {
            res = writeln!(f, "\t: Reason: {}", self.message().unwrap());
        }
        res
    }
}
