/*! # Raw Locked Heap
 *
 * Implements a raw locked heap that relies to a [`Mutex`] to ensure mutual
 * exclusion access
 *
 * [`Mutex`]: sync::Mutex
 */

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

/** # Raw Lazy Mutex Supplier
 *
 * Represents the callback used by the [`RawLazyLockedHeap`] to obtain the
 * [`RawMutex`] implementation
 *
 * [`RawLazyLockedHeap`]: crate::locked::raw::RawLazyLockedHeap
 * [`RawMutex`]: sync::RawMutex
 */
pub type RawLazyMutexSupplier<M> = fn() -> Option<M>;

/** # Raw Lazy Locked Heap
 *
 * Implements a locked heap with a customizable [`Mutex`] backend that is
 * lazily initialized.
 *
 * This allow the use of the struct as `global_allocator` using constant
 * initialization
 *
 * [`Mutex`]: sync::Mutex
 */
pub struct RawLazyLockedHeap<M>
    where M: RawMutex + 'static {
    m_lazy_locked_heap: Lazy<Mutex<M, Heap>, LazyHeapInitializer<M>>
}

impl<M> RawLazyLockedHeap<M> where M: RawMutex + 'static {
    /** # Constructs a `RawLazyLockedHeap`
     *
     * No heap/mutex initialization are performed inside this method
     */
    pub const unsafe fn new(raw_mutex_supplier: RawLazyMutexSupplier<M>,
                            mem_supplier: HeapMemorySupplier)
                            -> Self {
        Self { m_lazy_locked_heap:
                   Lazy::new(LazyHeapInitializer::new(raw_mutex_supplier, mem_supplier)) }
    }

    /** # Forces the lazy initialization
     *
     * Calls the lock method to throws the initialization of the object.
     *
     * This step is not really necessary since the first call to the object
     * already initializes it, but for performance critical applications
     * this is a way to have constant access to the object from the first
     * access
     */
    pub fn force_init(&self) {
        self.m_lazy_locked_heap.lock().allocated_mem();
    }

    /** Returns the size of the current managed area in bytes
     */
    pub fn managed_mem(&self) -> usize {
        self.m_lazy_locked_heap.lock().managed_mem()
    }

    /** Returns the currently allocated size in bytes
     */
    pub fn allocated_mem(&self) -> usize {
        self.m_lazy_locked_heap.lock().allocated_mem()
    }

    /** Returns the available memory amount
     */
    pub fn free_memory(&self) -> usize {
        self.m_lazy_locked_heap.lock().free_memory()
    }
}

unsafe impl<M> GlobalAlloc for RawLazyLockedHeap<M> where M: RawMutex {
    /** Allocate memory as described by the given `layout`
     */
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.m_lazy_locked_heap
            .lock()
            .allocate(layout)
            .map_or(ptr::null_mut(), |nn_ptr| nn_ptr.as_ptr())
    }

    /** Deallocate the block of memory at the given `ptr` pointer with the
     * given `layout`
     */
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Some(nn_ptr) = NonNull::new(ptr) {
            self.m_lazy_locked_heap.lock().deallocate(nn_ptr, layout)
        }
    }
}

/** # Lazy Heap Initializer
 *
 * Implements a concrete type for the [`FnOnce`] trait used by the [`Lazy`].
 *
 * Since the [`Lazy`] by default defines his `F` generic parameter to `fn()`
 * (that cannot capture objects from his environment) and because the
 * closures have no concrete type because are implemented by the compiler
 * when created, this is the only way (for now) to have a lazy function to
 * give to the [`Lazy`] that captures local objects
 *
 * [`FnOnce`]: core::ops::FnOnce
 * [`Lazy`]: sync::Lazy
 */
struct LazyHeapInitializer<T>
    where T: RawMutex {
    m_raw_mutex_supplier: RawLazyMutexSupplier<T>,
    m_mem_supplier: HeapMemorySupplier
}

impl<T> LazyHeapInitializer<T> where T: RawMutex {
    /** # Constructs a `LazyHeapInitializer`
     *
     * The returned instance is ready to be called
     */
    const fn new(raw_mutex_supplier: RawLazyMutexSupplier<T>,
                 mem_supplier: HeapMemorySupplier)
                 -> Self {
        Self { m_raw_mutex_supplier: raw_mutex_supplier,
               m_mem_supplier: mem_supplier }
    }
}

impl<T> FnOnce<()> for LazyHeapInitializer<T> where T: RawMutex {
    /** The returned type after the call operator is used
     */
    type Output = Mutex<T, Heap>;

    /** Performs the call operation
     */
    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        let raw_mutex =
            (self.m_raw_mutex_supplier)().expect("Failed to lazy obtain `RawMutex`");
        Mutex::const_new(raw_mutex, unsafe { Heap::new(self.m_mem_supplier) })
    }
}
