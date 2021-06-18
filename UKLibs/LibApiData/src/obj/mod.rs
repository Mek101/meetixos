/*! `Object` related data */

use crate::sys::KernHandle;

pub mod config;
pub mod grants;
pub mod info;
pub mod modes;
pub mod types;
pub mod uses;

/**
 * Convenience type renaming for `Object` reference
 */
pub type RawObjId = KernHandle;
