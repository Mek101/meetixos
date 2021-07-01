/*! Kernel function call paths */

use core::fmt;

use crate::sys::codes::{
    KernDeviceFnId,
    KernDirFnId,
    KernFileFnId,
    KernHandleFnId,
    KernInstantFnId,
    KernIpcChanFnId,
    KernLinkFnId,
    KernMMapFnId,
    KernMutexFnId,
    KernObjConfigFnId,
    KernObjectFnId,
    KernOsEntConfigFnId,
    KernOsEntFnId,
    KernOsGroupFnId,
    KernOsUserFnId,
    KernPathFnId,
    KernProcFnId,
    KernTaskConfigFnId,
    KernTaskFnId,
    KernThreadFnId
};

/**
 * Lists the callable Kernel function paths.
 *
 * Each variant represent a Kernel call class, which is the primary key of
 * the Kernel's routines table, and each class contains the specific codes
 * for the call class, which is the secondary key of the Kernel's routines
 * table
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum KernFnPath {
    KernHandle(KernHandleFnId),
    ObjConfig(KernObjConfigFnId),
    TaskConfig(KernTaskConfigFnId),
    OsEntConfig(KernOsEntConfigFnId),
    Object(KernObjectFnId),
    Task(KernTaskFnId),
    Device(KernDeviceFnId),
    Dir(KernDirFnId),
    File(KernFileFnId),
    IpcChan(KernIpcChanFnId),
    Link(KernLinkFnId),
    MMap(KernMMapFnId),
    Mutex(KernMutexFnId),
    Instant(KernInstantFnId),
    Path(KernPathFnId),
    OsEntity(KernOsEntFnId),
    OsUser(KernOsUserFnId),
    OsGroup(KernOsGroupFnId),
    Proc(KernProcFnId),
    Thread(KernThreadFnId),
    Invalid
}

impl KernFnPath {
    /**
     * Returns the current function class variant as `u16`
     */
    pub fn raw_fn_class(&self) -> u16 {
        match self {
            Self::KernHandle(_) => 0,
            Self::ObjConfig(_) => 1,
            Self::TaskConfig(_) => 2,
            Self::OsEntConfig(_) => 3,
            Self::Object(_) => 4,
            Self::Task(_) => 5,
            Self::Device(_) => 6,
            Self::Dir(_) => 7,
            Self::File(_) => 8,
            Self::IpcChan(_) => 9,
            Self::Link(_) => 10,
            Self::MMap(_) => 11,
            Self::Mutex(_) => 12,
            Self::Instant(_) => 13,
            Self::Path(_) => 14,
            Self::OsEntity(_) => 15,
            Self::OsUser(_) => 16,
            Self::OsGroup(_) => 17,
            Self::Proc(_) => 18,
            Self::Thread(_) => 19,
            _ => u16::MAX
        }
    }

    /**
     * Returns the current function id as `u16`
     */
    pub fn raw_fn_id(&self) -> u16 {
        match *self {
            Self::KernHandle(fn_id) => fn_id.into(),
            Self::ObjConfig(fn_id) => fn_id.into(),
            Self::TaskConfig(fn_id) => fn_id.into(),
            Self::OsEntConfig(fn_id) => fn_id.into(),
            Self::Object(fn_id) => fn_id.into(),
            Self::Task(fn_id) => fn_id.into(),
            Self::Device(fn_id) => fn_id.into(),
            Self::Dir(fn_id) => fn_id.into(),
            Self::File(fn_id) => fn_id.into(),
            Self::IpcChan(fn_id) => fn_id.into(),
            Self::Link(fn_id) => fn_id.into(),
            Self::MMap(fn_id) => fn_id.into(),
            Self::Mutex(fn_id) => fn_id.into(),
            Self::Instant(fn_id) => fn_id.into(),
            Self::Path(fn_id) => fn_id.into(),
            Self::OsEntity(fn_id) => fn_id.into(),
            Self::OsUser(fn_id) => fn_id.into(),
            Self::OsGroup(fn_id) => fn_id.into(),
            Self::Proc(fn_id) => fn_id.into(),
            Self::Thread(fn_id) => fn_id.into(),
            _ => u16::MAX
        }
    }
}

impl Default for KernFnPath {
    fn default() -> Self {
        Self::Invalid
    }
}

impl fmt::Display for KernFnPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KernHandle(fn_id) => write!(f, "KernFnPath::KernHandle({:?})", fn_id),
            Self::ObjConfig(fn_id) => write!(f, "KernFnPath::ObjConfig({:?})", fn_id),
            Self::TaskConfig(fn_id) => write!(f, "KernFnPath::TaskConfig({:?})", fn_id),
            Self::OsEntConfig(fn_id) => write!(f, "KernFnPath::OSEntConfig({:?})", fn_id),
            Self::Object(fn_id) => write!(f, "KernFnPath::Object({:?})", fn_id),
            Self::Task(fn_id) => write!(f, "KernFnPath::Task({:?})", fn_id),
            Self::Device(fn_id) => write!(f, "KernFnPath::Device({:?})", fn_id),
            Self::Dir(fn_id) => write!(f, "KernFnPath::Dir({:?})", fn_id),
            Self::File(fn_id) => write!(f, "KernFnPath::File({:?})", fn_id),
            Self::IpcChan(fn_id) => write!(f, "KernFnPath::IpcChan({:?})", fn_id),
            Self::Link(fn_id) => write!(f, "KernFnPath::Link({:?})", fn_id),
            Self::MMap(fn_id) => write!(f, "KernFnPath::MMap({:?})", fn_id),
            Self::Mutex(fn_id) => write!(f, "KernFnPath::Mutex({:?})", fn_id),
            Self::Instant(fn_id) => write!(f, "KernFnPath::Time({:?})", fn_id),
            Self::Path(fn_id) => write!(f, "KernFnPath::Path({:?})", fn_id),
            Self::OsEntity(fn_id) => write!(f, "KernFnPath::OSEntity({:?})", fn_id),
            Self::OsUser(fn_id) => write!(f, "KernFnPath::OSUser({:?})", fn_id),
            Self::OsGroup(fn_id) => write!(f, "KernFnPath::OSGroup({:?})", fn_id),
            Self::Proc(fn_id) => write!(f, "KernFnPath::Proc({:?})", fn_id),
            Self::Thread(fn_id) => write!(f, "KernFnPath::Thread({:?})", fn_id),
            Self::Invalid => write!(f, "KernFnPath::Invalid")
        }
    }
}
