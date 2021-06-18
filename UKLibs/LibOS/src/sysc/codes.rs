/*! Kernel function call codes */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the system call codes for the `LibApi::objs::ObjConfig` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernObjConfigFnId {
    ApplyConfig
}

/**
 * Lists the system call codes for the `LibApi::tasks::TaskConfig` struct
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

/**
 * Lists the system call codes for the `LibApi::ent::OSEntConfig` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOsEntConfigFnId {
    CreateEntity,
    InitFind
}

/**
 * Lists the system call codes for the `LibApi::objs::Object` trait
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

/**
 * Lists the system call codes for the `LibApi::tasks::Task` trait
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

/**
 * Lists the system call codes for the `LibApi::objs::impls::Device` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernDeviceFnId {
    Read,
    Write,
    SetPos,
    MapToMem,
    IOSetup
}

/**
 * Lists the system call codes for the `LibApi::objs::impls::Dir` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernDirFnId {
    InitIter
}

/**
 * Lists the system call codes for the `LibApi::objs::impls::File` struct
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

/**
 * Lists the system call codes for the `LibApi::objs::impls::IpcChan` struct
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

/**
 * Lists the system call codes for the `LibApi::objs::impls::KrnIterator`
 * struct
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

/**
 * Lists the system call codes for the `LibApi::objs::impls::Link` struct
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

/**
 * Lists the system call codes for the `LibApi::objs::impls::MMap` struct
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

/**
 * Lists the system call codes for the `LibApi::objs::impls::Mutex` struct
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

/**
 * Lists the system call codes for the `LibApi::time::Instant` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernTimeInstFnId {
    Now
}

/**
 * Lists the system call codes for the `LibApi::path::Path` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernPathFnId {
    Exists
}

/**
 * Lists the system call codes for the `LibApi::ent::OSEntity` trait
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOsEntFnId {
    Name
}

/**
 * Lists the system call codes for the `LibApi::ent::impls::OSUser` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOsUserFnId {
    Groups
}

/**
 * Lists the system call codes for the `LibApi::ent::impls::OSGroup` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOsGroupFnId {
    AddUser
}

/**
 * Lists the system call codes for the `LibApi::tasks::impls::Proc` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernProcFnId {
    MainThread,
    Mount,
    UnMount
}

/**
 * Lists the system call codes for the `LibApi::tasks::impls::Thread` struct
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
