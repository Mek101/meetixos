/*! Generics-customizable locked `Heap` */

use core::{
    alloc::{
        GlobalAlloc,
        Layout
    },
    ptr,
    ptr::NonNull
};

use sync::{
    Lazy,
    Mutex,
    RawMutex
};

use crate::{
    Heap,
    HeapMemorySupplier
};

/**
 * Callback used by the `RawLazyLockedHeap` to obtain the `sync::RawMutex`
 * implementation
 */
pub type RawLazyMutexSupplier<M> = fn() -> Option<M>;

/**
 * Locked heap with a customizable `sync::Mutex` backend that is lazily
 * initialized
 *
 * This allow the use of the struct as `global_allocator` using constant
 * initialization
 */
pub struct RawLazyLockedHeap<M>
    where M: RawMutex + 'static {
    m_lazy_locked_heap: Lazy<Mutex<M, Heap>, LazyHeapInitializer<M>>
}

impl<M> RawLazyLockedHeap<M> where M: RawMutex + 'static {
    /**
     * Constructs a `RawLazyLockedHeap` without effectively initialize the
     * internal `sync::Mutex` or `Heap`
     */
    pub const unsafe fn new(raw_mutex_supplier: RawLazyMutexSupplier<M>,
                            mem_supplier: HeapMemorySupplier)
                            -> Self {
        Self { m_lazy_locked_heap:
                   Lazy::new(LazyHeapInitializer::new(raw_mutex_supplier, mem_supplier)) }
    }

    /**
     * Forces the initialization of this lazy object
     */
    pub fn force_init(&self) {
        self.m_lazy_locked_heap.lock().allocated_mem();
    }

    /**
     * Returns the size of the current managed area in bytes
     */
    pub fn managed_mem(&self) -> usize {
        self.m_lazy_locked_heap.lock().managed_mem()
    }

    /**
     * Returns the currently allocated size in bytes
     */
    pub fn allocated_mem(&self) -> usize {
        self.m_lazy_locked_heap.lock().allocated_mem()
    }

    /**
     * Returns the available memory amount
     */
    pub fn free_memory(&self) -> usize {
        self.m_lazy_locked_heap.lock().free_memory()
    }
}

unsafe impl<M> GlobalAlloc for RawLazyLockedHeap<M> where M: RawMutex {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.m_lazy_locked_heap
            .lock()
            .allocate(layout)
            .map_or(ptr::null_mut(), |nn_ptr| nn_ptr.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Some(nn_ptr) = NonNull::new(ptr) {
            self.m_lazy_locked_heap.lock().deallocate(nn_ptr, layout)
        }
    }
}

/**
 * Concrete type for the `FnOnce` trait used by the `sync::Lazy`.
 *
 * Since the `sync::Lazy` by default defines his `F` generic parameter to
 * `fn()` (that cannot capture objects from his environment) and because the
 * closures have no concrete type because are implemented by the compiler
 * during build process, this is the only way (for now) to have a lazy
 * function to give to the `sync::Lazy` that captures local objects
 */
struct LazyHeapInitializer<T>
    where T: RawMutex {
    m_raw_mutex_supplier: RawLazyMutexSupplier<T>,
    m_mem_supplier: HeapMemorySupplier
}

impl<T> LazyHeapInitializer<T> where T: RawMutex {
    /**
     * Constructs a `LazyHeapInitializer`
     */
    const fn new(raw_mutex_supplier: RawLazyMutexSupplier<T>,
                 mem_supplier: HeapMemorySupplier)
                 -> Self {
        Self { m_raw_mutex_supplier: raw_mutex_supplier,
               m_mem_supplier: mem_supplier }
    }
}

impl<T> FnOnce<()> for LazyHeapInitializer<T> where T: RawMutex {
    type Output = Mutex<T, Heap>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        let raw_mutex =
            (self.m_raw_mutex_supplier)().expect("Failed to lazy obtain `RawMutex`");
        Mutex::const_new(raw_mutex, unsafe { Heap::new(self.m_mem_supplier) })
    }
}
