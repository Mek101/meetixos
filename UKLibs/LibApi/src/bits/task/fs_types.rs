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
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum FSType {
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
     * Virtual `Proc` information filesystem.
     *
     * As virtual filesystem reads/writes the data using the Kernel internal
     * APIs
     */
    Proc,

    /**
     * Virtual `Device` collection filesystem.
     *
     * As virtual filesystem reads/writes the data using the Kernel internal
     * APIs
     */
    Devices
}

impl Default for FSType {
    fn default() -> Self {
        Self::MeetiX
    }
}

impl fmt::Display for FSType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FatX => write!(f, "FatX"),
            Self::CdROM => write!(f, "CdROM"),
            Self::MeetiX => write!(f, "MeetiX"),
            Self::Proc => write!(f, "Proc"),
            Self::Devices => write!(f, "Devices")
        }
    }
}
