/*! Supported filesystems types */

use core::{
    convert::TryFrom,
    fmt
};

/**
 * Lists the supported filesystem types by the MeetiX Kernel
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
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

impl Into<usize> for FsType {
    fn into(self) -> usize {
        self as usize
    }
}

impl TryFrom<usize> for FsType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FatX),
            1 => Ok(Self::CdROM),
            2 => Ok(Self::MeetiX),
            3 => Ok(Self::KernData),
            4 => Ok(Self::Devices),
            _ => Err(())
        }
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
