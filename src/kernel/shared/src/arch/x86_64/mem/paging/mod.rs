/*! # x86_64 Paging
 *
 * Implements various hardware implementations of structure, methods and
 * constants for the common HAL's paging module
 */

pub use dir::X64PageDirSupport as HwPageDirSupport;
pub use flush::X64MapFlusher as HwMapFlusher;

mod dir;
mod flush;
