/*! # Mutex Object
 *
 * Implements a shareable mutually exclusive gate
 */

use os::sysc::{
    codes::KernMutexFnId,
    fn_path::KernFnPath
};
use sync::{
    GuardNoSend,
    RawMutex
};

use crate::{
    bits::obj::ObjType,
    caller::KernCaller,
    objs::{
        ObjId,
        Object,
        UserCreatable
    }
};

/** # Mutual Exclusion Gate
 *
 * Represents a generic container that uses an [`OsRawMutex`] to ensure
 * mutual exclusive access to the value held.
 *
 * Refer to the [`sync::Mutex`] for complete documentation
 *
 * [`OsRawMutex`]: crate::objs::impls::OsRawMutex
 * [`sync::Mutex`]: sync::Mutex
 */
pub type Mutex<T> = sync::Mutex<OsRawMutex, T>;

/** # Mutex Guard Box
 *
 * Scoped box that allow access to the data when the [`Mutex`] is locked
 *
 * [`Mutex`]: crate::objs::impls::Mutex
 */
pub type MutexGuard<'a, T> = sync::MutexGuard<'a, OsRawMutex, T>;

/** # Raw Mutex
 *
 * Represents a reference to an open mutex.
 *
 * It allows inter-process locking because, like all the other objects,
 * is representable into the VFS tree; so if at least two different
 * tasks tries to lock it, one of them acquire it, the other(s) will
 * wait until the mutex will be unlocked
 */
#[repr(transparent)]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct OsRawMutex {
    m_handle: ObjId
}

impl OsRawMutex {
    /** # Construct an uninitialized `OsRawMutex`
     *
     * Const stub used only to satisfy the `INIT` constant requirement of
     * [`RawMutex`]
     *
     * [`RawMutex`]: sync::RawMutex
     */
    const fn const_init_for_raw() -> Self {
        Self { m_handle: ObjId::const_new() }
    }

    /** # Constructs a `Mutex`
     *
     * The [`Mutex`] created with this `OsRawMutex` contains `value`.
     *
     * If another process references the same `OsRawMutex` will have no
     * access to the stored `value`.
     *
     * The method consumes the instance because move it into the [`Mutex`]
     *
     * [`Mutex`]: crate::objs::impls::Mutex
     */
    pub const fn into_mutex<T>(self, value: T) -> Mutex<T> {
        Mutex::const_new(self, value)
    }
}

unsafe impl RawMutex for OsRawMutex {
    /** Create [`Mutex`] with this constant will throw [`panic!()`] at first
     * call
     *
     * [`Mutex`]: crate::objs::impls::Mutex
     * [`panic!()`]: core::panic!
     */
    const INIT: Self = Self::const_init_for_raw();

    /** The [`Mutex`] cannot be send across different threads using rust
     * primitives, because each thread have his own open object table, but
     * is obviously possible using [`Object::send()`] system call
     *
     * [`Mutex`]: crate::objs::impls::Mutex
     * [`Object::send()`]: crate::objs::Object::send
     */
    type GuardMarker = GuardNoSend;

    /** Acquires this mutex, blocking the current thread until it is able to
     * do so.
     */
    fn lock(&self) {
        let _res = self.kern_call_0(KernFnPath::Mutex(KernMutexFnId::Lock)).unwrap();
    }

    /** Attempts to acquire this mutex without blocking. Returns true if the
     * lock was successfully acquired and false otherwise.
     */
    fn try_lock(&self) -> bool {
        self.kern_call_0(KernFnPath::Mutex(KernMutexFnId::TryLock))
            .map(|res| res != 0)
            .unwrap_or(false)
    }

    /** Unlocks this mutex.
     */
    unsafe fn unlock(&self) {
        let _res = self.kern_call_0(KernFnPath::Mutex(KernMutexFnId::Unlock)).unwrap();
    }

    /** Checks whether the mutex is currently locked.
     */
    fn is_locked(&self) -> bool {
        self.kern_call_0(KernFnPath::Mutex(KernMutexFnId::IsLocked))
            .map(|res| res != 0)
            .unwrap_or(false)
    }
}

impl Object for OsRawMutex {
    /** The value of the [`ObjType`] that matches the implementation
     *
     * [`ObjType`]: crate::bits::obj::types::ObjType
     */
    const TYPE: ObjType = ObjType::OsRawMutex;

    /** Returns the immutable reference to the underling [`ObjId`] instance
     *
     * [`ObjId`]: crate::objs::ObjId
     */
    fn obj_handle(&self) -> &ObjId {
        &self.m_handle
    }

    /** Returns the mutable reference to the underling [`ObjId`] instance
     *
     * [`ObjId`]: crate::objs::ObjId
     */
    fn obj_handle_mut(&mut self) -> &mut ObjId {
        &mut self.m_handle
    }
}

impl From<ObjId> for OsRawMutex {
    /** Performs the conversion
     */
    fn from(id: ObjId) -> Self {
        Self { m_handle: id }
    }
}

impl KernCaller for OsRawMutex {
    /** Returns the upper 32bits of the 64bit identifier of a system call
     */
    fn caller_handle_bits(&self) -> u32 {
        self.obj_handle().caller_handle_bits()
    }
}

impl UserCreatable for OsRawMutex {
    /* No methods to implement */
}
