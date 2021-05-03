/*! # System Call Codes
 *
 * Lists the system call codes enumerations.
 *
 * A system call code is an identifier of 16 bits used by the kernel in
 * conjunction with the corresponding class to access the second layer of
 * system call routines and identify the proper system call to call
 */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/** # `ObjConfig` System Calls Codes
 *
 * Lists the system call codes for the [`ObjConfig`] struct
 *
 * [`ObjConfig`]: api::objs::config::ObjConfig
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernObjConfigFnId {
    ApplyConfig
}

/** # `TaskConfig` System Calls Codes
 *
 * Lists the system call codes for the [`TaskConfig`] struct
 *
 * [`TaskConfig`]: api::tasks::config::TaskConfig
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernTaskConfigFnId {
    CreateTask,
    InitFind
}

/** # `OSEntConfig` System Calls Codes
 *
 * Lists the system call codes for the [`OSEntConfig`] struct
 *
 * [`OSEntConfig`]: api::ents::config::OSEntConfig
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOSEntConfigFnId {
    CreateEntity,
    InitFind
}

/** # `Object` System Calls Codes
 *
 * Lists the system call codes for the [`Object`] trait
 *
 * [`Object`]: api::objs::object::Object
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernObjectFnId {
    AddRef,
    Drop,
    DropName,
    Info,
    UpdateInfo,
    Send,
    Recv,
    Watch,
    IsValid
}

/** # `Task` System Calls Codes
 *
 * Lists the system call codes for the [`Task`] trait
 *
 * [`Task`]: api::tasks::task::Task
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernTaskFnId {
    This,
    Terminate,
    Yield,
    IsAlive
}

/** # `Dir` System Calls Codes
 *
 * Lists the system call codes for the [`Dir`] struct
 *
 * [`Dir`]: api::objs::impls::dir::Dir
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernDirFnId {
    InitIter
}

/** # `File` System Calls Codes
 *
 * Lists the system call codes for the [`File`] struct
 *
 * [`File`]: api::objs::impls::file::File
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernFileFnId {
    ReadData,
    WriteData,
    Copy,
    Move,
    SetPos,
    MapToMem
}

/** # `IpcChan` System Calls Codes
 *
 * Lists the system call codes for the [`IpcChan`] struct
 *
 * [`IpcChan`]: api::objs::impls::ipc_chan::IpcChan
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernIpcChanFnId {
    Send,
    Recv
}

/** # `Iterator` System Calls Codes
 *
 * Lists the system call codes for the [`KrnIterator`] struct
 *
 * [`KrnIterator`]: api::objs::impls::iter::KrnIterator
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KrnIteratorFnId {
    NextValue,
    SetBeginToEndPos,
    SetEndToBeginPos
}

/** # `Link` System Calls Codes
 *
 * Lists the system call codes for the [`Link`] struct
 *
 * [`Link`]: api::objs::impls::link::Link
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernLinkFnId {
    Deref,
    ReferTo
}

/** # `MMap` System Calls Codes
 *
 * Lists the system call codes for the [`MMap`] struct
 *
 * [`MMap`]: api::objs::impls::mmap::MMap
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernMMapFnId {
    GetPtr,
    DropPtr,
    IsFile
}

/** # `Mutex` System Calls Codes
 *
 * Lists the system call codes for the [`Mutex`] struct
 *
 * [`Mutex`]: api::objs::impls::mutex::Mutex
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernMutexFnId {
    Lock,
    TryLock,
    Unlock,
    IsLocked
}

/** # `Instant` System Calls Codes
 *
 * Lists the system call codes for the [`Instant`] struct
 *
 * [`Instant`]: api::time::Instant
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernTimeInstFnId {
    Now
}

/** # `Path` System Calls Codes
 *
 * Lists the system call codes for the [`Path`] struct
 *
 * [`Path`]: api::path::Path
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernPathFnId {
    Exists
}

/** # `OSEntity` System Calls Codes
 *
 * Lists the system call codes for the [`OSEntity`] trait
 *
 * [`OSEntity`]: api::ents::entity::OSEntity
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOSEntFnId {
    Name
}

/** # `OSUser` System Calls Codes
 *
 * Lists the system call codes for the [`OSUser`] struct
 *
 * [`OSUser`]: api::ents::impls::user::OSUser
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOSUserFnId {
    Groups
}

/** # `OSGroup` System Calls Codes
 *
 * Lists the system call codes for the [`OSGroup`] struct
 *
 * [`OSGroup`]: api::ents::impls::group::OSGroup
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOSGroupFnId {
    AddUser
}

/** # `Proc` System Calls Codes
 *
 * Lists the system call codes for the [`Proc`] struct
 *
 * [`Proc`]: api::tasks::impls::proc::Proc
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernProcFnId {
    MainThread
}

/** # `Thread` System Calls Codes
 *
 * Lists the system call codes for the [`Thread`] struct
 *
 * [`Thread`]: api::tasks::impls::thread::Thread
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernThreadFnId {
    WaitFor,
    AddCleaner,
    CallbackReturn,
    GetEntryData
}
