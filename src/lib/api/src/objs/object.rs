/*! # `Object` Handle
 *
 * Implements the base struct and the trait used as base for the kernel's
 * managed objects
 */

use core::mem;

use os::sysc::{codes::KernObjectFnId, fn_path::KernFnPath};

use crate::{
    bits::{
        obj::{ObjType, ObjUse, RecvMode},
        task::{RWatchCBThreadEntry, ThreadEntryData}
    },
    caller::{KernCaller, Result},
    config::{CreatMode, FindMode},
    objs::{impls::Any, infos::ObjInfo, ObjConfig},
    tasks::Task,
    time::Instant
};

/** # Object Handle
 *
 * Represents an opaque handle that takes place of the old style file
 * descriptor integer, used by all the Unix-like OS to keep reference to an
 * open resource.
 *
 * Itself the object doesn't have much utilities because most of his methods
 * are private, but exposed via the [`Object`] trait and implemented by the
 * various [implementations].
 *
 * Read more doc about [`Object`] and [`ObjId`] -> [here]
 *
 * [`Object`]: /api/objs/trait.Object.html
 * [implementations]: /api/objs/impls/index.html
 * [`ObjId`]: /api/objs/struct.ObjId.html
 * [here]: /api/index.html#objects
 */
#[repr(transparent)]
#[derive(Debug, Default, Eq, PartialEq)]
pub struct ObjId(u32);

impl ObjId {
    /** # Constructs an un initialized `ObjId`
     *
     * Used only by the [`OsRawMutex`] to satisfy the constant
     * initialization
     *
     * [`OsRawMutex`]: /api/objs/impls/struct.OsRawMutex.html
     */
    pub(crate) const fn const_new() -> Self {
        Self(0)
    }

    /** # Share this `ObjId` with another `Task`
     *
     * Sends this object instance to another [`Task`] (a [`Thread`] or a
     * [`Process`]) to share the same resource.
     *
     * The concurrency is managed internally by the kernel with two
     * `RWLock`s (one for the data and one for the informations), so
     * multiple tasks can read the data or the infos but only one a time
     * can write them
     *
     * [`Task`]: /api/tasks/trait.Task.html
     * [`Thread`]: /api/tasks/impls/struct.Thread.html
     * [`Process`]: /api/tasks/impls/struct.Proc.html
     */
    fn send<T>(&self, receiver: &T) -> Result<()>
        where T: Task {
        self.kern_call_1(KernFnPath::Object(KernObjectFnId::Send),
                         receiver.task_handle().id_usize())
            .map(|_| ())
    }

    /** # Accepts an incoming `ObjId`
     *
     * The previous handle is first released with [`Drop`] then overwritten
     * with the new handle received according to the [`RecvMode`] given
     *
     * [`Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html
     * [`RecvMode`]: /api/bits/obj/enum.RecvMode.html
     */
    pub(crate) fn recv(&mut self, obj_type: ObjType, mode: RecvMode) -> Result<()> {
        self.kern_call_2(KernFnPath::Object(KernObjectFnId::Recv),
                         obj_type.into(),
                         mode.into())
            .map(|obj_id| {
                *self = Self::from(obj_id);
                ()
            })
    }

    /** # Updates the infos of this object
     *
     * Internally used by [`ObjInfo::update()`]
     *
     * [`ObjInfo::update()`]:
     * /api/objs/infos/struct.ObjInfo.html#method.update
     */
    pub(crate) fn update_infos<T>(&self, infos: &ObjInfo<T>) -> Result<()>
        where T: Object {
        self.kern_call_1(KernFnPath::Object(KernObjectFnId::UpdateInfo),
                         infos as *const _ as usize)
            .map(|_| ())
    }

    /** # Drops the object name
     *
     * Makes the object no longer reachable via the VFS.
     *
     * When all the tasks, that already references it, drop it will be
     * definitively destroyed by the kernel
     */
    fn drop_name(&self) -> Result<()> {
        self.kern_call_0(KernFnPath::Object(KernObjectFnId::DropName)).map(|_| ())
    }

