/*! HH_Loader paging management */

use shared::{
    addr::{
        virt::VirtAddr,
        Address
    },
    mem::paging::dir::PageDir
};

pub mod allocator;

/**
 * Returns the currently active `PageDir` instance
 */
pub fn paging_current_page_dir() -> PageDir {
    unsafe { PageDir::active_page_dir(VirtAddr::new_zero()) }
}
