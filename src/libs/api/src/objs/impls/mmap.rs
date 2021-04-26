/*! # Memory Mapping Object
 *
 * Implements a scope managed address space memory mapping object a a a a a
 * a a a a a a
 */

use core::{
    intrinsics::size_of,
    mem,
    ops::{Deref, DerefMut},
    slice
};

use os::sysc::{codes::KernMMapFnId, fn_path::KernFnPath};

use crate::{
    bits::obj::{MMapPtrMode, ObjType, WithExecutableDataObject},
    caller::{KernCaller, Result},
    objs::{ObjId, Object, SizeableData, UserCreatable}
};

impl_obj_id_object! {
    /** # Memory Mapping
     *
     * Represents a reference to a mapped piece of the owner's address
     * space.
     *
     * `MMap` acts like a simple [`Box`] that instead references memory
     * directly returned by the kernel (not by an userspace allocator)
     * for a more direct use.
     *
     * The memory referenced by a `MMap` can be shared among different
     * processes and securely accessed via [`MMap::get_ptr()`] and
     * [`MMap::get_ptr_mut()`] because the kernel manages accesses via
     * `RWLock`, so multiple threads can read, but only one can write.
     *
     * When the `MMap` object goes out of scope the memory is unmapped
     * from the caller process
     *
     * [`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
     * [`MMap::get_ptr()`]: /api/objs/impls/struct.html#method.get_ptr
     * [`MMap::get_ptr_mut()`]: /api/objs/impls/struct.html#method.get_ptr_mut
     */
    pub struct MMap : impl WithExecutableDataObject,
                           SizeableData,
                           UserCreatable {
        where TYPE = ObjType::MMap;
    }
}

impl MMap {
    /** # Obtains a readable memory access
     *
     * Returns a [`ConstMMapBox`] that allows the caller to read the memory
     * increasing of one the readers counter.
     *
     * If the `MMap` have reached the maximum amount of readers or there is
     * already a writer the thread waits until someone releases the memory
     *
     * [`ConstMMapBox`]: /api/objs/impls/struct.ConstMMapBox.html
     */
    pub fn get_ptr<T>(&self) -> Result<ConstMMapBox<T>> {
        self.get(MMapPtrMode::Readable).map(|(raw_ptr, size)| {
                                           ConstMMapBox::new(self,
                                                             raw_ptr as *const T,
                                                             size)
                                       })
    }

    /** # Returns the writeable memory access
     *
     * Returns a [`MutMMapBox`] that allows the caller to write the memory.
     *
     * If the `MMap` already have a writer or have at least one reader the
     * thread waits until all releases the memory
     *
     * [`MutMMapBox`]: /api/objs/impls/struct.MutMMapBox.html
     */
    pub fn get_ptr_mut<T>(&self) -> Result<MutMMapBox<T>> {
        self.get(MMapPtrMode::Writeable)
            .map(|(raw_ptr, size)| MutMMapBox::new(self, raw_ptr as *mut T, size))
    }

    /** # Leaks the `MMap`'s memory
     *
     * Returns a mutable reference to the `MMap`'s memory disallowing any
     * other access.
     *
     * The lifetime of that memory becomes `'static` because the kernel will
     * unmap it only when the process dies.
     *
     * The usage of this method is discouraged for common uses because
     * causes memory leaks that is not possible to remove until the
     * process dies.
     *
     * It may also cause a deadlock for other processes if this is a
     * [`File backed MMap`] with active sync to the underling [`File`].
     *
     * The method consumes the instance and [forget] about it, so the kernel
     * will not close this object
     *
     * [`File backed MMap`]: /api/objs/impls/struct.File.html#method.mmap
     * [`File`]: /api/objs/impls/struct.File.html
     * [forget]: https://doc.rust-lang.org/std/mem/fn.forget.html
     */
    pub fn leak_ptr<T>(self) -> &'static mut [T] {
        let ref_slice = self.get(MMapPtrMode::Writeable)
                            .map(|(raw_ptr, size)| unsafe {
                                slice::from_raw_parts_mut(raw_ptr as *mut T,
                                                          size / size_of::<T>())
                            })
                            .unwrap();
        mem::forget(self);
        ref_slice
    }

    /** Returns whether this `MMap` instance originates from a
     * [`File::mmap()`] call
     *
     * [`File::mmap()`]: /api/objs/impls/struct.File.html#method.mmap
     */
    pub fn is_file_backed(&self) -> bool {
        self.kern_call_0(KernFnPath::MMap(KernMMapFnId::IsFile))
            .map(|res| res != 0)
            .unwrap_or(false)
    }

    /** Returns the raw pointer to the memory according to the given
     * [`MMapPtrMode`]
     *
     * [`MMapPtrMode`]: /api/bits/obj/enum.MMapPtrMode.html
     */
    fn get(&self, mode: MMapPtrMode) -> Result<(usize, usize)> {
        let mut size = 0;
        self.kern_call_2(KernFnPath::MMap(KernMMapFnId::GetPtr),
                         &mut size as *mut _ as usize,
                         mode.into())
            .map(|raw_ptr| (raw_ptr, size))
    }

    /** Drops the access to the pointer
     */
    fn drop_ptr(&self, was_mut: bool) {
        self.kern_call_1(KernFnPath::MMap(KernMMapFnId::DropPtr), was_mut as usize)
            .unwrap();
    }
}

