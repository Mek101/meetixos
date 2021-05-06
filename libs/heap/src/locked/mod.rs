/*! Thread-safe `Heap` wrappers */

#[cfg(not(feature = "disable_locked"))]
pub mod os;
pub mod raw;
