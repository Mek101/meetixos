/*! `OsError` classes */

use core::fmt;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * List the well-known error classes which an `OsError` can represent
 */
#[repr(u8)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum OsErrorClass {
    /**
     * Default value, used for uninitialized `OsError`s
     */
    Unknown,

    /**
     * At least one of the given arguments of the last system call doesn't
     * match the expected range
     */
    InvalidArgument,

    /**
     * The last instance call was referencing an invalid obj
     */
    InvalidHandleReference,

    /**
     * The previous system call it was supposed to create an handle with a
     * specific id or a name have failed due to an already existing handle
     * with the same id/name
     */
    IdentifierNotAvailable,

    /**
     * The previous system call was failed because the current
     * `OSUser`/`OSGroup`s have not enough grant to perform the
     * requested operation
     */
    NotEnoughGrants,

    /**
     * The previous system call was failed because the Kernel have exhausted
     * the virtual/physical memory available
     */
    NotEnoughMemory,

    /**
     * The previous system call was failed because the given `Path`
     * references an unexisting obj name
     */
    ReferenceNotFound,

    /**
     * The two `ObjType` types given doesn't match
     */
    TypesNotMatch,

    /**
     * The previous system call was failed because the current `Thread`
     * have reached the limit of referencable resources a time
     */
    LimitReached,

    /**
     * The previous system call was failed because at least one of the
     * given parameters exceed the expected limit (i.e a `Path` or a name
     * to long)
     */
    LimitOverflow,

    /**
     * The previous system call was failed because a poll requested data was
     * not still available (i.e a `Object::recv()` in `RecvMode::Poll`)
     */
    NoDataAvailable,

    /**
     * The previous system call was failed because was requested an
     * operation not enabled by the builder (i.e a `File::read()`
     * without a previous `ObjConfig::for_read()` call)
     */
    OperationNotEnabled,

    /**
     * This is not properly an error, just indicates that the obj have no
     * more data to read (i.e in `File` and `Dir`)
     */
    EndOfDataReached,

    /**
     * The previous system call was failed because the running transaction
     * was interrupted by something else
     */
    InterruptedOperation
}

impl Default for OsErrorClass {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for OsErrorClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::InvalidArgument => write!(f, "Invalid argument"),
            Self::InvalidHandleReference => write!(f, "Invalid handle reference"),
            Self::IdentifierNotAvailable => write!(f, "Identifier not available"),
            Self::NotEnoughGrants => write!(f, "Not enough grants"),
            Self::NotEnoughMemory => write!(f, "Not enough memory"),
            Self::ReferenceNotFound => write!(f, "Reference not found"),
            Self::TypesNotMatch => write!(f, "Types not match"),
            Self::LimitReached => write!(f, "Limit reached"),
            Self::LimitOverflow => write!(f, "Limit overflow"),
            Self::NoDataAvailable => write!(f, "Data not available"),
            Self::OperationNotEnabled => write!(f, "Operation not enabled"),
            Self::EndOfDataReached => write!(f, "End of data reached"),
            Self::InterruptedOperation => write!(f, "Interrupted operation")
        }
    }
}
