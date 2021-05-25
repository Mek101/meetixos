/*! Kernel paging management */

use shared::{
    info::descriptor::LoaderInfo,
    logger::{
        debug,
        trace
    },
    mem::paging::{
        dir::PageDir,
        flush::MapFlusher,
        frame::VirtFrame
    }
};

use crate::mem::{
    paging::allocators::{
        KernAllocator,
        NoAllocator
    },
    vm_layout::vml_layout
};

pub mod allocators;

/**
 * Unmaps the `HH_Loader`'s ranges, after this call the `loader_info` is no
 * more reachable
 */
pub fn paging_unmap_loader(loader_info: &LoaderInfo) {
    /* convert the inclusive ranges into std range */
    let loader_mapped_range = {
        let inclusive_range = loader_info.loader_mapped_range();

        VirtFrame::range_of(inclusive_range.start().clone(),
                            inclusive_range.end().clone() + 1)
    };
    let loader_reserved_range = {
        let inclusive_range = loader_info.loader_reserved_range();

        VirtFrame::range_of(inclusive_range.start().clone(),
                            inclusive_range.end().clone() + 1)
    };

    //assert!(loader_reserved_range.)

    let mut page_dir = paging_current_page_dir();

    /* unmap the mapped range of the loader.
     *
     * The mapped range is the physical memory mapping which the loader initially
     * maps in identity mapping to be able to switch to paging mode.
     *
     * <NoAllocator> is used because the physical frames in the range cannot be
     * marked as free because is simply memory mapping used to access it
     */
    debug!("Unmapping loader_mapped_range: {:?}", loader_mapped_range);
    match page_dir.unmap_range(loader_mapped_range.clone(), &NoAllocator, false) {
        Ok(unmap_flusher) => unmap_flusher.flush(),
        Err(err) => panic!("Failed to unmap HH_Loader mapped range ({:?}): cause: {}",
                           loader_mapped_range, err)
    }

    /* unmap the reserved range of the loader.
     *
     * The reserved range is where the HH_Loader resides with his text and data.
     * The pages in this range can be marked as available, but not the page
     * tables, since they are statically allocated by the <HH_Loader> in the
     * assembly stub, the physical frames are part of the reserved_range (i.e
     * .data section)
     */
    debug!("Unmapping loader_reserved_range: {:?}", loader_reserved_range);
    match page_dir.unmap_range(loader_reserved_range.clone(),
                               &KernAllocator::new_tweak(false),
                               true)
    {
        Ok(unmap_flusher) => unmap_flusher.flush(),
        Err(err) => panic!("Failed to unmap HH_Loader reserved range ({:?}): cause: {}",
                           loader_mapped_range, err)
    }

    trace!("Current PageDir composition:\n{:?}", page_dir);
}

/**
 * Returns the currently active `PageDir` instance
 */
pub fn paging_current_page_dir() -> PageDir {
    unsafe { PageDir::active_page_dir(vml_layout().phys_mem_mapping_area().start_addr()) }
}
