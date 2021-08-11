/*! `Task` types */

use core::{
    convert::TryFrom,
    fmt
};

/**
 * Lists the available object types represented by a `Task`
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum TaskType {
    /**
     * Default value
     */
    Unknown,

    /**
     * Identifies a `Thread` task
     */
    Thread,

    /**
     * Identifies a `Proc` task
     */
    Proc
}

impl Default for TaskType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Into<usize> for TaskType {
    fn into(self) -> usize {
        self as usize
    }
}

impl TryFrom<usize> for TaskType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Thread),
            2 => Ok(Self::Proc),
            _ => Err(())
        }
    }
}

impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Thread => write!(f, "Thread"),
            Self::Proc => write!(f, "Proc")
        }
    }
}
