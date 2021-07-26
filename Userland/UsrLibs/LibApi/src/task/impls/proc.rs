/*! Process `Task` reference */

use alloc::vec::Vec;

use api_data::{
    entity::OsEntityId,
    sys::{
        codes::KernProcFnId,
        fn_path::KernFnPath
    },
    task::{
        fs_types::FsType,
        modes::FsMountMode,
        types::TaskType,
        TaskId
    }
};

use crate::{
    kern_handle::{
        KernHandle,
        Result
    },
    object::{
        grants::ObjGrants,
        impls::{
            device::Device,
            dir::Dir
        },
        ObjHandle,
        TObject
    },
    task::{
        TTask,
        TaskHandle
    }
};

/**
 * Reference to a program in execution
 */
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

impl Proc /* Methods */ {
    /**
     * Returns the `OsEntityId` of the `OsUser` which owns this `Proc`ess
     */
    pub fn os_user(&self) -> Result<OsEntityId> {
        self.task_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::Proc(KernProcFnId::OsUser))
            .map(|os_entity_id| os_entity_id as OsEntityId)
    }

    /**
     * Returns the `OsEntityId` of the `OsGroup` which owns this `Proc`ess
     */
    pub fn os_group(&self) -> Result<OsEntityId> {
        self.task_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::Proc(KernProcFnId::OsGroup))
            .map(|os_entity_id| os_entity_id as OsEntityId)
    }

    /**
     * Returns the current workdir of the `Proc`
     */
    pub fn work_dir(&self) -> Result<Dir> {
        self.task_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::Proc(KernProcFnId::GetWorkDir))
            .map(|raw_work_dir_handle| {
                Dir::from(ObjHandle::from_raw(raw_work_dir_handle))
            })
    }

    /**
     * Sets the current working `Dir` for this `Proc`.
     *
     * It is allowed for administrative processes to change working `Dir` to
     * other processes
     */
    pub fn set_work_dir(&self, work_dir: &Dir) -> Result<()> {
        self.task_handle()
            .kern_handle()
            .inst_kern_call_1(KernFnPath::Proc(KernProcFnId::SetWorkDir),
                              work_dir.obj_handle().kern_handle().raw_handle() as usize)
            .map(|_| ())
    }

    /**
     * Returns the `TaskId` of the main `Thread` of this `Proc`
     */
    pub fn main_thread_id(&self) -> Result<TaskId> {
        self.task_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::Proc(KernProcFnId::MainThread))
            .map(|raw_task_id| raw_task_id as TaskId)
    }

    /**
     * Returns the `Vec` with all the `TaskId`s of the sub-`Thread`s of this
     * `Proc`
     */
    pub fn sub_threads_ids(&self) -> Result<Vec<TaskId>> {
        let mut sub_threads_ids_vec = Vec::with_capacity(self.threads_count()?);

        self.task_handle()
            .kern_handle()
            .inst_kern_call_2(KernFnPath::Proc(KernProcFnId::SubThreads),
                              sub_threads_ids_vec.as_mut_ptr() as usize,
                              sub_threads_ids_vec.capacity())
            .map(|sub_threads_count| {
                unsafe {
                    sub_threads_ids_vec.set_len(sub_threads_count);
                }
                sub_threads_ids_vec
            })
    }

    /**
     * Returns the total amount of `Thread`s owned by this `Proc`
     */
    pub fn threads_count(&self) -> Result<usize> {
        self.task_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::Proc(KernProcFnId::ThreadsCount))
    }
}

impl Proc /* Static Functions */ {
    /**
     * Mount a new filesystem instance at `mnt_point`.
     *
     * The Kernel loads a new filesystem instance which corresponds to
     * the given `FSType` and connects it to the empty `mnt_point`
     * given.
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

impl TTask for Proc {
    const TASK_TYPE: TaskType = TaskType::Proc;

    fn task_handle(&self) -> &TaskHandle {
        &self.m_task_handle
    }

    fn task_handle_mut(&mut self) -> &mut TaskHandle {
        &mut self.m_task_handle
    }
}
