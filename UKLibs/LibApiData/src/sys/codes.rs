/*! Kernel function call codes */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the system call codes for the `KernHandle` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernHandleFnId {
    IsValid,
    Clone,
    Drop
}

/**
 * Lists the system call codes for the `ObjConfig` struct
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
 * Lists the system call codes for the `TaskConfig` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernTaskConfigFnId {
    ApplyConfig
}

/**
 * Lists the system call codes for the `OSEntConfig` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOsEntConfigFnId {
    ApplyConfig
}

/**
 * Lists the system call codes for the `Object` trait
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernObjectFnId {
    DropName,
    Info,
    UpdateInfo,
    Send,
    Recv,
    Watch,
    IsValid
}

/**
 * Lists the system call codes for the `Task` trait
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernTaskFnId {
    This,
    Terminate,
    Yield
}

/**
 * Lists the system call codes for the `Device` struct
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
 * Lists the system call codes for the `Dir` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernDirFnId {
    NextChild,
    SetPos
}

/**
 * Lists the system call codes for the `File` struct
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
 * Lists the system call codes for the `IpcChan` struct
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
 * Lists the system call codes for the `Link` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernLinkFnId {
    Deref,
    BindTo
}

/**
 * Lists the system call codes for the `MMap` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernMMapFnId {
    GetPtr,
    DropPtr,
    IsFile,
    IsDevice
}

/**
 * Lists the system call codes for the `Mutex` struct
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
 * Lists the system call codes for the `Instant` struct
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
 * Lists the system call codes for the `Path` struct
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
 * Lists the system call codes for the `OsEntity` trait
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOsEntFnId {
    OsId,
    Name
}

/**
 * Lists the system call codes for the `OsUser` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOsUserFnId {
    Groups,
    GroupsCount
}

/**
 * Lists the system call codes for the `OsGroup` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KernOsGroupFnId {
    AddUser,
    Users,
    UsersCount
}

/**
 * Lists the system call codes for the `Proc` struct
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
 * Lists the system call codes for the `Thread` struct
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
