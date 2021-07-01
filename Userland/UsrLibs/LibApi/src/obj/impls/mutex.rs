/*! Open Mutex `Object` */

use api_data::{
    error::OsError,
    obj::types::ObjType,
    sys::{
        codes::KernMutexFnId,
        fn_path::KernFnPath
    }
};
use sync::{
    guards::{
        LockGuardNonSendable,
        LockGuardShareability
    },
    mutex::{
        BackRawMutex,
        CreatMayFailBackRawMutex
    }
};

use crate::obj::{
    AnonymousObject,
    ObjHandle,
    Object,
    UserCreatableObject
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

#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct OsRawMutex {
    m_obj_handle: ObjHandle
}

impl CreatMayFailBackRawMutex for OsRawMutex {
    type CreatError = OsError;

    fn try_creat() -> Result<Self, Self::CreatError>
        where Self: Sized {
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

    #[inline]
    fn do_lock(&self) {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::Mutex(KernMutexFnId::Lock))
            .unwrap_or_else(|os_err| {
                panic!("Failed to OsRawMutex::do_lock {:?}: cause: {}", self, os_err)
            });
    }

    #[inline]
    fn do_try_lock(&self) -> bool {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::Mutex(KernMutexFnId::TryLock))
            .map(|res| res != 0)
            .unwrap_or_else(|os_err| {
                panic!("Failed to OsRawMutex::do_try_lock {:?}: cause: {}", self, os_err)
            })
    }

    #[inline]
    unsafe fn do_unlock(&self) {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::Mutex(KernMutexFnId::Unlock))
            .unwrap_or_else(|os_err| {
                panic!("Failed to OsRawMutex::do_unlock {:?}: cause: {}", self, os_err)
            });
    }

    #[inline]
    fn do_is_locked(&self) -> bool {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::Mutex(KernMutexFnId::IsLocked))
            .map(|res| res != 0)
            .unwrap_or_else(|os_err| {
                panic!("Failed to OsRawMutex::do_is_locked {:?}: cause: {}", self, os_err)
            })
    }
}

impl From<ObjHandle> for OsRawMutex {
    fn from(obj_handle: ObjHandle) -> Self {
        Self { m_obj_handle: obj_handle }
    }
}

impl Object for OsRawMutex {
    const TYPE: ObjType = ObjType::OsRawMutex;

    #[inline]
    fn obj_handle(&self) -> &ObjHandle {
        &self.m_obj_handle
    }

    #[inline]
    fn obj_handle_mut(&mut self) -> &mut ObjHandle {
        &mut self.m_obj_handle
    }
}

impl UserCreatableObject for OsRawMutex {
    /* No methods to implement */
}

impl AnonymousObject for OsRawMutex {
    /* No methods to implement */
}
