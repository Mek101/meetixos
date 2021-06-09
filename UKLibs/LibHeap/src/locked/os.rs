/*! Userspace thread-safe `Heap` */

use core::ops::Deref;

use api::objs::{
    impls::{
        mmap::MMap,
        mutex::OsRawMutex
    },
    object::UserCreatable
};
use helps::align::align_up;

use crate::locked::raw::RawLazyLockedHeap;

/**
 * Multi strategy heap manager capable of use as `global_allocator` in multi
 * threaded environments.
 *
 * Internally uses an `LibApi::objs::impls::Mutex` to ensure mutually
 * exclusive access to the `Heap` instance
 */
pub struct OsLockedHeap {
    m_locked_heap: RawLazyLockedHeap<OsRawMutex>
}

impl OsLockedHeap {
    /**
     * Constructs a new `OsLockedHeap` which relies on anonymous
     * `LibApi::objs::impls::MMap`s to obtain more system memory
     */
    pub const fn new() -> Self {
        Self { m_locked_heap: unsafe {
                   RawLazyLockedHeap::new(|| {
                                              OsRawMutex::creat().for_read()
                                                                 .for_write()
                                                                 .apply_for_anon()
                                                                 .ok()
                                          },
                                          Self::default_mem_supplier)
               } }
    }

    /**
     * Default `HeapMemorySupplier` for the underling `Heap`
     */
    fn default_mem_supplier(requested_size: usize) -> Option<(usize, usize)> {
        let aligned_size = align_up(requested_size, 4096);

        /* create an anonymous memory mapping, then leak it.
         *
         * The leaked memory will be managed by the Heap manager until the process
         * live, in fact when the `Heap` deallocates the memory it is not returned to
         * the Kernel, but stored into the memory pool of the manager
         */
        MMap::creat().for_read()
                     .for_write()
                     .with_size(aligned_size)
                     .apply_for_anon()
                     .ok()
                     .map(|mmap| {
                         (mmap.leak_ptr::<u8>().as_mut_ptr() as usize, aligned_size)
                     })
    }
}

impl Deref for OsLockedHeap {
    type Target = RawLazyLockedHeap<OsRawMutex>;

    fn deref(&self) -> &Self::Target {
        &self.m_locked_heap
    }
}