/** # Immutable `MMap` Box
 *
 * Scoped box that holds the immutable reference to the [`MMap`]'s memory
 * allowing read-only access to it.
 *
 * This object is obtainable calling [`MMap::get_ptr()`]
 *
 * [`MMap`]: /api/objs/impls/struct.MMap.html
 * [`MMap::get_ptr()`]: /api/objs/impls/struct.MMap.html#method.get_ptr
 */
pub struct ConstMMapBox<'a, T> {
    m_mmap: &'a MMap,
    m_ref: &'a [T]
}

impl<'a, T> ConstMMapBox<'a, T> {
    /** # Constructs a new `ConstMMapBox`
     *
     * The instance will contain the slice reference to the memory
     */
    fn new(mmap: &'a MMap, ptr: *const T, size: usize) -> Self {
        unsafe {
            assert_eq!(size % size_of::<T>(), 0);
            Self { m_mmap: mmap,
                   m_ref: slice::from_raw_parts(ptr, size / size_of::<T>()) }
        }
    }
}

impl<'a, T> Deref for ConstMMapBox<'a, T> {
    /** The resulting type after dereference.
     */
    type Target = [T];

    /** Dereferences the value.
     */
    fn deref(&self) -> &Self::Target {
        self.m_ref
    }
}

impl<'a, T> Drop for ConstMMapBox<'a, T> {
    /** Releases the pointer unlocking a read slot of the `RLock`
     */
    fn drop(&mut self) {
        self.m_mmap.drop_ptr(false);
    }
}

/** # Mutable `MMap` Box
 *
 * Scoped box that holds the mutable reference to the [`MMap`]'s memory
 * allowing read/write access to it.
 *
 * This object is obtainable calling [`MMap::get_ptr_mut()`]
 *
 * [`MMap`]: /api/objs/impls/struct.MMap.html
 * [`MMap::get_ptr_mut()`]:
 * /api/objs/impls/struct.MMap.html#method.get_ptr_mut
 */
pub struct MutMMapBox<'a, T> {
    m_mmap: &'a MMap,
    m_ref: &'a mut [T]
}

impl<'a, T> MutMMapBox<'a, T> {
    /** # Constructs a new `MutMMapBox`
     *
     * The instance will contain the slice reference to the memory
     */
    fn new(mmap: &'a MMap, ptr: *mut T, size: usize) -> Self {
        unsafe {
            assert_eq!(size % size_of::<T>(), 0);
            Self { m_mmap: mmap,
                   m_ref: slice::from_raw_parts_mut(ptr, size / size_of::<T>()) }
        }
    }
}

impl<'a, T> Deref for MutMMapBox<'a, T> {
    /** The resulting type after dereference.
     */
    type Target = [T];

    /** Dereferences the value.
     */
    fn deref(&self) -> &Self::Target {
        self.m_ref
    }
}

impl<'a, T> DerefMut for MutMMapBox<'a, T> {
    /** Mutably dereferences the value.
     */
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.m_ref
    }
}

impl<'a, T> Drop for MutMMapBox<'a, T> {
    /** Releases the pointer unlocking the write slot of the `RLock`
     */
    fn drop(&mut self) {
        self.m_mmap.drop_ptr(true);
    }
}
