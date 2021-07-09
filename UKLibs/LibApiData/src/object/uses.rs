/*! `Object` usages */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use bits::bit_flags::BitFlagsValues;

/**
 * Lists the available usages of an `Object` based struct.
 *
 * The following variants can be used as bitwise parameters with
 * `Object::watch()` to tell the Kernel which `Object` events the caller
 * watch for.
 *
 * Otherwise are used as `Object::watch()`'s callback parameter to tell
 * the user which event is thrown
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum ObjUseBits {
    /**
     * Default value, never used
     */
    Unknown,

    /**
     * Let the watcher `Task` notified about the successful
     * `ObjConfig::apply_for()` calls for the watched `Object`
     */
    Opening,

    /**
     * Let the watcher `Task` notified about the successful data
     * read related operations of the watched `Object`, i.e:
     * `File::read()`, `IpcChan::recv()`, `MMap::get_ptr()` and so on
     */
    ReadingData,

    /**
     * Let the watcher `Task` notified about the successful data
     * write related operations of the watched `Object`, i.e:
     * `File::write()`, `IpcChan::send()`, `MMap::get_ptr_mut()`
     * and so on
     */
    WritingData,

    /**
     * Let the watcher `Task` notified about the successful `Object::boot()`
     * (and related) calls of the watched `Object`
     */
    ReadingInfo,

    /**
     * Let the watcher `Task` notified about the successful
     * `ObjInfo::update()` calls of the watched `Object`
     */
    WritingInfo,

    /**
     * Let the watcher `Task` notified about the successful `Object::send()`
     * calls of the watched `Object`
     */
    Sending,

    /**
     * Let the watcher `Task` notified about the successful `Object::recv()`
     * calls of the watched `Object`
     */
    Receiving,

    /**
     * Let the watcher `Task` notified about the successful
     * `Object::watch()` calls of the watched `Object`
     */
    Watching,

    /**
     * Let the watcher `Task` notified about the `Drop` of the watched
     * `Object` by the other users
     */
    Dropping,

    /**
     * Let the watcher `Task` notified about the successful
     * `Object::drop_name()` calls of the watched `Object`.
     *
     * The watched `Object` remains alive until the last owner task
     * keeps it opened
     */
    Deleting
}

impl Default for ObjUseBits {
    fn default() -> Self {
        Self::Unknown
    }
}

impl BitFlagsValues for ObjUseBits {
}
