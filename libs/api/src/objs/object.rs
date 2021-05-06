/*! `Object` handle */

use core::mem;

use os::sysc::{
    codes::KernObjectFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::{
        obj::{
            modes::RecvMode,
            types::ObjType,
            uses::ObjUse
        },
        task::data::thread::{
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
        config::ObjConfig,
        impls::any::Any,
        infos::info::ObjInfo
    },
    tasks::task::Task,
    time::Instant
};

/**
 * Object opaque handle.
 *
 * This object that takes place of the old style file descriptor
 * integer, used by all the Unix-like OS to keep reference to an
 * open resource  
 */
#[repr(transparent)]
#[derive(Debug, Default, Eq, PartialEq)]
pub struct ObjId(u32);

impl ObjId {
    /**
     * Constructs an un initialized `ObjId`.
     *
     * Used only by the `OsRawMutex` to satisfy the constant initialization
     */
    pub(crate) const fn const_new() -> Self {
        Self(0)
    }

    /**
     * Shares this `ObjId` with another `Task`
     */
    fn send<T>(&self, receiver: &T) -> Result<()>
        where T: Task {
        self.kern_call_1(KernFnPath::Object(KernObjectFnId::Send),
                         receiver.task_handle().id_usize())
            .map(|_| ())
    }

    /**
     * Accepts an incoming `ObjId`
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

    /**  
     * Updates the infos of this object.
     *
     * Internally used by `ObjInfo::update()`
     */
    pub(crate) fn update_infos<T>(&self, infos: &ObjInfo<T>) -> Result<()>
        where T: Object {
        self.kern_call_1(KernFnPath::Object(KernObjectFnId::UpdateInfo),
                         infos as *const _ as usize)
            .map(|_| ())
    }

    /**
     * Makes the object no longer reachable via the VFS.
     *
     * When all the tasks, that keep a reference to it, drop it, it will be
     * definitively destroyed by the kernel
     */
    fn drop_name(&self) -> Result<()> {
        self.kern_call_0(KernFnPath::Object(KernObjectFnId::DropName)).map(|_| ())
    }

    /**
     * Registers the given `callback` to be executed whenever one of the
     * bitwise given `ObjUse` happen
     */
    fn watch(&self, filter: ObjUse, callback_fn: RWatchCBThreadEntry) -> Result<()> {
        let thread_entry_data = ThreadEntryData::new_watch_callback(callback_fn);
        self.kern_call_2(KernFnPath::Object(KernObjectFnId::Watch),
                         filter.into(),
                         &thread_entry_data as *const _ as usize)
            .map(|_| ())
    }

    /**
     * Returns the `ObjInfo` of this object
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

    /**
     * Returns whether this object instance references a still valid kernel
     * object
     */
    pub fn is_valid(&self) -> bool {
        self.0 != 0
        && self.kern_call_0(KernFnPath::Object(KernObjectFnId::IsValid))
               .map(|_| true)
               .unwrap_or(false)
    }

    /**
     * Returns the raw identifier of this `ObjId`
     */
    pub fn as_raw(&self) -> u32 {
        self.0
    }

    /**
     * Returns the raw identifier of this `ObjId` as `usize`
     */
    pub fn as_raw_usize(&self) -> usize {
        self.as_raw() as usize
    }
}

impl Clone for ObjId {
    /**
     * Increases the references count to the object referenced.
     *
     * The returned `ObjId` is a new instance but reference the same
     * kernel's object, so changes on any of the cloned instances affect the
     * same kernel's object
     */
    fn clone(&self) -> Self {
        self.kern_call_0(KernFnPath::Object(KernObjectFnId::AddRef))
            .map(|_| Self::from(self.0))
            .unwrap()
    }
}

impl Drop for ObjId {
    /**
     * Decreases by one the references count to the referenced kernel's
     * object.
     *
     * The life of the objects varies by type:
     *
     * Permanent objects, like `File`s, `Dir`ectories, `Link`s and
     * `OsRawMutex`es, persists until they are explicitly destroyed with
     * `Object::drop_name()`.
     *
     * The other kind of objects, like `MMap`s and `IpcChan`nels, live
     * until there is a reference to them. When the references reaches the 0
     * they are definitely destroyed
     */
    fn drop(&mut self) {
        if self.is_valid() {
            self.kern_call_0(KernFnPath::Object(KernObjectFnId::Drop)).unwrap();
        }
    }
}

impl From<u32> for ObjId {
    fn from(raw_id: u32) -> Self {
        Self(raw_id)
    }
}

impl From<usize> for ObjId {
    fn from(raw_id: usize) -> Self {
        Self::from(raw_id as u32)
    }
}

impl KernCaller for ObjId {
    fn caller_handle_bits(&self) -> u32 {
        self.as_raw()
    }
}

/**
 * Common interface implemented by all the `ObjId` based objects.
 *
 * It mainly exposes the private methods of the `ObjId` for safe calling
 * and provides convenient methods to easily perform works that normally
 * implies more than one call
 */
pub trait Object: From<ObjId> + Default + Clone + Sync + Send {
    /**
     * The value of the `ObjType` that matches the implementation
     */
    const TYPE: ObjType;

    /**
     * Returns the immutable reference to the underling `ObjId` instance
     */
    fn obj_handle(&self) -> &ObjId;

    /**
     * Returns the mutable reference to the underling `ObjId` instance
     */
    fn obj_handle_mut(&mut self) -> &mut ObjId;

    /**
     * Returns an uninitialized `ObjConfig` to open an existing `Object`
     */
    fn open() -> ObjConfig<Self, FindMode> {
        ObjConfig::<Self, FindMode>::new()
    }

    /**
     * Consumes the object into his `ObjId` instance
     */
    fn into_id(self) -> ObjId {
        let raw_id = self.obj_handle().as_raw();
        mem::forget(self);
        ObjId::from(raw_id)
    }

    /**
     * Consumes the object upcasting it to an `Any` instance
     */
    fn into_any(self) -> Any {
        Any::from(self.into_id())
    }

    /**
     * Makes the object no longer reachable via the VFS.
     *
     * When all the tasks, that keep a reference to it, drop it, it will be
     * definitively destroyed by the kernel
     */
    fn drop_name(&self) -> Result<()> {
        self.obj_handle().drop_name()
    }

    /**
     * Registers the given `callback` to be executed whenever one of the
     * bitwise given `ObjUse` happen.
     *
     * The caller must have information read grants to successfully call
     * this method.
     *
     * The given `callback` must accept an `ObjUseInstant` as argument and
     * must return a boolean that tells to the kernel whether the callback
     * must be re-called for the next event given via `filter` or must
     * be unregistered.
     *
     * Multiple `callback`s can be registered for different uses, but if the
     * given filters overlaps a previously registered callback an error will
     * be returned
     */
    fn watch(&self, filter: ObjUse, callback_fn: RWatchCBThreadEntry) -> Result<()> {
        self.obj_handle().watch(filter, callback_fn)
    }

    /** # Share this `Object` with another `Task`
     *
     * Sends this object instance to another `Task` to share the same
     * resource.
     *
     * The concurrency is managed internally by the kernel with two
     * `RWLock`s (one for the data and one for the informations), so
     * multiple tasks can read the data or the infos but only one a time
     * can write them
     */
    fn send<T>(&self, task: &T) -> Result<()>
        where T: Task {
        self.obj_handle().send(task)
    }

    /**  
     * Accepts an incoming `Object`
     *
     * The previous handle is first released with `Drop` then overwritten
     * with the new handle received according to the `RecvMode` given
     */
    fn recv(&mut self, mode: RecvMode) -> Result<()> {
        self.obj_handle_mut().recv(Self::TYPE, mode)
    }

    /**
     * Convenience method that internally creates an uninitialized object
     * instance then performs an `Object::recv()` using the given `RecvMode`
     */
    fn recv_new(mode: RecvMode) -> Result<Self> {
        let mut obj = Self::default();
        obj.recv(mode).map(|_| obj)
    }

    /**
     * Returns the `ObjInfo` of this object
     */
    fn infos(&self) -> Result<ObjInfo<Self>> {
        self.obj_handle().infos()
    }

    /**
     * Returns the `ObjType` of the object
     */
    fn obj_type(&self) -> ObjType {
        self.infos().unwrap_or_default().obj_type()
    }

    /**
     * Returns all the `Instant` timestamps ordered as
     *
     * 0. Creation `Instant`
     * 1. Last access `Instant`
     * 2. Last data modify `Instant`
     * 3. Last info modify `Instant`
     */
    fn timestamps(&self) -> (Instant, Instant, Instant, Instant) {
        self.infos().unwrap_or_default().timestamps()
    }

    /**
     * Returns the size of the object's data in bytes
     */
    fn size(&self) -> usize {
        self.infos().unwrap_or_default().size()
    }
}

/**
 * Interface implemented for all the user creatable objects
 */
pub trait UserCreatable: Object {
    /**
     * Returns an uninitialized `ObjConfig` to create a new `Object`
     */
    fn creat() -> ObjConfig<Self, CreatMode> {
        ObjConfig::<Self, CreatMode>::new()
    }
}
