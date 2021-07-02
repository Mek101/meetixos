/*! `Task` exit value */

use crate::{
    error::OsError,
    sys::AsSysCallPtr
};

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

impl AsSysCallPtr for TaskExitStatus {
    /* No methods to implement */
}

impl Default for TaskExitStatus {
    fn default() -> Self {
        Self::Success
    }
}