    /** # Enable `Object` watching
     *
     * Registers the given `callback` to be executed whenever one of the
     * bitwise given [`ObjUse`] happen.
     *
     * The caller must have [information read grants] to successfully call
     * this method.
     *
     * The given `callback` must accept an [`ObjUseInstant`] as argument and
     * must return a boolean that tells to the kernel whether the callback
     * must be re-called for the next event given via `filter` or must
     * be unregistered.
     *
     * Multiple `callback`s can be registered for different uses, but if the
     * given filters overlaps a previously registered callback an error will
     * be returned
     *
     * [`ObjUse`]: /api/objs/infos/struct.ObjUse.html
     * [information read grants]:
     * /api/bits/obj/struct.Grants.html#method.set_info_readable
     * [`ObjUseInstant`]: /api/objs/infos/struct/ObjUseInstant.html
     */
    fn watch(&self, filter: ObjUse, callback_fn: RWatchCBThreadEntry) -> Result<()> {
        let thread_entry_data = ThreadEntryData::new_watch_callback(callback_fn);
        self.kern_call_2(KernFnPath::Object(KernObjectFnId::Watch),
                         filter.into(),
                         &thread_entry_data as *const _ as usize)
            .map(|_| ())
    }

    /** Returns the [`ObjInfo`] of this object
     *
     * [`ObjInfo`]: /api/objs/infos/struct.ObjInfo.html
     */
    pub(crate) fn infos<T>(&self) -> Result<ObjInfo<T>>
        where T: Object {
        let mut infos = ObjInfo::default();
        self.kern_call_1(KernFnPath::Object(KernObjectFnId::Info),
                         &mut infos as *mut _ as usize)
            .map(|_| {
                infos.set_obj(self);
                infos
            })
    }

    /** Returns whether this object instance references a still valid kernel
     * object
     */
    pub fn is_valid(&self) -> bool {
        self.0 != 0
        && self.kern_call_0(KernFnPath::Object(KernObjectFnId::IsValid))
               .map(|_| true)
               .unwrap_or(false)
    }

    /** Returns the raw identifier of this [`ObjId`]
     *
     * [`ObjId`]: /api/objs/struct.ObjId.html
     */
    pub fn as_raw(&self) -> u32 {
        self.0
    }

    /** Returns the raw identifier of this [`ObjId`] as `usize`
     *
     * [`ObjId`]: /api/objs/struct.ObjId.html
     */
    pub fn as_raw_usize(&self) -> usize {
        self.as_raw() as usize
    }
}

impl Clone for ObjId {
    /** Increases the references count to the object referenced.
     *
     * The returned [`ObjId`] is a new instance but reference the same
     * kernel's object, so changes on any of the cloned instances affect the
     * same kernel's object
     *
     * [`ObjId`]: /api/objs/struct.ObjId.html
     */
    fn clone(&self) -> Self {
        self.kern_call_0(KernFnPath::Object(KernObjectFnId::AddRef))
            .map(|_| Self::from(self.0))
            .unwrap()
    }
}

impl Drop for ObjId {
    /** Decreases by one the references count to the referenced kernel's
     * object.
     *
     * The life of the objects varies by type:
     *
     * Permanent objects, like [`File`]s, [`Dir`]ectories, [`Link`]s and
     * [`OsRawMutex`]es, persists until they are explicitly destroyed with
     * [`Object::drop_name()`].
     *
     * The other kind of objects, like [`MMap`]s and [`IpcChan`]nels, live
     * until there is a reference to them. When the references reaches the 0
     * they are definitely destroyed
     *
     * [`File`]: /api/objs/impls/struct.File.html
     * [`Dir`]: /api/objs/impls/struct.Dir.html
     * [`Link`]: /api/objs/impls/struct.Link.html
     * [`OsRawMutex`]: /api/objs/impls/struct.OsRawMutex.html
     * [`Object::drop_name()`]: /api/objs/trait.Object.html#method.drop_name
     * [`MMap`]: /api/objs/impls/struct.MMap.html
     * [`IpcChan`]: /api/objs/impls/struct.IpcChan.html
     */
    fn drop(&mut self) {
        if self.is_valid() {
            self.kern_call_0(KernFnPath::Object(KernObjectFnId::Drop)).unwrap();
        }
    }
}

impl From<u32> for ObjId {
    /** Performs the conversion
     */
    fn from(raw_id: u32) -> Self {
        Self(raw_id)
    }
}

