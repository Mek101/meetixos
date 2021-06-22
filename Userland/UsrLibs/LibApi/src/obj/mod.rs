/*! Objects management */

use api_data::{
    obj::{
        info::RawObjInfo,
        modes::ObjRecvMode,
        types::ObjType,
        uses::ObjUseBits,
        RawObjHandle
    },
    sys::{
        codes::KernObjectFnId,
        fn_path::KernFnPath,
        RawKernHandle
    },
    task::thread::RWatchThreadEntry
};
use bits::flags::BitFlags;

use crate::{
    handle::{
        KernHandle,
        Result
    },
    obj::info::ObjInfo,
    task::Task
};

pub mod config;
pub mod grants;
pub mod impls;
pub mod info;

pub type ObjUseFilters = BitFlags<usize, ObjUseBits>;

#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
pub struct ObjHandle {
    m_handle: KernHandle
}

impl ObjHandle {
    fn send<T>(&self, recv_task: &T) -> Result<()>
        where T: Task {
        self.m_handle
            .inst_kern_call_1(KernFnPath::Object(KernObjectFnId::Send), 0)
            .map(|_| ())
    }

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

    fn drop_name(&self) -> Result<()> {
        self.m_handle
            .inst_kern_call_0(KernFnPath::Object(KernObjectFnId::DropName))
            .map(|_| ())
    }

    fn watch(&self,
             use_filter: ObjUseFilters,
             callback_fn: RWatchThreadEntry)
             -> Result<()> {
        extern "C" fn c_callback_entry() -> ! {
            unreachable!();
        }

        self.m_handle
            .inst_kern_call_3(KernFnPath::Object(KernObjectFnId::Watch),
                              use_filter.raw_bits(),
                              callback_fn as usize,
                              c_callback_entry as usize)
            .map(|_| ())
    }

    fn info(&self) -> Result<RawObjInfo>
        where T: Object {
        let mut raw_obj_info = RawObjInfo::default();
        self.m_handle
            .inst_kern_call_1(KernFnPath::Object(KernObjectFnId::Info),
                              raw_obj_info.as_syscall_ptr())
            .map(|_| raw_obj_info)
    }

    fn update_info(&self, raw_obj_info: &mut RawObjInfo) -> Result<()> {
        self.m_handle
            .inst_kern_call_1(KernFnPath::Object(KernObjectFnId::UpdateInfo),
                              raw_obj_info.as_syscall_ptr())
            .map(|_| ())
    }

    pub fn kern_handle(&self) -> &KernHandle {
        &self.m_handle
    }
}

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
     * Sends this `Object` instance to another `Task` to share the same
     * resource.
     *
     * The concurrency is managed internally by the Kernel with two
     * `RWLock`s (one for the data and one for the information), so
     * multiple tasks can read the data or the info but only one a time
     * can write them
     */
    fn send(&self, recv_task: &T) -> Result<()>
        where T: Task {
        self.obj_handle().send(recv_task)
    }

    /**  
     * Accepts an incoming `Object`
     *
     * The previous handle is first released with `Drop` then overwritten
     * with the new handle received according to the `RecvMode` given
     */
    fn recv(&mut self, recv_mode: ObjRecvMode) -> Result<()> {
        self.obj_handle_mut().recv(Self::TYPE, recv_mode)
    }

    /**
     * Convenience method that internally creates an uninitialized obj
     * instance then performs an `Object::recv()` using the given `RecvMode`
     */
    fn recv_new(recv_mode: ObjRecvMode) -> Result<Self> {
        let mut obj_handle = Self::default();
        obj_handle.recv(recv_mode).map(|_| obj_handle)
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

trait UserCreatableObject {}
