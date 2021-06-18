/*! Open Mutex `Object` */

use os::sysc::{
    codes::KernMutexFnId,
    fn_path::KernFnPath
};
use sync::{
    guards::LockGuardNonSendable,
    mutex::{
        BackRawMutex,
        FallibleCreatBackRawMutex
    }
};

use crate::{
    bits::obj::types::ObjType,
    caller::KernCaller,
    errors::error::OsError,
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
pub type OsMutex<T> = sync::mutex::Mutex<OsRawMutex, T>;

/**
 * RAII box that allow access to the data when the `Mutex` is locked
 */
pub type OsMutexGuard<'a, T> = sync::mutex::guard::MutexDataGuard<'a, OsRawMutex, T>;

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
     * Constructs an `OsMutex` from an already existing `OsRawMutex`
     */
    pub const fn into_os_mutex<T>(self, value: T) -> OsMutex<T> {
        OsMutex::raw_new(self, value)
    }
}

impl FallibleCreatBackRawMutex for OsRawMutex {
    type CreatError = OsError;

    fn creat_raw() -> Result<Self, Self::CreatError> {
        Self::creat().for_read().for_write().apply_for_anon()
    }
}

unsafe impl BackRawMutex for OsRawMutex {
    /**
     * The `Mutex` cannot be send across different threads using rust
     * primitives, because each thread have his own open obj table, but
     * is obviously possible using `Object::send()` system call
     */
    type LockGuardShareabilityMark = LockGuardNonSendable;

    fn do_lock(&self) {
        self.kern_call_0(KernFnPath::Mutex(KernMutexFnId::Lock))
            .unwrap_or_else(|err| panic!("Failed to OsRawMutex::do_lock {:?}: cause: {}", self, err));
    }

    fn do_try_lock(&self) -> bool {
        self.kern_call_0(KernFnPath::Mutex(KernMutexFnId::TryLock))
            .map(|res| res != 0)
            .unwrap_or_else(|err| {
                panic!("Failed to OsRawMutex::do_try_lock {:?}: cause: {}", self, err)
            })
    }

    unsafe fn do_unlock(&self) {
        self.kern_call_0(KernFnPath::Mutex(KernMutexFnId::Unlock))
            .unwrap_or_else(|err| panic!("Failed to OsRawMutex::do_unlock {:?}: cause: {}", self, err));
    }

    fn do_is_locked(&self) -> bool {
        self.kern_call_0(KernFnPath::Mutex(KernMutexFnId::IsLocked))
            .map(|res| res != 0)
            .unwrap_or_else(|err| {
                panic!("Failed to OsRawMutex::do_is_locked {:?}: cause: {}", self, err)
            })
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
