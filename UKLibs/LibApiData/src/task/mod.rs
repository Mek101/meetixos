/*! Tasks management */

use crate::sys::KernHandle;

pub mod config;
pub mod fs_types;
pub mod modes;
pub mod thread;
pub mod types;

/**
 * Convenience type renaming for `Task` reference
 */
pub type RawTaskId = KernHandle;
