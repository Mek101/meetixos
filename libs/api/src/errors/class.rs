/*! # `Error` Classes
 *
 * Implements an enumeration that is responsible to provide a restricted
 * macro groups of errors that is possible to encounter using the system
 * calls
 */

use core::fmt;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/** # `Error` Classes
 *
 * List the known error classes that an [`Error`] instance is permitted to
 * fall
 *
 * [`Error`]: crate::errors::error::Error
 */
#[repr(u8)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum ErrorClass {
    /** Value used as placeholder for uninitialized [`Error`] value
     *
     * [`Error`]: crate::errors::error::Error
     */
    Unknown,

    /** At least one of the given arguments of the last system call doesn't
     * match the expected range
     */
    InvalidArgument,

    /** The last instance call was referencing an invalid object
     */
    InvalidHandleReference,

    /** The previous system call it was supposed to create an handle with a
     * specific id or a name have failed due to an already existing handle
     * with the same id/name
     */
    IdentifierNotAvailable,

    /** The previous system call was failed because the current
     * [`OSUser`]/[`OSGroup`]s have not enough grant to perform the
     * requested operation
     *
     * [`OSUser`]: crate::ents::impls::OSUser
     * [`OSGroup`]: crate::ents::impls::OSGroup
     */
    NotEnoughGrants,

    /** The previous system call was failed because the kernel have exhausted
     * the virtual/physical memory available
     */
    NotEnoughMemory,

    /** The previous system call was failed because the given [`Path`]
     * references an unexisting object name
     *
     * [`Path`]: crate::path::Path
     */
    ReferenceNotFound,

    /** The two [`ObjType`] types given doesn't match
     *
     * [`ObjType`]: crate::bits::obj::types::ObjType
     */
    TypesNotMatch,

    /** The previous system call was failed because the current [`Thread`]
     * have reached the limit of referencable resources a time
     *
     * [`Thread`]: crate::tasks::impls::Thread
     */
    LimitReached,

    /** The previous system call was failed because at least one of the
     * given parameters exceed the expected limit (i.e a [`Path`] or a name
     * to long)
     *
     * [`Path`]: crate::path::Path
     */
    LimitOverflow,

    /** The previous system call was failed because a poll requested data was
     * not still available (i.e a [`Object::recv()`] in [`RecvMode::Poll`])
     *
     * [`Object::recv()`]: crate::objs::Object::recv
     * [`RecvMode::Poll`]: crate::bits::obj::modes::RecvMode::Poll
     */
    NoDataAvailable,

    /** The previous system call was failed because was requested an
     * operation not enabled by the builder (i.e a [`File::read()`]
     * without a previous [`ObjConfig::for_read()`] call)
     *
     * [`File::read()`]: crate::objs::impls::File::read
     * [`ObjConfig::for_read()`]: crate::objs::ObjConfig::for_read
     */
    OperationNotEnabled,

    /** This is not properly an error, just indicates that the object have no
     * more data to read (i.e in [`File`] and [`Dir`])
     *
     * [`File`]: crate::objs::impls::File
     * [`Dir`]: crate::objs::impls::Dir
     */
    EndOfDataReached,

    /** The previous system call was failed because the running transaction
     * was interrupted by something else
     */
    InterruptedOperation
}

impl Default for ErrorClass {
    /** Returns the "default value" for a type
     */
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for ErrorClass {
    /** Formats the value using the given formatter
     */
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
