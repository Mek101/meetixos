/*! `Object` types */

use core::fmt;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the available object types represented by an `ObjId`
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
     * Identifies a `File` object
     */
    File,

    /**
     * Identifies a `Dir` object
     */
    Dir,

    /**
     * Identifies a `Link` object
     */
    Link,

    /**
     * Identifies a `MMap` object
     */
    MMap,

    /**
     * Identifies an `IpcChan` object
     */
    IpcChan,

    /**
     * Identifies an `OsRawMutex` object
     */
    OsRawMutex,

    /**
     * Identifies an `KrnIterator` object
     */
    KrnIterator
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
            Self::File => write!(f, "File"),
            Self::Dir => write!(f, "Dir"),
            Self::Link => write!(f, "Link"),
            Self::MMap => write!(f, "MMap"),
            Self::IpcChan => write!(f, "IpcChan"),
            Self::OsRawMutex => write!(f, "OsRawMutex"),
            Self::KrnIterator => write!(f, "KrnIterator")
        }
    }
}
