/*! `Object` usages */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the available usages of an `Object` based struct.
 *
 * The following variants can be used as bitwise parameters with
 * `Object::watch()` to tell the Kernel which object events the caller
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
pub enum ObjUse {
    /**
     * Default value, never used
     */
    Unknown     = 0,

    /**
     * Let the watcher(s) task(s) notified about the successful
     * `ObjConfig::apply_for()` calls of the watched object
     */
    Opening     = 1,

    /**
     * Let the watcher(s) task(s) notified about the successful data
     * read related operations of the watched object, i.e:
     * `File::read()`, `IpcChan::recv()`, `MMap::get_ptr()` and so on
     */
    ReadingData = 2,

    /**
     * Let the watcher(s) task(s) notified about the successful data
     * write related operations of the watched object, i.e:
     * `File::write()`, `IpcChan::send()`, `MMap::get_ptr_mut()`
     * and so on
     */
    WritingData = 4,

    /**
     * Let the watcher(s) task(s) notified about the successful
     * `Object::info()` (and related) calls of the watched object
     */
    ReadingInfo = 8,

    /**
     * Let the watcher(s) task(s) notified about the successful
     * `ObjInfo::update()` calls of the watched object
     */
    WritingInfo = 16,

    /**
     * Let the watcher(s) task(s) notified about the successful
     * `Object::send()` calls of the watched object
     */
    Sending     = 32,

    /**
     * Let the watcher(s) task(s) notified about the successful
     * `Object::recv()` calls of the watched object
     */
    Receiving   = 64,

    /**
     * Let the watcher(s) task(s) notified about the successful
     * `Object::watch()` calls of the watched object
     */
    Watching    = 128,

    /**
     * Let the watcher(s) task(s) notified about the `Drop`
     * of the watched object by the other users
     */
    Dropping    = 256,

    /**
     * Let the watcher(s) task(s) notified about the successful
     * `Object::drop_name()` calls of the watched object.
     *
     * The watched object remains alive until the last owner task
     * keeps it opened
     */
    Deleting    = 512
}

impl Default for ObjUse {
    fn default() -> Self {
        Self::Unknown
    }
}
