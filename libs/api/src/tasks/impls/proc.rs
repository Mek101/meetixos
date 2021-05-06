/*! Process `Task` reference */

use os::sysc::{
    codes::KernProcFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::task::types::TaskType,
    caller::{
        KernCaller,
        Result
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
