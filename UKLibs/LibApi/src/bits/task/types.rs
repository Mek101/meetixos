/*! `Task` types */

use core::fmt;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the available object types represented by an `TaskId`
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
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

impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Thread => write!(f, "Thread"),
            Self::Proc => write!(f, "Proc")
        }
    }
}
