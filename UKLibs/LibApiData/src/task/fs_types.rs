/*! Supported filesystems types */

use core::fmt;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the supported filesystem types by the MeetiX Kernel
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum FsType {
    /**
     * Old `File Allocation Table` filesystem.
     *
     * It needs a valid block `Device` to read/write from/to
     */
    FatX,

    /**
     * Old `CDROM` filesystem.
     *
     * It needs a valid block `Device` to read/write from/to
     */
    CdROM,

    /**
     * Standard MeetiX's filesystem.
     *
     * It needs a valid block `Device` to read/write from/to
     */
    MeetiX,

    /**
     * Virtual Kernel Data information filesystem.
     *
     * As virtual filesystem reads/writes the data using the Kernel
     * internal APIs
     */
    KernData,

    /**
     * Virtual `Device` collection filesystem.
     *
     * As virtual filesystem reads/writes the data using the Kernel
     * internal APIs
     */
    Devices
}

impl Default for FsType {
    fn default() -> Self {
        Self::MeetiX
    }
}

impl fmt::Display for FsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FatX => write!(f, "FatX Filesystem"),
            Self::CdROM => write!(f, "CdROM Filesystem"),
            Self::MeetiX => write!(f, "MeetiX Filesystem"),
            Self::KernData => write!(f, "KernelData Filesystem"),
            Self::Devices => write!(f, "Devices Filesystem")
        }
    }
}
