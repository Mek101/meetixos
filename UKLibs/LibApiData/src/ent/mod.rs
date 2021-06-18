/*! `OSEntity` related data */

use crate::sys::KernHandle;

pub mod config;
pub mod types;

/**
 * Convenience type renaming for `OsEntity` reference
 */
pub type RawOsEntityId = KernHandle;
