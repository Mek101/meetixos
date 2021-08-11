/*! `Object` types */

use core::{
    convert::TryFrom,
    fmt
};

/**
 * Lists the available object types represented by a `Object`
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
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

impl Into<usize> for ObjType {
    fn into(self) -> usize {
        self as usize
    }
}

impl TryFrom<usize> for ObjType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Device),
            2 => Ok(Self::Dir),
            3 => Ok(Self::File),
            4 => Ok(Self::IpcChan),
            5 => Ok(Self::Link),
            6 => Ok(Self::MMap),
            7 => Ok(Self::OsRawMutex),
            _ => Err(())
        }
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
