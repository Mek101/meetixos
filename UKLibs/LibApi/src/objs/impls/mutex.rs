/*! Open Mutex `Object` */

use os::sysc::{
    codes::KernMutexFnId,
    fn_path::KernFnPath
};
use sync::{
    GuardNoSend,
    RawMutex
};

use crate::{
    bits::obj::types::ObjType,
    caller::KernCaller,
    objs::object::{
        ObjId,
        Object,
        UserCreatable
    }
};

/**
 * Generic container that uses an `OsRawMutex` to ensure mutual exclusive
 * access to the value held
 */
pub type Mutex<T> = sync::Mutex<OsRawMutex, T>;

/**
 * RAII box that allow access to the data when the `Mutex` is locked
 */
pub type MutexGuard<'a, T> = sync::MutexGuard<'a, OsRawMutex, T>;

/**
 * Reference to an open mutex on the VFS.
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
    /**
     * Const stub used only to satisfy the `INIT` constant requirement of
     * the `RawMutex` trait
     */
    const fn const_init_for_raw() -> Self {
        Self { m_handle: ObjId::const_new() }
    }

    /**
     * The `Mutex` created with this `OsRawMutex` contains `value`.
     *
     * If another process references the same `OsRawMutex` will have no
     * access to the stored `value`.
     *
     * The method consumes the instance because move it into the `Mutex`
     */
    pub const fn into_mutex<T>(self, value: T) -> Mutex<T> {
        Mutex::const_new(self, value)
    }
}

unsafe impl RawMutex for OsRawMutex {
    /**
     * Use a `Mutex` which relies on this `RawMutex` implementation and
     * created with this constant will `panic!()` at first call
     */
    const INIT: Self = Self::const_init_for_raw();

    /**
     * The `Mutex` cannot be send across different threads using rust
     * primitives, because each thread have his own open object table, but
     * is obviously possible using `Object::send()` system call
     */
    type GuardMarker = GuardNoSend;

    fn lock(&self) {
        let _ = self.kern_call_0(KernFnPath::Mutex(KernMutexFnId::Lock)).unwrap();
    }

    fn try_lock(&self) -> bool {
        self.kern_call_0(KernFnPath::Mutex(KernMutexFnId::TryLock))
            .map(|res| res != 0)
            .unwrap_or(false)
    }

    unsafe fn unlock(&self) {
        let _ = self.kern_call_0(KernFnPath::Mutex(KernMutexFnId::Unlock)).unwrap();
    }

    fn is_locked(&self) -> bool {
        self.kern_call_0(KernFnPath::Mutex(KernMutexFnId::IsLocked))
            .map(|res| res != 0)
            .unwrap_or(false)
    }
}

impl Object for OsRawMutex {
    const TYPE: ObjType = ObjType::OsRawMutex;

    fn obj_handle(&self) -> &ObjId {
        &self.m_handle
    }

    fn obj_handle_mut(&mut self) -> &mut ObjId {
        &mut self.m_handle
    }
}

impl From<ObjId> for OsRawMutex {
    fn from(id: ObjId) -> Self {
        Self { m_handle: id }
    }
}

impl KernCaller for OsRawMutex {
    fn caller_handle_bits(&self) -> u32 {
        self.obj_handle().caller_handle_bits()
    }
}

impl UserCreatable for OsRawMutex {
    /* No methods to implement */
}
