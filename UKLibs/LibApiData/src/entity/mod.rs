/*! `OSEntity` related data */

pub mod config;
pub mod types;

/**
 * Convenience type renaming for `OsEntity` reference
 */
pub type RawOsEntityHandle = crate::sys::RawKernHandle;

/**
 * System wide unique `OsEntity` identifier
 */
pub type OsEntityId = u64;
