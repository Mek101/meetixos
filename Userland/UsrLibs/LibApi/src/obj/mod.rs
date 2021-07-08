/*! Objects management */

/* TODO resolve Into<Object> and TryInto<Object> implementations
 * use core::{
 *     any::type_name,
 *     convert::TryInto,
 *     result
 * };
 * use api_data::error::{
 *     class::OsErrorClass,
 *     OsError
 * };
 * use crate::task::impls::{
 *    Proc,
 *    Thread
 * };
 */

use api_data::{
    obj::{
        info::RawObjInfo,
        modes::ObjRecvMode,
        types::ObjType,
        uses::ObjUseBits
    },
    sys::{
        codes::KernObjectFnId,
        fn_path::KernFnPath,
        AsSysCallPtr
    },
    task::thread::RWatchThreadEntry
};
use bits::flags::BitFlags;

use crate::{
    config::{
        CreatMode,
        OpenMode
    },
    handle::{
        KernHandle,
        Result
    },
    obj::{
        config::ObjConfig,
        info::ObjInfo
    },
    task::{
        impls::thread::c_thread_entry,
        Task
    }
};

pub mod config;
pub mod grants;
pub mod impls;
pub mod info;

/**
 * `Object::watch()` use filter flags
 */
pub type ObjUseFilters = BitFlags<usize, ObjUseBits>;

/**
 * Generic opaque `Object` handle
 */
#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct ObjHandle {
    m_handle: KernHandle
}

impl ObjHandle {
    /**
     * Constructs an `ObjHandle` from the `raw_handle` value given
     */
    pub(crate) fn from_raw(raw_handle: usize) -> Self {
        Self { m_handle: KernHandle::from_raw(raw_handle) }
    }

    /**
     * Shares this handle with the given `Task`
     */
    fn send<T>(&self, recv_task: &T) -> Result<()>
        where T: Task {
        self.m_handle
            .inst_kern_call_1(KernFnPath::Object(KernObjectFnId::Send),
                              recv_task.task_handle().kern_handle().raw_handle() as usize)
            .map(|_| ())
    }

    /**
     * Overwrites this handle with an incoming one according to the given
     * `ObjType` and the `ObjRecvMode`
     */
    fn recv(&mut self, obj_type: ObjType, recv_mode: ObjRecvMode) -> Result<()> {
        self.m_handle
            .inst_kern_call_2(KernFnPath::Object(KernObjectFnId::Recv),
                              obj_type.into(),
                              recv_mode.into())
            .map(|raw_obj_handle| {
                *self = Self { m_handle: KernHandle::from_raw(raw_obj_handle) };
                ()
            })
    }

    /**
     * Makes this named `Object` no longer reachable via the VFS
     */
    fn drop_name(&self) -> Result<()> {
        self.m_handle
            .inst_kern_call_0(KernFnPath::Object(KernObjectFnId::DropName))
            .map(|_| ())
    }

    /**
     * Registers the given `callback` to be executed whenever one of the
     * given `ObjUseBits` happen
     */
    fn watch(&self,
             use_filter: ObjUseFilters,
             callback_fn: RWatchThreadEntry)
             -> Result<()> {
        self.m_handle
            .inst_kern_call_3(KernFnPath::Object(KernObjectFnId::Watch),
                              use_filter.raw_bits(),
                              callback_fn as usize,
                              c_thread_entry as usize)
            .map(|_| ())
    }

    /**
     * Returns the `RawObjInfo` metadata of this `Object`
     */
    fn info(&self) -> Result<RawObjInfo> {
        let mut raw_obj_info = RawObjInfo::default();
        self.m_handle
            .inst_kern_call_1(KernFnPath::Object(KernObjectFnId::Info),
                              raw_obj_info.as_syscall_ptr_mut())
            .map(|_| raw_obj_info)
    }

    /**
     * Updates the `Object`'s metadata
     */
    pub(crate) fn update_info(&self, raw_obj_info: &mut RawObjInfo) -> Result<()> {
        self.m_handle
            .inst_kern_call_1(KernFnPath::Object(KernObjectFnId::UpdateInfo),
                              raw_obj_info.as_syscall_ptr())
            .map(|_| ())
    }

    /**
     * Returns the reference to the underling `KernHandle`
     */
    #[inline]
    pub fn kern_handle(&self) -> &KernHandle {
        &self.m_handle
    }
}

