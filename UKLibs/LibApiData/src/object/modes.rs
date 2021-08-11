/*! `Object` modes bits */

use core::convert::TryFrom;

use crate::sys::TAsSysCallPtr;

/**
 * Lists the available modes for `Object::recv()`
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum ObjRecvMode {
    /**
     * Simply asks to the Kernel whether an `Object` of the requested
     * type is already available into the `Task`'s receiving queue.
     *
     * If any `Object` is available the system call returns an error0
     */
    Poll,

    /**
     * Puts the `Task` in a waiting state until an `Object` of the requested
     * type is available into the `Task`'s receiving queue.
     *
     * If already available the system call immediately returns and the task
     * will not fall into waiting state.
     */
    Sync
}

impl Into<usize> for ObjRecvMode {
    fn into(self) -> usize {
        self as usize
    }
}

impl TryFrom<usize> for ObjRecvMode {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Poll),
            1 => Ok(Self::Sync),
            _ => Err(())
        }
    }
}

/**
 * Lists the internally used modes that are given to the Kernel to manage
 * synchronization over the memory of a `MMap`
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum MMapPtrMode {
    /**
     * Internally used when called `MMap::ptr()`
     */
    ForRead,

    /**
     * Internally used when called `MMap::ptr_mut()`
     */
    ForWrite
}

impl Into<usize> for MMapPtrMode {
    fn into(self) -> usize {
        self as usize
    }
}

impl TryFrom<usize> for MMapPtrMode {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ForRead),
            1 => Ok(Self::ForWrite),
            _ => Err(())
        }
    }
}

/**
 * Lists the available modes for `[Device/File/Dir]::[set_]pos()`
 */
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum SeekMode {
    /**
     * The given offset will be interpreted as an absolute offset
     */
    Absolute(usize),

    /**
     * The given offset will be added to the current offset (position
     * relative)
     */
    Relative(isize),

    /**
     * The cursor of the object will be moved to the end
     */
    End
}

impl SeekMode /* Methods */ {
    /**
     * Returns the integer which identifies the mode
     */
    pub fn mode(&self) -> usize {
        match self {
            Self::Absolute(_) => 0,
            Self::Relative(_) => 1,
            Self::End => 2
        }
    }

    /**
     * Returns `Some(offset)` if the current variant have any
     */
    pub fn offset(&self) -> Option<usize> {
        match *self {
            Self::Absolute(off) => Some(off),
            Self::Relative(off) => Some(off as usize),
            Self::End => None
        }
    }
}

impl TAsSysCallPtr for SeekMode {
    /* No methods to implement */
}
