/*! Kernel core loader */

use core::mem::size_of;

use shared::{
    addr::virt::VirtAddr,
    logger::{
        debug,
        info,
        trace
    }
};

use crate::{
    arch::loader::arch_loader_switch_to_kernel,
    info::info_prepare_loader_info,
    loader::{
        cache::KernelPreLoadCache,
        elf::loader_elf_load_core_elf,
        stack::loader_stack_setup_core_stack
    },
    mem::paging::paging_current_page_dir
};

pub mod cache;
pub mod elf;
pub mod stack;

/* includes the module which links the Kernel core binary */
include!(env!("KERNEL_BIN"));

/* various information about the Kernel core which are accessed frequently */
static mut KERNEL_PRELOAD_CACHE: Option<KernelPreLoadCache> = None;

/**
 * Initializes the global Kernel core cache to be rapidly accessed
 * afterwards
 */
pub fn loader_init_core_cache() {
    assert!(unsafe { KERNEL_PRELOAD_CACHE.is_none() });

    unsafe {
        KERNEL_PRELOAD_CACHE = Some(KernelPreLoadCache::new(KERNEL_BYTES.as_slice()));
    }
}

/**
 * Effectively loads the Kernel core
 */
pub fn loader_load_core() {
    /* load the Kernel core parts needed for switching */
    let stack_area = loader_stack_setup_core_stack();
    let core_entry_point = loader_elf_load_core_elf();
    let loader_info = info_prepare_loader_info();

    info!("Starting Kernel Core...\n");
    debug!("Switching to Kernel core jumping at: {:x}", core_entry_point);
    trace!("PageDir composition:\n{:?}", paging_current_page_dir());

    /* switch to the Kernel core */
    unsafe {
        arch_loader_switch_to_kernel(stack_area.end_addr() - size_of::<usize>(),
                                     VirtAddr::from(loader_info as *const _),
                                     core_entry_point);
    }
}

/**
 * Returns the global static reference to the `KernelPreLoadCache`
 */
pub fn loader_core_preload_cache() -> &'static KernelPreLoadCache<'static> {
    if let Some(preload_cache) = unsafe { KERNEL_PRELOAD_CACHE.as_ref() } {
        preload_cache
    } else {
        panic!("Tried to obtain Kernel pre-load cache, without pre-loading it");
    }
}
