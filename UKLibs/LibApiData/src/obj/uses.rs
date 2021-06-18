/*! `Object` usages */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the available usages of an `Object` based struct.
 *
 * The following variants can be used as bitwise parameters with
 * `Object::watch()` to tell the Kernel which obj events the caller
 * watch for.
 *
 * Otherwise are used as `Object::watch()`'s callback parameter to tell
 * the user which event is thrown
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum ObjUseBits {
    /**
     * Default value, never used
     */
    Unknown,

    /**
     * Let the watcher(s) task(s) notified about the successful
     * `ObjConfig::apply_for()` calls of the watched obj
     */
    Opening,

    /**
     * Let the watcher(s) task(s) notified about the successful data
     * read related operations of the watched obj, i.e:
     * `File::read()`, `IpcChan::recv()`, `MMap::get_ptr()` and so on
     */
    ReadingData,

    /**
     * Let the watcher(s) task(s) notified about the successful data
     * write related operations of the watched obj, i.e:
     * `File::write()`, `IpcChan::send()`, `MMap::get_ptr_mut()`
     * and so on
     */
    WritingData,

    /**
     * Let the watcher(s) task(s) notified about the successful
     * `Object::info()` (and related) calls of the watched obj
     */
    ReadingInfo,

    /**
     * Let the watcher(s) task(s) notified about the successful
     * `ObjInfo::update()` calls of the watched obj
     */
    WritingInfo,

    /**
     * Let the watcher(s) task(s) notified about the successful
     * `Object::send()` calls of the watched obj
     */
    Sending,

    /**
     * Let the watcher(s) task(s) notified about the successful
     * `Object::recv()` calls of the watched obj
     */
    Receiving,

    /**
     * Let the watcher(s) task(s) notified about the successful
     * `Object::watch()` calls of the watched obj
     */
    Watching,

    /**
     * Let the watcher(s) task(s) notified about the `Drop`
     * of the watched obj by the other users
     */
    Dropping,

    /**
     * Let the watcher(s) task(s) notified about the successful
     * `Object::drop_name()` calls of the watched obj.
     *
     * The watched obj remains alive until the last owner task
     * keeps it opened
     */
    Deleting
}

impl Default for ObjUseBits {
    fn default() -> Self {
        Self::Unknown
    }
}