/* TODO: the compiler reports: error: cannot specialize on trait
 * `obj::Object` impl<T> Into<T> for ObjHandle where T: Object {
 *     fn into(self) -> T {
 *         let real_obj_type = self.boot().unwrap_or_default().obj_type();
 *
 *         if real_obj_type == T::TYPE {
 *             T::from(self)
 *         } else {
 *             panic!("ObjHandle({})::into::<{}>() - Failed, {} != {}",
 *                    self.kern_handle().raw_handle(),
 *                    type_name::<T>(),
 *                    real_obj_type,
 *                    T::TYPE);
 *         }
 *     }
 * }
 *
 * impl<T> TryInto<T> for ObjHandle where T: Object {
 *     type Error = OsError;
 *
 *     fn try_into(self) -> result::Result<T, Self::Error> {
 *         let real_obj_type = self.boot()?.obj_type();
 *
 *         if real_obj_type == T::TYPE {
 *             Ok(T::from(self))
 *         } else {
 *             Err(OsError::new(OsErrorClass::TypesNotMatch,
 *                              KernFnPath::Invalid,
 *                              Some(self.kern_handle().raw_handle()),
 *                              Proc::this().os_id()?,
 *                              Thread::this().os_id()?,
 *                              None))
 *         }
 *     }
 * }
 */

/**
 * Common interface implemented by all the `ObjHandle` based objects.
 *
 * It mainly exposes the private methods of the `ObjHandle` for safe calling
 * and provides convenient methods to easily perform works that normally
 * implies more than one call
 */
pub trait Object: From<ObjHandle> + Default + Clone {
    /**
     * The value of the `ObjType` that matches the implementation
     */
    const TYPE: ObjType;

    /**
     * Returns the immutable reference to the underling `ObjHandle` instance
     */
    fn obj_handle(&self) -> &ObjHandle;

    /**
     * Returns the mutable reference to the underling `ObjHandle` instance
     */
    fn obj_handle_mut(&mut self) -> &mut ObjHandle;

    /**
     * Returns an `ObjConfig` for `Object` opening
     */
    fn open<'a>() -> ObjConfig<'a, Self, OpenMode> {
        ObjConfig::<Self, OpenMode>::new()
    }

    /**
     * Shares this `Object` instance with the given `Task`
     */
    fn send<T>(&self, recv_task: &T) -> Result<()>
        where T: Task {
        self.obj_handle().send(recv_task)
    }

    /**
     * Overwrites this `Object` with an incoming one according to the given
     * `ObjRecvMode`
     */
    fn recv(&mut self, recv_mode: ObjRecvMode) -> Result<()> {
        self.obj_handle_mut().recv(Self::TYPE, recv_mode)
    }

    /**
     * Convenience method that internally creates an uninitialized object
     * instance then performs an `Object::recv()` using the given `RecvMode`
     */
    fn recv_new(recv_mode: ObjRecvMode) -> Result<Self> {
        let mut obj_handle = Self::default();
        obj_handle.recv(recv_mode).map(|_| obj_handle)
    }

    /**
     * Makes this named `Object` no longer reachable via the VFS
     */
    fn drop_name(&self) -> Result<()> {
        self.obj_handle().drop_name()
    }

    /**
     * Registers the given `callback` to be executed whenever one of the
     * given `ObjUseBits` happen
     *
     * The caller must have boot-read grants to successfully call
     * this method.
     *
     * The given `callback` must accept an `ObjUseInstant` as argument and
     * must return a boolean that tells to the Kernel whether the
     * callback must be re-called for the next event given via `filter`
     * or must be unregistered.
     *
     * Multiple `callback`s can be registered for different uses, but if the
     * given filters overlaps a previously registered callback an error will
     * be returned
     */
    fn watch(&self,
             use_filter: ObjUseFilters,
             callback_fn: RWatchThreadEntry)
             -> Result<()> {
        self.obj_handle().watch(use_filter, callback_fn)
    }

    /**
     * Returns the `ObjInfo` of this `Object`
     */
    fn info(&self) -> Result<ObjInfo<Self>> {
        self.obj_handle()
            .info()
            .map(|raw_obj_info| ObjInfo::new(raw_obj_info, self.obj_handle().clone()))
    }
}

/**
 * Marker interface for `Object`s which support `Object::creat()`
 */
pub trait UserCreatableObject: Object {
    /**
     * Returns an `ObjConfig` for `Object` creation
     */
    fn creat<'a>() -> ObjConfig<'a, Self, CreatMode> {
        ObjConfig::<Self, CreatMode>::new()
    }
}

/**
 * Marker interface for `Object`s which support
 * `ObjConfig::with_data_size()`
 */
pub trait SizeableDataObject {
    /* No methods, just a marker trait */
}

/**
 * Marker interface for `Object`s which support `ObjConfig::for_exec()`
 */
pub trait ExecutableDataObject {
    /* No methods, just a marker trait */
}

/**
 * Marker interface for `Object`s which support
 * `ObjConfig::apply_for_anon()`
 */
pub trait AnonymousObject {
    /* No methods, just a marker trait */
}
