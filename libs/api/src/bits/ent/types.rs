/*! `OSEntity` types */

use core::fmt;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the available object types represented by an `OSEntityId`
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum OSEntityType {
    /**
     * Default value
     */
    Unknown,

    /**
     * Identifies an `OSUser` entity
     */
    User,

    /**
     * Identifies an `OSGroup` entity
     */
    Group
}

impl Default for OSEntityType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for OSEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::User => write!(f, "User"),
            Self::Group => write!(f, "Group")
        }
    }
}
