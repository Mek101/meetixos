/*! Kernel function call codes */

use core::convert::TryFrom;

/**
 * Lists the system call codes for the `KernHandle` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernHandleFnId {
    IsValid,
    Clone,
    Drop
}

impl Into<u16> for KernHandleFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernHandleFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::IsValid),
            1 => Ok(Self::Clone),
            2 => Ok(Self::Drop),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `ObjConfig` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernObjConfigFnId {
    ApplyConfig
}

impl Into<u16> for KernObjConfigFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernObjConfigFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ApplyConfig),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `TaskConfig` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernTaskConfigFnId {
    ApplyConfig
}

impl Into<u16> for KernTaskConfigFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernTaskConfigFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ApplyConfig),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `OsEntConfig` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernOsEntConfigFnId {
    ApplyConfig
}

impl Into<u16> for KernOsEntConfigFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernOsEntConfigFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ApplyConfig),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `OsEntity` trait
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernOsEntFnId {
    OsId,
    Name
}

impl Into<u16> for KernOsEntFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernOsEntFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OsId),
            1 => Ok(Self::Name),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `Object` trait
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernObjectFnId {
    DropName,
    Info,
    UpdateInfo,
    Send,
    Recv,
    Watch,
    IsValid
}

impl Into<u16> for KernObjectFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernObjectFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DropName),
            1 => Ok(Self::Info),
            2 => Ok(Self::UpdateInfo),
            3 => Ok(Self::Send),
            4 => Ok(Self::Recv),
            5 => Ok(Self::Watch),
            6 => Ok(Self::IsValid),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `Task` trait
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernTaskFnId {
    OsId,
    This,
    Exit,
    Kill,
    Yield,
    IsAlive
}

impl Into<u16> for KernTaskFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernTaskFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OsId),
            1 => Ok(Self::This),
            2 => Ok(Self::Exit),
            3 => Ok(Self::Kill),
            4 => Ok(Self::Yield),
            5 => Ok(Self::IsAlive),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `Device` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernDeviceFnId {
    Read,
    Write,
    SetPos,
    MapToMem,
    IOSetup
}

impl Into<u16> for KernDeviceFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernDeviceFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Read),
            1 => Ok(Self::Write),
            2 => Ok(Self::SetPos),
            3 => Ok(Self::MapToMem),
            4 => Ok(Self::IOSetup),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `Dir` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernDirFnId {
    NextChild,
    SetPos
}

impl Into<u16> for KernDirFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernDirFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NextChild),
            1 => Ok(Self::SetPos),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `File` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernFileFnId {
    ReadData,
    WriteData,
    Copy,
    Move,
    SetPos,
    MapToMem
}

impl Into<u16> for KernFileFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernFileFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ReadData),
            1 => Ok(Self::WriteData),
            2 => Ok(Self::Copy),
            3 => Ok(Self::Move),
            4 => Ok(Self::SetPos),
            5 => Ok(Self::MapToMem),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `IpcChan` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernIpcChanFnId {
    Send,
    Recv
}

impl Into<u16> for KernIpcChanFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernIpcChanFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Send),
            1 => Ok(Self::Recv),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `Link` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernLinkFnId {
    Deref,
    BindTo
}

impl Into<u16> for KernLinkFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernLinkFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Deref),
            1 => Ok(Self::BindTo),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `MMap` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernMMapFnId {
    GetPtr,
    DropPtr
}

impl Into<u16> for KernMMapFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernMMapFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GetPtr),
            1 => Ok(Self::DropPtr),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `Mutex` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernMutexFnId {
    Lock,
    TryLock,
    Unlock,
    IsLocked
}

impl Into<u16> for KernMutexFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernMutexFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Lock),
            1 => Ok(Self::TryLock),
            2 => Ok(Self::Unlock),
            3 => Ok(Self::IsLocked),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `Instant` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernInstantFnId {
    Now
}

impl Into<u16> for KernInstantFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernInstantFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Now),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `Path` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernPathFnId {
    Exists
}

impl Into<u16> for KernPathFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernPathFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Exists),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `OsUser` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernOsUserFnId {
    GroupsIds,
    GroupsCount
}

impl Into<u16> for KernOsUserFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernOsUserFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GroupsIds),
            1 => Ok(Self::GroupsCount),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `OsGroup` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernOsGroupFnId {
    AddUser,
    UsersIds,
    UsersCount
}

impl Into<u16> for KernOsGroupFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernOsGroupFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AddUser),
            1 => Ok(Self::UsersIds),
            2 => Ok(Self::UsersCount),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `Proc` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernProcFnId {
    OsUser,
    OsGroup,
    GetWorkDir,
    SetWorkDir,
    MainThread,
    SubThreads,
    ThreadsCount,
    Mount,
    UnMount
}

impl Into<u16> for KernProcFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernProcFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OsUser),
            1 => Ok(Self::OsGroup),
            2 => Ok(Self::GetWorkDir),
            3 => Ok(Self::SetWorkDir),
            4 => Ok(Self::MainThread),
            5 => Ok(Self::SubThreads),
            6 => Ok(Self::ThreadsCount),
            7 => Ok(Self::Mount),
            8 => Ok(Self::UnMount),
            _ => Err(())
        }
    }
}

/**
 * Lists the system call codes for the `Thread` struct
 */
#[repr(u16)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum KernThreadFnId {
    Join,
    Pause,
    Sleep,
    WaitIrq,
    Resume,
    AddCleaner,
    CallbackReturn,
    GetEntryData
}

impl Into<u16> for KernThreadFnId {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for KernThreadFnId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Join),
            1 => Ok(Self::Pause),
            2 => Ok(Self::Sleep),
            3 => Ok(Self::WaitIrq),
            4 => Ok(Self::Resume),
            5 => Ok(Self::AddCleaner),
            6 => Ok(Self::CallbackReturn),
            7 => Ok(Self::GetEntryData),
            _ => Err(())
        }
    }
}
