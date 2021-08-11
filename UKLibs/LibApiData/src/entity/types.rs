/*! `OsEntity` types */

use core::{
    convert::TryFrom,
    fmt
};

/**
 * Lists the available object types represented by a `OsEntity`
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum OsEntityType {
    /**
     * Default value
     */
    Unknown,

    /**
     * Identifies an `OsUser` entity
     */
    User,

    /**
     * Identifies an `OsGroup` entity
     */
    Group
}

impl Default for OsEntityType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Into<usize> for OsEntityType {
    fn into(self) -> usize {
        self as usize
    }
}

impl TryFrom<usize> for OsEntityType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::User),
            2 => Ok(Self::Group),
            _ => Err(())
        }
    }
}

impl fmt::Display for OsEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::User => write!(f, "User"),
            Self::Group => write!(f, "Group")
        }
    }
}
