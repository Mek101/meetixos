/*! # Object Usages By A Tasks
 *
 * Implements the enum that list all the possible usages that a task can do
 * with an [`Object`] based struct
 *
 * [`Object`]: crate::objs::object::Object
 */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/** # `Object` Watchable Usages
 *
 * Lists the available usages that a task can perform with an [`Object`]
 * based struct.
 *
 * The following variants can be used as bitwise parameters with
 * [`Object::watch()`] to tell the kernel which object events the caller
 * watch for.
 *
 * Otherwise are used as [`Object::watch()`]'s callback parameter to tell
 * the user which event is thrown
 *
 * [`Object`]: crate::objs::object::Object
 * [`Object::watch()`]: crate::objs::object::Object::watch
 */
#[repr(u8)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum ObjUse {
    /** # Unknown use
     *
     * Usable only as uninitialized default value
     */
    Unknown     = 0,

    /** # Object opening
     *
     * Let the watcher(s) task(s) notified about the successful
     * [`ObjConfig::apply_for()`] calls of the watched object
     *
     * [`ObjConfig::apply_for()`]: crate::objs::config::ObjConfig::apply_for
     */
    Opening     = 1,

    /** # Object data reading
     *
     * Let the watcher(s) task(s) notified about the successful data
     * read related operations of the watched object, i.e:
     * [`File::read()`], [`IpcChan::recv()`], [`MMap::get_ptr()`] and so on
     *
     * [`File::read()`]: crate::objs::impls::file::File::read
     * [`IpcChan::recv()`]: crate::objs::impls::ipc_chan::IpcChan::recv
     * [`MMap::get_ptr()`]: crate::objs::impls::mmap::MMap::get_ptr
     */
    ReadingData = 2,

    /** # Object data writing
     *
     * Let the watcher(s) task(s) notified about the successful data
     * write related operations of the watched object, i.e:
     * [`File::write()`], [`IpcChan::send()`], [`MMap::get_ptr_mut()`]
     * and so on
     *
     * [`File::write()`]: crate::objs::impls::file::File::write
     * [`IpcChan::send()`]: crate::objs::impls::ipc_chan::IpcChan::send
     * [`MMap::get_ptr_mut()`]: crate::objs::impls::mmap::MMap::get_ptr_mut
     */
    WritingData = 4,

    /** # Object info reading
     *
     * Let the watcher(s) task(s) notified about the successful
     * [`Object::infos()`] (and related) calls of the watched object
     *
     * [`Object::infos()`]: crate::objs::object::Object::infos
     */
    ReadingInfo = 8,

    /** # Object info writing
     *
     * Let the watcher(s) task(s) notified about the successful
     * [`ObjInfo::update()`] calls of the watched object
     *
     * [`ObjInfo::update()`]: /api/objs/struct.ObjInfo.html#method.update
     */
    WritingInfo = 16,

    /** # Object sending
     *
     * Let the watcher(s) task(s) notified about the successful
     * [`Object::send()`] calls of the watched object
     *
     * [`Object::send()`]: crate::objs::object::Object::send
     */
    Sending     = 32,

    /** # Object receiving
     *
     * Let the watcher(s) task(s) notified about the successful
     * [`Object::recv()`] calls of the watched object
     *
     * [`Object::recv()`]: crate::objs::object::Object::recv
     */
    Receiving   = 64,

    /** # Object watching
     *
     * Let the watcher(s) task(s) notified about the successful
     * [`Object::watch()`] calls of the watched object
     *
     * [`Object::watch()`]: crate::objs::object::Object::watch
     */
    Watching    = 128,

    /** # Object dropping
     *
     * Let the watcher(s) task(s) notified about the [`Drop`]
     * of the watched object by the other users
     *
     * [`Drop`]: core::ops::Drop
     */
    Dropping    = 256,

    /** # Object deleting
     *
     * Let the watcher(s) task(s) notified about the successful
     * [`Object::drop_name()`] calls of the watched object.
     *
     * The watched object remains alive until the last owner task
     * keeps it opened
     *
     * [`Object::drop_name()`]: crate::objs::object::Object::drop_name
     */
    Deleting    = 512
}
