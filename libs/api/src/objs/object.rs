/*! # `Object` Handle
 *
 * Implements the base struct and the trait used as base for the kernel's
 * managed objects
 */

use core::mem;

use os::sysc::{
    codes::KernObjectFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::{
        obj::{
            ObjType,
            ObjUse,
            RecvMode
        },
        task::{
            RWatchCBThreadEntry,
            ThreadEntryData
        }
    },
    caller::{
        KernCaller,
        Result
    },
    config::{
        CreatMode,
        FindMode
    },
    objs::{
        impls::Any,
        infos::ObjInfo,
        ObjConfig
    },
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
 * [`Object`]: crate::objs::object::Object
 * [implementations]: /api/objs/impls/index.html
 * [`ObjId`]: crate::objs::object::ObjId
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
     * [`OsRawMutex`]: crate::objs::impls::mutex::OsRawMutex
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
     * [`Task`]: crate::tasks::task::Task
     * [`Thread`]: crate::tasks::impls::thread::Thread
     * [`Process`]: crate::tasks::impls::proc::Proc
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
     * [`Drop`]: core::ops::Drop
     * [`RecvMode`]: crate::bits::obj::modes::RecvMode
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
     * [`ObjInfo::update()`]: crate::objs::infos::info::ObjInfo::update
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
     * The caller must have [information read grants](RG) to successfully
     * call this method.
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
     * [`ObjUse`]: crate::bits::obj::uses::ObjUse
     * [RG]: crate::bits::obj::grants::Grants::set_info_readable
     * [`ObjUseInstant`]: crate::objs::infos::use_instant::ObjUseInstant
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
     * [`ObjInfo`]: crate::objs::infos::info::ObjInfo
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
     * [`ObjId`]: crate::objs::object::ObjId
     */
    pub fn as_raw(&self) -> u32 {
        self.0
    }

    /** Returns the raw identifier of this [`ObjId`] as `usize`
     *
     * [`ObjId`]: crate::objs::object::ObjId
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
     * [`ObjId`]: crate::objs::object::ObjId
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
     * [`File`]: crate::objs::impls::file::File
     * [`Dir`]: crate::objs::impls::dir::Dir
     * [`Link`]: crate::objs::impls::link::Link
     * [`OsRawMutex`]: crate::objs::impls::mutex::OsRawMutex
     * [`Object::drop_name()`]: crate::objs::object::Object::drop_name
     * [`MMap`]: crate::objs::impls::mmap::MMap
     * [`IpcChan`]: crate::objs::impls::ipc_chan::IpcChan
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
 * [`ObjId`]: crate::objs::object::ObjId
 */
pub trait Object: From<ObjId> + Default + Clone + Sync + Send {
    /** The value of the [`ObjType`] that matches the implementation
     *
     * [`ObjType`]: crate::bits::obj::types::ObjType
     */
    const TYPE: ObjType;

    /** Returns the immutable reference to the underling [`ObjId`] instance
     *
     * [`ObjId`]: crate::objs::object::ObjId
     */
    fn obj_handle(&self) -> &ObjId;

    /** Returns the mutable reference to the underling [`ObjId`] instance
     *
     * [`ObjId`]: crate::objs::object::ObjId
     */
    fn obj_handle_mut(&mut self) -> &mut ObjId;

    /** Returns an uninitialized [`ObjConfig`] to open an existing [`Object`]
     *
     * [`ObjConfig`]: crate::objs::config::ObjConfig
     * [`Object`]: crate::objs::object::Object
     */
    fn open() -> ObjConfig<Self, FindMode> {
        ObjConfig::<Self, FindMode>::new()
    }

    /** # Obtain the underling `ObjId`
     *
     * Consumes the object into his [`ObjId`] instance
     *
     * [`ObjId`]: crate::objs::object::ObjId
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
     * [`Any`]: crate::objs::impls::any::Any
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
     * The caller must have [information read grants](RG) to successfully
     * call this method.
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
     * [`ObjUse`]: crate::bits::obj::uses::ObjUse
     * [RG]: crate::bits::obj::grants::Grants::set_info_readable
     * [`ObjUseInstant`]: crate::objs::infos::use_instant::ObjUseInstant
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
     * [`Task`]: crate::tasks::task::Task
     * [`Thread`]: crate::tasks::impls::thread::Thread
     * [`Process`]: crate::tasks::impls::proc::Proc
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
     * [`Drop`]: core::ops::Drop
     * [`RecvMode`]: crate::bits::obj::modes::RecvMode
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
     * [`Object::recv()`]: crate::objs::object::Object::recv
     * [`RecvMode`]: crate::bits::obj::modes::RecvMode
     */
    fn recv_new(mode: RecvMode) -> Result<Self> {
        let mut obj = Self::default();
        obj.recv(mode).map(|_| obj)
    }

    /** Returns the [`ObjInfo`] of this object
     *
     * [`ObjInfo`]: crate::objs::infos::info::ObjInfo
     */
    fn infos(&self) -> Result<ObjInfo<Self>> {
        self.obj_handle().infos()
    }

    /** Returns the [`ObjType`] of the object
     *
     * [`ObjType`]: crate::bits::obj::types::ObjType
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
     * [`Instant`]: crate::time::Instant
     */
    fn timestamps(&self) -> (Instant, Instant, Instant, Instant) {
        self.infos().unwrap_or_default().timestamps()
    }

    /** Returns the [size] of the object
     *
     * [size]: crate::objs::infos::info::ObjInfo::size
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
     * [`ObjConfig`]: crate::objs::config::ObjConfig
     * [`Object`]: crate::objs::object::Object
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
            const TYPE: ObjType = $ObjType;

            fn obj_handle(&self) -> &ObjId {
                &self.0
            }

            fn obj_handle_mut(&mut self) -> &mut ObjId {
                &mut self.0
            }
        }

        impl From<ObjId> for $ObjTypeName {
            fn from(id: ObjId) -> Self {
                Self(id)
            }
        }

        impl KernCaller for $ObjTypeName {
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
