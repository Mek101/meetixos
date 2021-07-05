/*! `OsEntity` types */

use core::fmt;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the available object types represented by a `OsEntity`
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
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

impl fmt::Display for OsEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::User => write!(f, "User"),
            Self::Group => write!(f, "Group")
        }
    }
}
