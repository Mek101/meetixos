/*! `Object` modes bits */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the available modes for `Object::recv()`
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum ObjRecvMode {
    /**
     * Simply asks to the Kernel whether an obj of the requested type is
     * already available into the obj receiving queue, if not it returns
     * an `Err(NoDataAvailable)` error
     */
    Poll,

    /**
     * Puts the task in a waiting state until an obj of the
     * requested type is available into the task's receiving queue.
     *
     * If already available the system call immediately returns and the task
     * will not fall into waiting state.
     *
     * If the Kernel's wait queue is full the system call returns an
     * `Err(LimitReached)` error
     */
    Sync
}

/**
 * Lists the internally used modes to identify the direction of the
 * `KrnIterator` in use
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KrnIterDirection {
    /**
     * Internally used when called `KrnIterator::find_next()`
     */
    BeginToEnd,

    /**
     * Internally used when called `KrnIterator::find_next_back()`
     */
    EndToBegin
}

/**
 * Lists the internally used modes that are given to the Kernel to manage
 * synchronization over the memory of a `MMap`
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum MMapPtrMode {
    /**
     * Internally used when called `MMap::get_ptr()`
     */
    ForRead,

    /**
     * Internally used when called `MMap::get_ptr_mut()`
     */
    ForWrite
}

/**
 * Lists the available modes for `File::set_pos()` and `Dir::set_index()`
 */
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum SeekMode {
    /**
     * The given offset will be interpreted as an absolute offset
     */
    Absolute(u64),

    /**
     * The given offset will be added to the current offset (position
     * relative)
     */
    Relative(i64),

    /**
     * The cursor of the obj will be moved to the end
     */
    End
}

impl SeekMode {
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
            Self::Absolute(off) => Some(off as usize),
            Self::Relative(off) => Some(off as usize),
            Self::End => None
        }
    }
}
