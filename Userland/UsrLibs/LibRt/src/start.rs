/*! Userland entry-point */

use api::task::{
    impls::proc::Proc,
    Task
};
use api_data::{
    error::OsError,
    task::exit::TaskExitStatus
};

/**
 * Entry point for userspace applications
 */
#[lang = "start"]
unsafe extern "C" fn lang_start<T>(rust_entry_point: fn() -> T,
                                   _argc: isize,
                                   _argv: *const *const u8)
    where T: Termination + 'static {
    Proc::exit(rust_entry_point().report());
}

/**
 * Termination trait useful to obtain the TaskExitStatus
 */
#[lang = "termination"]
pub trait Termination {
    /**
     * Returns the `TaskExitStatus`
     */
    fn report(self) -> TaskExitStatus;
}

impl Termination for () {
    fn report(self) -> TaskExitStatus {
        TaskExitStatus::Success
    }
}

impl Termination for Result<u8, OsError> {
    fn report(self) -> TaskExitStatus {
        match self {
            Ok(exit_value) => TaskExitStatus::WithValue(exit_value as usize),
            Err(os_error) => TaskExitStatus::WithError(os_error)
        }
    }
}

impl Termination for Result<u16, OsError> {
    fn report(self) -> TaskExitStatus {
        match self {
            Ok(exit_value) => TaskExitStatus::WithValue(exit_value as usize),
            Err(os_error) => TaskExitStatus::WithError(os_error)
        }
    }
}

impl Termination for Result<u32, OsError> {
    fn report(self) -> TaskExitStatus {
        match self {
            Ok(exit_value) => TaskExitStatus::WithValue(exit_value as usize),
            Err(os_error) => TaskExitStatus::WithError(os_error)
        }
    }
}

impl Termination for Result<u64, OsError> {
    fn report(self) -> TaskExitStatus {
        match self {
            Ok(exit_value) => TaskExitStatus::WithValue(exit_value as usize),
            Err(os_error) => TaskExitStatus::WithError(os_error)
        }
    }
}

impl Termination for Result<usize, OsError> {
    fn report(self) -> TaskExitStatus {
        match self {
            Ok(exit_value) => TaskExitStatus::WithValue(exit_value),
            Err(os_error) => TaskExitStatus::WithError(os_error)
        }
    }
}
