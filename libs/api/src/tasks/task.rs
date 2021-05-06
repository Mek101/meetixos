/*! `Task` Handle */

use os::sysc::{
    codes::KernTaskFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::task::types::TaskType,
    caller::{
        KernCaller,
        Result
    },
    config::{
        CreatMode,
        FindMode
    },
    tasks::config::TaskConfig
};

/**
 * Task opaque handle.
 *
 * This object takes place of the old style [`pid_t`] and [`pthread_t`]
 * types, used by all the Unix-like OS to identify processes and threads
 */
#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct TaskId {
    m_raw: u32
}

impl TaskId {
    /**
     * Obtains the current `TaskId` of the `TaskType` given
     */
    fn this(task_type: TaskType) -> Self {
        let mut task = Self::default();
        task.kern_call_1(KernFnPath::Task(KernTaskFnId::This), task_type.into())
            .map(|task_id| {
                task.m_raw = task_id as u32;
                task
            })
            .unwrap()
    }

    /**
     * Terminates this `TaskId`
     */
    fn terminate(&self, call_cleanup: bool) -> Result<()> {
        self.kern_call_1(KernFnPath::Task(KernTaskFnId::Terminate), call_cleanup as usize)
            .map(|_| ())
    }

    /**  
     * Yields the execution
     */
    fn yield_next(&self) {
        self.kern_call_0(KernFnPath::Task(KernTaskFnId::Yield)).unwrap();
    }

    /**
     * Returns whether the referenced task is alive
     */
    fn is_alive(&self) -> bool {
        self.m_raw != 0
        && self.kern_call_0(KernFnPath::Task(KernTaskFnId::IsAlive))
               .map(|res| res != 0)
               .unwrap_or(false)
    }

    /**
     * Returns the raw identifier of this `TaskId`
     */
    pub fn id(&self) -> u32 {
        self.m_raw
    }

    /**
     * Returns the raw identifier of this `TaskId` as `usize`
     */
    pub fn id_usize(&self) -> usize {
        self.id() as usize
    }
}

impl From<u32> for TaskId {
    fn from(raw_id: u32) -> Self {
        Self { m_raw: raw_id }
    }
}

impl From<usize> for TaskId {
    fn from(raw_id: usize) -> Self {
        Self::from(raw_id as u32)
    }
}

impl KernCaller for TaskId {
    fn caller_handle_bits(&self) -> u32 {
        self.id()
    }
}

/**
 * Common interface implemented by all the `TaskId` based objects.
 *
 * It mainly exposes the private methods of the `TaskId` for safe calling
 * and provides convenient methods to easily perform works that normally
 * implies more than one call
 */
pub trait Task: From<TaskId> + Sync + Send {
    /**
     * The value of the `TaskType` that matches the implementation
     */
    const TASK_TYPE: TaskType;

    /**
     * Returns the immutable reference to the underling `TaskId` instance
     */
    fn task_handle(&self) -> &TaskId;

    /**
     * Returns the mutable reference to the underling `TaskId` instance
     */
    fn task_handle_mut(&mut self) -> &mut TaskId;

    /**
     * Returns an uninitialized `TaskConfig` to spawn a new `Task`
     */
    fn spawn() -> TaskConfig<Self, CreatMode> {
        TaskConfig::new()
    }

    /**
     * Returns an uninitialized `TaskConfig` to find an existing `Task`
     */
    fn find() -> TaskConfig<Self, FindMode> {
        TaskConfig::new()
    }

    /**
     * Obtains the current `Task` reference according to the wrapping type
     * (i.e `Proc` or `Thread`)
     */
    fn this() -> Self {
        Self::from(TaskId::this(Self::TASK_TYPE))
    }

    /**
     * Returns the numeric identifier of this `Task`
     */
    fn id(&self) -> u32 {
        self.task_handle().id()
    }

    /**
     * Terminates the task referenced by this `Task` if the caller
     * user/group is the same of the `Task` to terminate or it's the
     * administrator.
     *
     * The `call_cleanup` flag tells to the kernel whether it must call the
     * registered cleanup function(s) (if any) before definitively terminate
     * the target.
     *
     * If the target is a `Proc` and it have more than one active
     * `Thread` the kernel first terminates them all, then terminates
     * the `Proc`
     */
    fn terminate(&self, call_cleanup: bool) -> Result<()> {
        self.task_handle().terminate(call_cleanup)
    }

    /**
     * Preempts the remaining CPU quantum of this `Task` if it was spawned
     * with `SchedPolicy::Preemptive`, or to release the CPU for the
     * other tasks if it was spawner with `SchedPolicy::Cooperative`
     */
    fn yield_next(&self) {
        self.task_handle().yield_next()
    }

    /**
     * Returns whether the referenced `Task` is alive
     */
    fn is_alive(&self) -> bool {
        self.task_handle().is_alive()
    }
}
