/*! # Object Modes Bits
 *
 * Implements various enumerations that are used for certain [`Object`]
 * related calls
 *
 * [`Object`]: crate::objs::object::Object
 */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/** # `Object::recv()` Waiting Modes
 *
 * Lists the available modes for [`Object::recv()`]
 *
 * [`Object::recv()`]: crate::objs::object::Object::recv
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum RecvMode {
    /** This mode simply asks to the kernel whether an object of the
     * requested type is already available into the object receiving
     * queue, if not it returns an [`Err(NoDataAvailable)`](E) error
     *
     * [E]: crate::errors::ErrorClass::NoDataAvailable
     */
    Poll,

    /** This mode puts the task in a waiting state until an object of the
     * requested type is available into the task's receiving queue.
     *
     * If already available the system call immediately returns and the task
     * will not fall into waiting state.
     *
     * If the kernel's wait queue is full the system call returns an
     * [`Err(LimitReached)`](E) error
     *
     * [E]: crate::errors::ErrorClass::LimitReached
     */
    Sync
}

/** # `KrnIterator::find_next()` Modes
 *
 * Lists the internally used modes to identify the direction of the
 * [`KrnIterator`] in use
 *
 * [`KrnIterator`]: crate::objs::impls::iter::KrnIterator
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum KrnIterDirection {
    /** Internally used when called [`KrnIterator::find_next()`](L)
     *
     * [L]: crate::objs::impls::iter::KrnIterator::find_next
     */
    BeginToEnd,

    /** Internally used when called [`KrnIterator::find_next_back()`](L)
     *
     * [L]: crate::objs::impls::iter::KrnIterator::find_next_back
     */
    EndToBegin
}

/** # `MMap::get_ptr` Modes
 *
 * Lists the internally used modes that are given to the kernel to manage
 * synchronization over the memory of a [`MMap`]
 *
 * [`MMap`]: crate::objs::impls::mmap::MMap
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum MMapPtrMode {
    /** Internally used when called [`MMap::get_ptr()`]
     *
     * [`MMap::get_ptr()`]: crate::objs::impls::mmap::MMap::get_ptr
     */
    Readable,

    /** Internally used when called [`MMap::get_ptr_mut()`]
     *
     * [`MMap::get_ptr_mut()`]: crate::objs::impls::mmap::MMap::get_ptr_mut
     */
    Writeable
}

/** # `File::set_pos` & `Dir::set_pos` Modes
 *
 * Lists the available modes for [`File::set_pos()`] and
 * [`Dir::set_index()`]
 *
 * [`File::set_pos()`]: crate::objs::impls::file::File::set_pos
 * [`Dir::set_pos()`]: crate::objs::impls::dir::Dir::set_index
 */
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum SeekMode {
    /** The given offset will be interpreted as an absolute offset
     */
    Absolute(u64),

    /** The given offset will be added to the current offset
     * (position relative)
     */
    Relative(i64),

    /** The cursor of the object will be moved to the end
     */
    End
}

impl SeekMode {
    /* Returns the integer which identifies the mode
     */
    pub fn mode(&self) -> usize {
        match self {
            SeekMode::Absolute(_) => 0,
            SeekMode::Relative(_) => 1,
            SeekMode::End => 2
        }
    }

    /** Returns [`Some(offset)`] if the variants have any
     *
     * [`Some(offset)`]: core::option::Option::Some
     */
    pub fn off(&self) -> Option<usize> {
        match *self {
            SeekMode::Absolute(off) => Some(off as usize),
            SeekMode::Relative(off) => Some(off as usize),
            SeekMode::End => None
        }
    }
}
