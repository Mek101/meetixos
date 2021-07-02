/*! Tasks management */

use api_data::{
    sys::{
        codes::KernTaskFnId,
        fn_path::KernFnPath,
        AsSysCallPtr,
        RawKernHandle
    },
    task::{
        exit::TaskExitStatus,
        types::TaskType,
        TaskId
    }
};

use crate::{
    config::{
        CreatMode,
        OpenMode
    },
    handle::{
        KernHandle,
        Result
    },
    task::config::TaskConfig
};

pub mod config;
pub mod impls;

/**
 * Generic opaque `Task` handle
 */
#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct TaskHandle {
    m_handle: KernHandle
}

impl TaskHandle {
    /**
     * Constructs an `TaskHandle` from the `raw_handle` value given
     */
    pub(crate) fn from_raw(raw_handle: usize) -> Self {
        Self { m_handle: KernHandle::from_raw(raw_handle) }
    }

    /**
     * Obtains the current `TaskHandle` of the `TaskType` given
     */
    fn this(task_type: TaskType) -> Self {
        KernHandle::kern_call_1(KernFnPath::Task(KernTaskFnId::This), task_type.into())
                   .map(|raw_task_handle| Self { m_handle: KernHandle::from_raw(raw_task_handle) })
                   .expect("Failed to obtain current Task handle")
    }

    /**
     * Exists the current `TaskHandle` with the given `TaskExitStatus`
     */
    fn exit(task_type: TaskType, exit_status: TaskExitStatus) -> ! {
        let _ = KernHandle::kern_call_2(KernFnPath::Task(KernTaskFnId::Exit),
                                        task_type.into(),
                                        exit_status.as_syscall_ptr());
        unreachable!()
    }

    /**
     * Returns the `TaskId` of this `TaskHandle`
     */
    pub fn os_id(&self) -> Result<TaskId> {
        self.m_handle
            .inst_kern_call_0(KernFnPath::Task(KernTaskFnId::OsId))
            .map(|task_id| task_id as TaskId)
    }

    /**
     * Terminates this `TaskHandle`
     */
    fn kill(&self, allow_cleanup: bool) -> Result<()> {
        self.m_handle
            .inst_kern_call_1(KernFnPath::Task(KernTaskFnId::Kill),
                              allow_cleanup as usize)
            .map(|_| ())
    }

    /**  
     * Yields the execution
     */
    fn yield_next(&self) {
        self.m_handle
            .inst_kern_call_0(KernFnPath::Task(KernTaskFnId::Yield))
            .expect("Failed to yield to next task");
    }

    /**
     * Returns whether the referenced task is valid and alive
     */
    fn is_alive(&self) -> bool {
        self.m_handle
            .inst_kern_call_0(KernFnPath::Task(KernTaskFnId::IsAlive))
            .map(|res| res != 0)
            .unwrap_or(false)
    }

    /**
     * Returns the reference to the `KernHandle`
     */
    pub fn kern_handle(&self) -> &KernHandle {
        &self.m_handle
    }
}

/**
 * Common interface implemented by all the `TaskHandle` based objects.
 *
 * It mainly exposes the private methods of the `TaskHandle` for safe
 * calling and provides convenient methods to easily perform works that
 * normally implies more than one call
 */
pub trait Task: From<TaskHandle> {
    /**
     * The value of the `TaskType` that matches the implementation
     */
    const TASK_TYPE: TaskType;

    /**
     * Returns the immutable reference to the underling `TaskHandle`
     * instance
     */
    fn task_handle(&self) -> &TaskHandle;

    /**
     * Returns the mutable reference to the underling `TaskHandle` instance
     */
    fn task_handle_mut(&mut self) -> &mut TaskHandle;

    /**
     * Returns a `TaskConfig` for `Task` spawn
     */
    fn spawn() -> TaskConfig<Self, CreatMode> {
        TaskConfig::<Self, CreatMode>::new()
    }

    /**
     * Returns a `TaskConfig` for `Task` opening
     */
    fn open() -> TaskConfig<Self, OpenMode> {
        TaskConfig::<Self, OpenMode>::new()
    }

    /**
     * Obtains the current `Task` reference according to the wrapping type
     * (i.e `Proc` or `Thread`)
     */
    fn this() -> Self {
        Self::from(TaskHandle::this(Self::TASK_TYPE))
    }

    /**
     * Exists this `Task` with the given `TaskExitStatus`
     */
    fn exit(exit_status: TaskExitStatus) -> ! {
        TaskHandle::exit(Self::TASK_TYPE, exit_status)
    }

    /**
     * Returns the `TaskId` of this `Task`
     */
    fn os_id(&self) -> Result<TaskId> {
        self.os_id()
    }

    /**
     * Kills this `Task` if the caller have enough permissions to do that.
     *
     * When `allow_cleanup` is `true` the `Task`, before exit, calls the
     * registered cleanup-routines
     */
    fn kill(&self, allow_cleanup: bool) -> Result<()> {
        self.task_handle().kill(allow_cleanup)
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
