/*! # Process Management
 *
 * Implements the running process reference
 */

use os::sysc::{
    codes::KernProcFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::task::TaskType,
    caller::{
        KernCaller,
        Result
    },
    tasks::{
        impls::Thread,
        Task,
        TaskId
    }
};

/** # Running `Process`
 *
 * Represents a reference to context that is being executing at least
 * one [`Thread`]
 *
 * [`Thread`]: crate::tasks::impls::Thread
 */
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Proc {
    m_handle: TaskId
}

impl Proc {
    /** Returns the main [`Thread`] of this process
     *
     * [`Thread`]: crate::tasks::impls::Thread
     */
    pub fn main_thread(&self) -> Result<Thread> {
        self.kern_call_0(KernFnPath::Proc(KernProcFnId::MainThread))
            .map(|thr_id| Thread::from(TaskId::from(thr_id)))
    }
}

impl Task for Proc {
    /** The value of the [`TaskType`] that matches the implementation
     *
     * [`TaskType`]: crate::bits::task::types::TaskType
     */
    const TASK_TYPE: TaskType = TaskType::Proc;

    /** Returns the immutable reference to the underling [`TaskId`] instance
     *
     * [`TaskId`]: crate::tasks::TaskId
     */
    fn task_handle(&self) -> &TaskId {
        &self.m_handle
    }

    /** Returns the mutable reference to the underling [`TaskId`] instance
     *
     * [`TaskId`]: crate::tasks::TaskId
     */
    fn task_handle_mut(&mut self) -> &mut TaskId {
        &mut self.m_handle
    }
}

impl From<TaskId> for Proc {
    /** Performs the conversion
     */
    fn from(id: TaskId) -> Self {
        Self { m_handle: id }
    }
}

impl KernCaller for Proc {
    /** Returns the upper 32bits of the 64bit identifier of a system call
     */
    fn caller_handle_bits(&self) -> u32 {
        self.task_handle().caller_handle_bits()
    }
}
