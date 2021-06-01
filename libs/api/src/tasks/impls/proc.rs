/*! Process `Task` reference */

use os::sysc::{
    codes::KernProcFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::{
        obj::grants::Grants,
        task::{
            fs_types::FSType,
            modes::MountMode,
            types::TaskType
        }
    },
    caller::{
        KernCaller,
        Result
    },
    objs::{
        impls::{
            device::Device,
            dir::Dir
        },
        object::Object
    },
    tasks::{
        impls::thread::Thread,
        task::{
            Task,
            TaskId
        }
    }
};

/**
 * Reference to a context that is being executing at least one `Thread`, if
 * alive
 */
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Proc {
    m_handle: TaskId
}

impl Proc {
    /**
     * Returns the main `Thread` of this process
     */
    pub fn main_thread(&self) -> Result<Thread> {
        self.kern_call_0(KernFnPath::Proc(KernProcFnId::MainThread))
            .map(|thr_id| Thread::from(TaskId::from(thr_id)))
    }

    /**
     * Mount a new filesystem instance at `mnt_point`.
     *
     * The kernel loads a new filesystem instance which corresponds to the
     * given `FSType` and connects it to the empty `mnt_point` given.
     *
     * Depending on the `FSType` the `src_device` could be expected as valid
     * `Some` instance.
     *
     * The given `Grants` and the `MountMode` form the protection to the new
     * mounted filesystem
     */
    pub fn mount(fs_type: FSType,
                 src_device: Option<Device>,
                 mnt_point: &Dir,
                 mnt_point_grants: Grants<Dir>,
                 mount_mode: MountMode)
                 -> Result<()> {
        let args = [fs_type.into(),
                    src_device.is_some() as usize,
                    src_device.unwrap_or_default().obj_handle().as_raw_usize(),
                    mnt_point.obj_handle().as_raw_usize(),
                    mnt_point_grants.as_raw_usize(),
                    mount_mode.into()];

        Self::this().kern_call_2(KernFnPath::Proc(KernProcFnId::Mount),
                                 args.as_ptr() as usize,
                                 args.len())
                    .map(|_| ())
    }

    /**
     * Unmounts an already mounted filesystem
     */
    pub fn unmount(mnt_point: Dir) -> Result<()> {
        Self::this().kern_call_1(KernFnPath::Proc(KernProcFnId::UnMount),
                                 mnt_point.obj_handle().as_raw_usize())
                    .map(|_| ())
    }
}

impl Task for Proc {
    const TASK_TYPE: TaskType = TaskType::Proc;

    fn task_handle(&self) -> &TaskId {
        &self.m_handle
    }

    fn task_handle_mut(&mut self) -> &mut TaskId {
        &mut self.m_handle
    }
}

impl From<TaskId> for Proc {
    fn from(id: TaskId) -> Self {
        Self { m_handle: id }
    }
}

impl KernCaller for Proc {
    fn caller_handle_bits(&self) -> u32 {
        self.task_handle().caller_handle_bits()
    }
}
