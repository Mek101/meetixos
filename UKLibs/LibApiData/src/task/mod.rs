/*! `Task` related data */

pub mod config;
pub mod exit;
pub mod fs_types;
pub mod modes;
pub mod thread;
pub mod types;

/**
 * Convenience type renaming for `Task` reference
 */
pub type RawTaskHandle = crate::sys::RawKernHandle;

/**
 * System wide unique `Task` identifier
 */
pub type TaskId = u64;
