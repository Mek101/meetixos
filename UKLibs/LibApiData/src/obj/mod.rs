/*! `Object` related data */

pub mod config;
pub mod device;
pub mod dir;
pub mod grants;
pub mod info;
pub mod modes;
pub mod types;
pub mod uses;

/**
 * Convenience type renaming for `Object` reference
 */
pub type RawObjHandle = crate::sys::RawKernHandle;
