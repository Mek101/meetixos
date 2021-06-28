/*! Process `Task` reference */

use alloc::vec::Vec;

use api_data::{
    sys::{
        codes::KernProcFnId,
        fn_path::KernFnPath
    },
    task::{
        fs_types::FsType,
        modes::FsMountMode,
        types::TaskType
    }
};

use crate::{
    handle::{
        KernHandle,
        Result
    },
    obj::{
        grants::ObjGrants,
        impls::{
            device::Device,
            dir::Dir
        },
        Object
    },
    task::{
        thread::Thread,
        Task,
        TaskHandle
    }
};

#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct Proc {
    m_task_handle: TaskHandle
}

impl Proc {
    /**
     * Returns the main `Thread` of this process
     */
    pub fn main_thread(&self) -> Result<Thread> {
        self.kern_call_0(KernFnPath::Proc(KernProcFnId::MainThread))
            .map(|raw_thread_handle| {
                Thread::from(TaskHandle::from_raw(raw_thread_handle))
            })
    }

    pub fn sub_threads(&self) -> Result<Vec<Thread>> {
        let mut sub_threads_vec = Vec::with_capacity(self.threads_count()?);

        self.task_handle()
            .kern_handle()
            .inst_kern_call_2(KernFnPath::Proc(KernProcFnId::SubThreads),
                              sub_threads_vec.as_mut_ptr() as usize,
                              sub_threads_vec.capacity())
            .map(|_| sub_threads_vec)
    }

    pub fn threads_count(&self) -> Result<usize> {
        self.task_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::Proc(KernProcFnId::ThreadsCount))
    }

    /**
     * Mount a new filesystem instance at `mnt_point`.
     *
     * The Kernel loads a new filesystem instance which corresponds to the
     * given `FSType` and connects it to the empty `mnt_point` given.
     *
     * Depending on the `FSType` the `src_device` could be expected as valid
     * `Some` instance.
     *
     * The given `Grants` and the `MountMode` form the protection to the new
     * mounted filesystem
     */
    pub fn mount(fs_type: FsType,
                 src_device: Option<Device>,
                 mnt_point: &Dir,
                 mnt_point_grants: ObjGrants<Dir>,
                 mnt_mode: FsMountMode)
                 -> Result<()> {
        KernHandle::kern_call_6(KernFnPath::Proc(KernProcFnId::Mount),
                                fs_type.into(),
                                src_device.is_some() as usize,
                                src_device.unwrap_or_default()
                                          .obj_handle()
                                          .kern_handle()
                                          .raw_handle()
                                as usize,
                                mnt_point.obj_handle().kern_handle().raw_handle()
                                as usize,
                                mnt_point_grants.raw_bits(),
                                mnt_mode.into()).map(|_| ())
    }

    /**
     * Unmounts an already mounted filesystem
     */
    pub fn unmount(mnt_point: &Dir) -> Result<()> {
        KernHandle::kern_call_1(KernFnPath::Proc(KernProcFnId::UnMount),
                                mnt_point.obj_handle().kern_handle().raw_handle()
                                as usize).map(|_| ())
    }
}

impl From<TaskHandle> for Proc {
    fn from(task_handle: TaskHandle) -> Self {
        Self { m_task_handle: task_handle }
    }
}

impl Task for Proc {
    const TASK_TYPE: TaskType = TaskType::Proc;

    fn task_handle(&self) -> &TaskHandle {
        &self.m_task_handle
    }

    fn task_handle_mut(&mut self) -> &mut TaskHandle {
        &mut self.m_task_handle
    }
}
