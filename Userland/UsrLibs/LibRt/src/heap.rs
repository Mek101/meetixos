/*! Userland heap management */

use core::{
    alloc::Layout,
    ptr::NonNull
};

use api::object::{
    impls::{
        mmap::MMap,
        mutex::OsRawMutex
    },
    Object,
    UserCreatableObject
};
use heap::lazy_locked_heap::LazyLockedHeap;
use sync::mutex::CreatMayFailBackRawMutex;

/**
 * Global heap allocator
 */
#[global_allocator]
static mut S_HEAP_ALLOCATOR: LazyLockedHeap<OsRawMutex> =
    unsafe { LazyLockedHeap::new(raw_mutex_supplier, heap_mem_supplier) };

/**
 * Catches the allocation failures
 */
#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Heap allocation failed, no more memory available, requested {:?}", layout);
}

/**
 * Tries to create an `OsRawMutex`
 */
fn raw_mutex_supplier() -> Option<OsRawMutex> {
    OsRawMutex::try_creat().ok()
}

/**
 * Allocates an anonymous `MMap` to extend the `Heap` memory
 */
fn heap_mem_supplier(requested_size: usize) -> Option<(NonNull<u8>, usize)> {
    MMap::creat().for_read()
                 .for_write()
                 .with_data_size(requested_size)
                 .apply_for_anon()
                 .ok()
                 .map(|mmap| {
                     if let Ok(mmap_info) = mmap.info() {
                         if let Some(leak_mmap_ptr) =
                             NonNull::new(mmap.leak::<u8>().as_mut_ptr())
                         {
                             (leak_mmap_ptr, mmap_info.data_bytes_used())
                         } else {
                             panic!("Failed to obtain the NonNull_ptr for heap_allocator")
                         }
                     } else {
                         panic!("Failed to obtain MMap boot for heap_allocator")
                     }
                 })
}