impl From<usize> for ObjId {
    /** Performs the conversion
     */
    fn from(raw_id: usize) -> Self {
        Self::from(raw_id as u32)
    }
}

impl KernCaller for ObjId {
    /** Returns the raw identifier of the object
     */
    fn caller_handle_bits(&self) -> u32 {
        self.as_raw()
    }
}

/** # `Object` Base Interface
 *
 * Defines a common interface implemented by all the [`ObjId`] based
 * objects.
 *
 * It mainly exposes the private methods of the [`ObjId`] for safe calling
 * and provides convenient methods to easily perform works that normally
 * implies more than one call.
 *
 * [`ObjId`]: /api/objs/struct.ObjId.html
 */
pub trait Object: From<ObjId> + Default + Clone {
    /** The value of the [`ObjType`] that matches the implementation
     *
     * [`ObjType`]: /api/bits/obj/enum.ObjType.html
     */
    const TYPE: ObjType;

    /** Returns the immutable reference to the underling [`ObjId`] instance
     *
     * [`ObjId`]: /api/objs/struct.ObjId.html
     */
    fn obj_handle(&self) -> &ObjId;

    /** Returns the mutable reference to the underling [`ObjId`] instance
     *
     * [`ObjId`]: /api/objs/struct.ObjId.html
     */
    fn obj_handle_mut(&mut self) -> &mut ObjId;

    /** Returns an uninitialized [`ObjConfig`] to open an existing [`Object`]
     *
     * [`ObjConfig`]: /api/objs/struct.ObjConfig.html
     * [`Object`]: /api/objs/trait.Object.html
     */
    fn open() -> ObjConfig<Self, FindMode> {
        ObjConfig::<Self, FindMode>::new()
    }

    /** # Obtain the underling `ObjId`
     *
     * Consumes the object into his [`ObjId`] instance
     *
     * [`ObjId`]: /api/objs/struct.ObjId.html
     */
    fn into_id(self) -> ObjId {
        let raw_id = self.obj_handle().as_raw();
        mem::forget(self);
        ObjId::from(raw_id)
    }

    /** # Upcast to `Any`
     *
     * Consumes the object upcasting it to an [`Any`] instance
     *
     * [`Any`]: /api/objs/impls/struct.Any.html
     */
    fn into_any(self) -> Any {
        Any::from(self.into_id())
    }

    /** # Drops the object name
     *
     * Makes the object no longer reachable via the VFS.
     *
     * When all the tasks, that already references it, drop it will be
     * definitively destroyed by the kernel
     */
    fn drop_name(&self) -> Result<()> {
        self.obj_handle().drop_name()
    }

    /** # Enable `Object` watching
     *
     * Registers the given `callback` to be executed whenever one of the
     * bitwise given [`ObjUse`] happen.
     *
     * The caller must have [information read grants] to successfully call
     * this method.
     *
     * The given `callback` must accept an [`ObjUseInstant`] as argument and
     * must return a boolean that tells to the kernel whether the callback
     * must be re-called for the next event given via `filter` or must
     * be unregistered.
     *
     * Multiple `callback`s can be registered for different uses, but if the
     * given filters overlaps a previously registered callback an error will
     * be returned
     *
     * [`ObjUse`]: /api/objs/infos/struct.ObjUse.html
     * [information read grants]:
     * /api/bits/obj/struct.Grants.html#method.set_info_readable
     * [`ObjUseInstant`]: /api/objs/infos/struct/ObjUseInstant.html
     */
    fn watch(&self, filter: ObjUse, callback_fn: RWatchCBThreadEntry) -> Result<()> {
        self.obj_handle().watch(filter, callback_fn)
    }

    /** # Share this `Object` with another `Task`
     *
     * Sends this object instance to another [`Task`] (a [`Thread`] or a
     * [`Process`]) to share the same resource.
     *
     * The concurrency is managed internally by the kernel with two
     * `RWLock`s (one for the data and one for the informations), so
     * multiple tasks can read the data or the infos but only one a time
     * can write them
     *
     * [`Task`]: /api/tasks/trait.Task.html
     * [`Thread`]: /api/tasks/impls/struct.Thread.html
     * [`Process`]: /api/tasks/impls/struct.Proc.html
     */
    fn send<T>(&self, task: &T) -> Result<()>
        where T: Task {
        self.obj_handle().send(task)
    }

