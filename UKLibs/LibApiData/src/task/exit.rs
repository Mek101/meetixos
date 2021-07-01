/*! `Task` exit value */

use crate::error::OsError;

/**
 * `Task` exist status
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum TaskExitStatus {
    /**
     * `Task` was terminated without errors and return value
     */
    Success,

    /**
     * `Task` was terminated without errors but with return value
     */
    WithValue(usize),

    /**
     * `Task` was terminated with errors
     */
    WithError(OsError)
}

impl TaskExitStatus {
    /**
     * Returns `&self` as usize pointer value
     */
    pub fn as_syscall_ptr(&self) -> usize {
        self as *const _ as usize
    }
}
