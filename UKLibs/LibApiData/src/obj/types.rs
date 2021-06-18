/*! `Object` types */

use core::fmt;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the available obj types represented by an `ObjId`
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum ObjType {
    /**
     * Default value
     */
    Unknown,

    /**
     * Identifies a `Device` obj
     */
    Device,

    /**
     * Identifies a `Dir` obj
     */
    Dir,

    /**
     * Identifies a `File` obj
     */
    File,

    /**
     * Identifies an `IpcChan` obj
     */
    IpcChan,

    /**
     * Identifies an `KrnIterator` obj
     */
    KrnIterator,

    /**
     * Identifies a `Link` obj
     */
    Link,

    /**
     * Identifies a `MMap` obj
     */
    MMap,

    /**
     * Identifies an `OsRawMutex` obj
     */
    OsRawMutex
}

impl Default for ObjType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for ObjType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Device => write!(f, "Device"),
            Self::Dir => write!(f, "Dir"),
            Self::File => write!(f, "File"),
            Self::IpcChan => write!(f, "IpcChan"),
            Self::KrnIterator => write!(f, "KrnIterator"),
            Self::Link => write!(f, "Link"),
            Self::MMap => write!(f, "MMap"),
            Self::OsRawMutex => write!(f, "OsRawMutex")
        }
    }
}
