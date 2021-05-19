/*! Kernel core loader */

use crate::loader::cache::KernelPreLoadCache;

pub mod cache;

/* includes the module which links the kernel core binary */
include!(env!("KERNEL_BIN"));

/* various information about the kernel core which are accessed frequently */
static mut KERNEL_PRELOAD_CACHE: Option<KernelPreLoadCache> = None;

/**
 * Initializes the global kernel core cache to be rapidly accessed
 * afterwards
 */
pub fn loader_init_core_cache() {
    assert!(unsafe { KERNEL_PRELOAD_CACHE.is_none() });

    unsafe {
        KERNEL_PRELOAD_CACHE = Some(KernelPreLoadCache::new(KERNEL_BYTES.as_slice()));
    }
}

/**
 * Effectively loads the kernel core
 */
pub fn loader_load_core() {
    let preload_cache = loader_core_preload_cache();
    for _program_hdr in preload_cache.elf_file().program_iter() {}
}

/**
 * Returns the global static reference to the `KernelPreLoadCache`
 */
pub fn loader_core_preload_cache() -> &'static KernelPreLoadCache<'static> {
    if let Some(preload_cache) = unsafe { KERNEL_PRELOAD_CACHE.as_ref() } {
        preload_cache
    } else {
        panic!("Tried to obtain kernel pre-load cache, without pre-loading it");
    }
}
