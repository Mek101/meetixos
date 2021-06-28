/*! `Task` exit value */

use crate::error::OsError;

#[derive(Debug)]
pub enum TaskExitStatus {
    Success,
    WithValue(usize),
    WithError(OsError)
}

impl TaskExitStatus {
    pub fn as_syscall_ptr(&self) -> usize {
        self as *const _ as usize
    }
}
