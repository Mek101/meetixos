/*! # Object Usages By A Tasks
 *
 * Implements the enum that list all the possible usages that a task can do
 * with an [`Object`] based struct
 *
 * [`Object`]: /api/objs/trait.Object.html
 */

c_handy_enum! {
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
     * [`Object`]: /api/objs/trait.Object.html
     * [`Object::watch()`]: /api/objs/trait.Object.html#method.watch
     */
    pub enum ObjUse : u16 {
        /** # Unknown use
         *
         * Usable only as uninitialized default value
         */
        Unknown = 0,

        /** # Object opening
         *
         * Let the watcher(s) task(s) notified about the successful
         * [`ObjConfig::apply_for()`] calls of the watched object
         *
         * [`ObjConfig::apply_for()`]: /api/objs/struct.ObjConfig.html#method.apply_for
         */
        Opening = 1,

        /** # Object data reading
         *
         * Let the watcher(s) task(s) notified about the successful data
         * read related operations of the watched object, i.e:
         * [`File::read()`], [`IpcChan::recv()`], [`MMap::get_ptr()`] and so on
         *
         * [`File::read()`]: /api/objs/impls/struct.File.html#method.read
         * [`IpcChan::recv()`]: /api/objs/impls/struct.IpcChan.html#method.recv
         * [`MMap::get_ptr()`]: /api/objs/impls/struct.MMap.html#method.get_ptr
         */
        ReadingData = 2,

        /** # Object data writing
         *
         * Let the watcher(s) task(s) notified about the successful data
         * write related operations of the watched object, i.e:
         * [`File::write()`], [`IpcChan::send()`], [`MMap::get_ptr_mut()`]
         * and so on
         *
         * [`File::write()`]: /api/objs/impls/struct.File.html#method.write
         * [`IpcChan::send()`]: /api/objs/impls/struct.IpcChan.html#method.send
         * [`MMap::get_ptr_mut()`]: /api/objs/impls/struct.MMap.html#method.get_ptr_mut
         */
        WritingData = 4,

        /** # Object info reading
         *
         * Let the watcher(s) task(s) notified about the successful
         * [`Object::infos()`] (and related) calls of the watched object
         *
         * [`Object::infos()`]: /api/objs/trait.Object.html#method.infos
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
         * [`Object::send()`]: /api/objs/trait.Object.html#method.send
         */
        Sending = 32,

        /** # Object receiving
         *
         * Let the watcher(s) task(s) notified about the successful
         * [`Object::recv()`] calls of the watched object
         *
         * [`Object::recv()`]: /api/objs/trait.Object.html#method.recv
         */
        Receiving = 64,

        /** # Object watching
         *
         * Let the watcher(s) task(s) notified about the successful
         * [`Object::watch()`] calls of the watched object
         *
         * [`Object::watch()`]: /api/objs/trait.Object.html#method.watch
         */
        Watching = 128,

        /** # Object dropping
         *
         * Let the watcher(s) task(s) notified about the [`Drop`]
         * of the watched object by the other users
         *
         * [`Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html
         */
        Dropping = 256,

        /** # Object deleting
         *
         * Let the watcher(s) task(s) notified about the successful
         * [`Object::drop_name()`] calls of the watched object.
         *
         * The watched object remains alive until the last owner task
         * keeps it opened
         *
         * [`Object::drop_name()`]: /api/objs/trait.Object.html#method.drop_name
         */
        Deleting = 512,
    }
}