    /** # Accepts an incoming `Object`
     *
     * The previous handle is first released with [`Drop`] then overwritten
     * with the new handle received according to the [`RecvMode`] given
     *
     * [`Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html
     * [`RecvMode`]: /api/bits/obj/enum.RecvMode.html
     */
    fn recv(&mut self, mode: RecvMode) -> Result<()> {
        self.obj_handle_mut().recv(Self::TYPE, mode)
    }

    /** # Constructs a new `Object` from the incoming one
     *
     * Convenience method that internally creates an uninitialized object
     * instance then performs an [`Object::recv()`] using the given
     * [`RecvMode`]
     *
     * [`Object::recv()`]: /api/objs/trait.Object.html#method.recv
     * [`RecvMode`]: /api/bits/obj/enum.RecvMode.html
     */
    fn recv_new(mode: RecvMode) -> Result<Self> {
        let mut obj = Self::default();
        obj.recv(mode).map(|_| obj)
    }

    /** Returns the [`ObjInfo`] of this object
     *
     * [`ObjInfo`]: /api/objs/infos/struct.ObjInfo.html
     */
    fn infos(&self) -> Result<ObjInfo<Self>> {
        self.obj_handle().infos()
    }

    /** Returns the [`ObjType`] of the object
     *
     * [`ObjType`]: /api/bits/obj/enum.ObjType.html
     */
    fn obj_type(&self) -> ObjType {
        self.infos().unwrap_or_default().obj_type()
    }

    /** Returns all the `Instant` timestamps ordered as
     *
     * 0. Creation [`Instant`]
     * 1. Last access [`Instant`]
     * 2. Last data modify [`Instant`]
     * 3. Last info modify [`Instant`]
     *
     * [`Instant`]: /api/time/struct.Instant.html
     */
    fn timestamps(&self) -> (Instant, Instant, Instant, Instant) {
        self.infos().unwrap_or_default().timestamps()
    }

    /** Returns the [size] of the object
     *
     * [size]: /api/objs/infos/struct.ObjInfo.html#method.size
     */
    fn size(&self) -> usize {
        self.infos().unwrap_or_default().size()
    }
}

/** # User Creatable
 *
 * Interface implemented for all the user creatable objects
 */
pub trait UserCreatable: Object {
    /** Returns an uninitialized [`ObjConfig`] to create a new [`Object`]
     *
     * [`ObjConfig`]: /api/objs/struct.ObjConfig.html
     * [`Object`]: /api/objs/trait.Object.html
     */
    fn creat() -> ObjConfig<Self, CreatMode> {
        ObjConfig::<Self, CreatMode>::new()
    }
}

macro_rules! impl_obj_id_object {
    {
        $(#[$Comments:meta])*
        pub struct $ObjTypeName:ident $( : impl $($CustomMarker:ident),* )? {
            where TYPE = $ObjType:path;
        }
    } => {
        $(#[$Comments])*
        #[repr(transparent)]
        #[derive(Debug, Default, Clone, Eq, PartialEq)]
        pub struct $ObjTypeName(ObjId);

        impl Object for $ObjTypeName {
            /** The value of the [`ObjType`] that matches the implementation
             *
             * [`ObjType`]: /api/bits/ent/enum.ObjType.html
             */
            const TYPE: ObjType = $ObjType;

            /** Returns the immutable reference to the underling [`ObjId`] instance
             *
             * [`ObjId`]: /api/objs/struct.ObjId.html
             */
            fn obj_handle(&self) -> &ObjId {
                &self.0
            }

            /** Returns the mutable reference to the underling [`ObjId`] instance
             *
             * [`ObjId`]: /api/objs/struct.ObjId.html
             */
            fn obj_handle_mut(&mut self) -> &mut ObjId {
                &mut self.0
            }
        }

        impl From<ObjId> for $ObjTypeName {
            /** Performs the conversion.
             */
            fn from(id: ObjId) -> Self {
                Self(id)
            }
        }

        impl KernCaller for $ObjTypeName {
            /** Returns the raw identifier of the object
             */
            fn caller_handle_bits(&self) -> u32 {
                self.obj_handle().caller_handle_bits()
            }
        }

        $($(
            impl $CustomMarker for $ObjTypeName {
                /* no methods to implement, just a marker */
            }
        )*)*
    };
}
