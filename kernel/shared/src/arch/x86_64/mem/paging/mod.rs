/*! x86_64 paging management */

pub use dir::X64PageDirSupport as HwPageDirSupport;
pub use flush::X64MapFlusher as HwMapFlusher;

mod dir;
mod flush;
