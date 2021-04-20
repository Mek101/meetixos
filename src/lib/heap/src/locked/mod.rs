/*! # Locked Heap Manager
 *
 * Implements the thread safe heap management
 */

#[cfg(not(feature = "disable_locked"))]
pub mod os;
pub mod raw;
