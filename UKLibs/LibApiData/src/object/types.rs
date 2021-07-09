/*! `Object` types */

use core::fmt;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the available object types represented by a `Object`
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum ObjType {
    /**
     * Default value
     */
    Unknown,

    /**
     * Identifies a `Device` object
     */
    Device,

    /**
     * Identifies a `Dir` object
     */
    Dir,

    /**
     * Identifies a `File` object
     */
    File,

    /**
     * Identifies an `IpcChan` object
     */
    IpcChan,

    /**
     * Identifies a `Link` object
     */
    Link,

    /**
     * Identifies a `MMap` object
     */
    MMap,

    /**
     * Identifies an `OsRawMutex` object
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
            Self::Link => write!(f, "Link"),
            Self::MMap => write!(f, "MMap"),
            Self::OsRawMutex => write!(f, "OsRawMutex")
        }
    }
}
