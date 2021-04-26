/*! # Mutex Object
 *
 * Implements a shareable mutually exclusive gate
 */

use os::sysc::{codes::KernMutexFnId, fn_path::KernFnPath};
use sync::{GuardNoSend, RawMutex};

use crate::{
    bits::obj::ObjType,
    caller::KernCaller,
    objs::{ObjId, Object, UserCreatable}
};

/** # Mutual Exclusion Gate
 *
 * Represents a generic container that uses an [`OsRawMutex`] to ensure
 * mutual exclusive access to the value held.
 *
 * Refer to the [`sync::Mutex`] for complete documentation
 *
 * [`OsRawMutex`]: /api/objs/impls/struct.OsRawMutex.html
 * [`sync::Mutex`]: /sync/struct.Mutex.html
 */
pub type Mutex<T> = sync::Mutex<OsRawMutex, T>;

/** # Mutex Guard Box
 *
 * Scoped box that allow access to the data when the [`Mutex`] is locked
 *
 * [`Mutex`]: /api/objs/impls/type.Mutex.html
 */
pub type MutexGuard<'a, T> = sync::MutexGuard<'a, OsRawMutex, T>;

impl_obj_id_object! {
    /** # Raw Mutex
     *
     * Represents a reference to an open mutex.
     *
     * It allows inter-process locking because, like all the other objects,
     * is representable into the VFS tree; so if at least two different
     * tasks tries to lock it, one of them acquire it, the other(s) will
     * wait until the mutex will be unlocked
     */
    pub struct OsRawMutex : impl UserCreatable {
        where TYPE = ObjType::OsRawMutex;
    }
}

impl OsRawMutex {
    /** # Construct an uninitialized `OsRawMutex`
     *
     * Const stub used only to satisfy the `INIT` constant requirement of
     * [`RawMutex`]
     *
     * [`RawMutex`]: https://docs.rs/lock_api/0.4.2/lock_api/struct.Mutex.html
     */
    const fn const_init_for_raw() -> Self {
        Self(ObjId::const_new())
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
     * [`Mutex`]: /api/objs/impls/type.Mutex.html
     */
    pub const fn into_mutex<T>(self, value: T) -> Mutex<T> {
        Mutex::const_new(self, value)
    }
}

unsafe impl RawMutex for OsRawMutex {
    /** Create [`Mutex`] with this constant will throw [`panic!()`] at first
     * call
     *
     * [`Mutex`]: /api/objs/impls/type.Mutex.html
     * [`panic!()`]: https://doc.rust-lang.org/std/macro.panic.html
     */
    const INIT: Self = Self::const_init_for_raw();

    /** The [`Mutex`] cannot be send across different threads using rust
     * primitives, because each thread have his own open object table, but
     * is obviously possible using [`Object::send()`] system call
     *
     * [`Mutex`]: /api/objs/impls/type.Mutex.html
     * [`Object::send()`]: /api/objs/trait.Object.html#method.send
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
