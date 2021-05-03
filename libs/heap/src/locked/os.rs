/*! # Locked Heap Manager
 *
 * Implements a concurrency-safe heap manager usable as `global_alloc` in a
 * userspace environment and implements for it the [`GlobalAlloc`] trait
 *
 * [`GlobalAlloc`]: core::alloc::GlobalAlloc
 */

use core::ops::Deref;

use linked_list_allocator::align_up;

use api::objs::{
    impls::{
        MMap,
        OsRawMutex
    },
    UserCreatable
};

use crate::{
    consts::PAGE_SIZE,
    locked::raw::RawLazyLockedHeap
};

/** # Locked Heap Manager
 *
 * Defines a thread-safe multi-strategy heap manager that could be used as
 * `global_allocator` in a multi threaded environment.
 *
 * Internally uses a [`Mutex`] to ensure mutually exclusive access to an
 * [`Heap`] instance
 *
 * [`Mutex`]: api::objs::impls::mutex::Mutex
 * [`Heap`]: crate::Heap
 */
pub struct OsLockedHeap {
    m_locked_heap: RawLazyLockedHeap<OsRawMutex>
}

impl OsLockedHeap {
    /** # Constructs a new `LockedHeap`
     *
     * If `mem_supplier` is [`None`] the object relies on an internal
     * implementation that uses anonymous [`MMap`]s
     *
     * [`None`]: core::option::Option::None
     * [`MMap`]: api::objs::impls::mmap::MMap
     */
    pub const fn new() -> Self {
        let raw_mutex_supplier =
            || OsRawMutex::creat().for_read().for_write().apply_for_anon().ok();
        Self { m_locked_heap: unsafe {
                   RawLazyLockedHeap::new(raw_mutex_supplier, Self::default_mem_supplier)
               } }
    }

    /** # User memory supplier
     *
     * Used as default [`HeapMemorySupplier`] for the underling [`Heap`]
     *
     * [`HeapMemorySupplier`]: crate::HeapMemorySupplier
     * [`Heap`]: crate::Heap
     */
    fn default_mem_supplier(requested_size: usize) -> Option<(usize, usize)> {
        let aligned_size = align_up(requested_size, PAGE_SIZE);

        /* create an anonymous memory mapping, then leak it.
         *
         * The leaked memory will be managed by the Heap manager until the process
         * live, in fact when the `Heap` deallocates the memory it is not returned to
         * the kernel, but stored into the memory pool of the manager
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
    /** The resulting type after dereference.    
     */
    type Target = RawLazyLockedHeap<OsRawMutex>;

    /** Dereferences the value.
     */
    fn deref(&self) -> &Self::Target {
        &self.m_locked_heap
    }
}
