/**! # `Task` Handle
 *
 * Implements the base struct and the trait used as base for the task
 * references
 */
use os::sysc::{
    codes::KernTaskFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::task::TaskType,
    caller::{
        KernCaller,
        Result
    },
    config::{
        CreatMode,
        FindMode
    },
    tasks::TaskConfig
};

/** # `Task` Handle
 *
 * Represents an opaque handle that takes place of the old style [`pid_t`]
 * and [`pthread_t`] types, used by all the Unix-like OS to identify
 * processes and threads.
 *
 * Itself the object doesn't have much utilities because most of his methods
 * are private, but exposed via the [`Task`] trait and implemented by the
 * various [`implementations`].
 *
 * Read more doc about [`Task`] and [`TaskId`] -> [here]
 *
 * [`pid_t`]: https://www.gnu.org/software/libc/manual/html_node/Process-Identification.html
 * [`pthread_t`]: https://www.man7.org/linux/man-pages/man3/pthread_self.3.html
 * [`Task`]: crate::tasks::Task
 * [`implementations`]: /api/tasks/impls/index.html
 * [`TaskId`]: crate::tasks::TaskId
 * [here]: /api/index.html#tasks
 */
#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct TaskId(u32);

impl TaskId {
    /** # Current `TaskId`
     *
     * Obtains the current `TaskId` of the [`TaskType`] given
     *
     * [`TaskType`]: crate::bits::task::types::TaskType
     */
    fn this(task_type: TaskType) -> Self {
        let mut task = Self::default();
        task.kern_call_1(KernFnPath::Task(KernTaskFnId::This), task_type.into())
            .map(|task_id| {
                task.0 = task_id as u32;
                task
            })
            .unwrap()
    }

    /** # Terminates this `TaskId`
     *
     * Terminates the task referenced by this `TaskId` if the caller
     * [`OSUser`]/[`OSGroup`] is the same of the `TaskId` to terminate or
     * it's the administrator.
     *
     * `call_cleanup` tells to the kernel whether it must call the
     * registered cleanup function (if any) before definitively
     * terminate the target
     *
     * [`OSUser`]: crate::ents::impls::OSUser
     * [`OSGroup`]: crate::ents::impls::OSGroup
     */
    fn terminate(&self, call_cleanup: bool) -> Result<()> {
        self.kern_call_1(KernFnPath::Task(KernTaskFnId::Terminate), call_cleanup as usize)
            .map(|_| ())
    }

    /** # Yields the execution
     *
     * Preempts the remaining CPU quantum of this `TaskId` if it was spawner
     * with [`SchedPolicy::Preemptive`][SP], or to release the CPU for the
     * other tasks if it was spawner with
     * [`SchedPolicy::Cooperative`][SC]
     *
     * [SP]: crate::bits::task::modes::SchedPolicy::Preemptive
     * [SC]: crate::bits::task::modes::SchedPolicy::Cooperative
     */
    fn yield_next(&self) {
        self.kern_call_0(KernFnPath::Task(KernTaskFnId::Yield)).unwrap();
    }

    /** Returns whether the referenced task is alive
     */
    fn is_alive(&self) -> bool {
        self.0 != 0
        && self.kern_call_0(KernFnPath::Task(KernTaskFnId::IsAlive))
               .map(|res| res != 0)
               .unwrap_or(false)
    }

    /** Returns the raw identifier of this [`TaskId`]
     *
     * [`TaskId`]: crate::tasks::TaskId
     */
    pub fn id(&self) -> u32 {
        self.0
    }

    /** Returns the raw identifier of this [`TaskId`] as `usize`
     *
     * [`TaskId`]: crate::tasks::TaskId
     */
    pub fn id_usize(&self) -> usize {
        self.id() as usize
    }
}

impl From<u32> for TaskId {
    /** Performs the conversion
     */
    fn from(raw_id: u32) -> Self {
        Self(raw_id)
    }
}

impl From<usize> for TaskId {
    /** Performs the conversion
     */
    fn from(raw_id: usize) -> Self {
        Self::from(raw_id as u32)
    }
}

impl KernCaller for TaskId {
    /** Returns the raw identifier of the object
     */
    fn caller_handle_bits(&self) -> u32 {
        self.id()
    }
}

/** # `Task` Base Interface
 *
 * Defines a common interface implemented by all the [`TaskId`] based
 * objects.
 *
 * It mainly exposes the private methods of the [`TaskId`] for safe calling
 * and provides convenient methods to easily perform works that normally
 * implies more than one call
 *
 * [`TaskId`]: crate::tasks::TaskId
 */
pub trait Task: From<TaskId> + Sync + Send {
    /** The value of the [`TaskType`] that matches the implementation
     *
     * [`TaskType`]: crate::bits::task::types::TaskType
     */
    const TASK_TYPE: TaskType;

    /** Returns the immutable reference to the underling [`TaskId`] instance
     *
     * [`TaskId`]: crate::tasks::TaskId
     */
    fn task_handle(&self) -> &TaskId;

    /** Returns the mutable reference to the underling [`TaskId`] instance
     *
     * [`TaskId`]: crate::tasks::TaskId
     */
    fn task_handle_mut(&mut self) -> &mut TaskId;

    /** Returns an uninitialized [`TaskConfig`] to spawn a new [`Task`]
     *
     * [`TaskConfig`]: crate::tasks::TaskConfig
     * [`Task`]: crate::tasks::Task
     */
    fn spawn() -> TaskConfig<Self, CreatMode> {
        TaskConfig::new()
    }

    /** Returns an uninitialized [`TaskConfig`] to find an existing [`Task`]
     *
     * [`TaskConfig`]: crate::tasks::TaskConfig
     * [`Task`]: crate::tasks::Task
     */
    fn find() -> TaskConfig<Self, FindMode> {
        TaskConfig::new()
    }

    /** # Current `Task`
     *
     * Obtains the current [`Task`] reference according to the wrapping type
     * (i.e [`Proc`] or [`Thread`])
     *
     * [`Task`]: crate::tasks::Task
     * [`Proc`]: crate::tasks::impls::Proc
     * [`Thread`]: crate::tasks::impls::Thread
     */
    fn this() -> Self {
        Self::from(TaskId::this(Self::TASK_TYPE))
    }

    /**
     * * Returns the numeric identifier of this [`Task`]
     *
     * [`Task`]: crate::tasks::Task
     */
    fn id(&self) -> u32 {
        self.task_handle().id()
    }

    /** # Terminates this `Task`
     *
     * Terminates the task referenced by this [`Task`] if the caller
     * user/group is the same of the [`Task`] to terminate or it's the
     * administrator.
     *
     * The `call_cleanup` flag tells to the kernel whether it must call the
     * registered cleanup function(s) (if any) before definitively terminate
     * the target.
     *
     * If the target is a [`Proc`] and it have more than one active
     * [`Thread`] the kernel first terminates them all, then terminates
     * the [`Proc`]
     *
     * [`Task`]: crate::tasks::Task
     * [`Proc`]: crate::tasks::impls::Proc
     * [`Thread`]: crate::tasks::impls::Thread
     */
    fn terminate(&self, call_cleanup: bool) -> Result<()> {
        self.task_handle().terminate(call_cleanup)
    }

    /** # Yields the execution
     *
     * Preempts the remaining CPU quantum of this [`Task`] if it was spawned
     * with [`SchedPolicy::Preemptive`][SP], or to release the CPU for the
     * other tasks if it was spawner with
     * [`SchedPolicy::Cooperative`][SC]
     *
     * [`Task`]: crate::tasks::Task
     * [SP]: crate::bits::task::modes::SchedPolicy::Preemptive
     * [SC]: crate::bits::task::modes::SchedPolicy::Cooperative
     */
    fn yield_next(&self) {
        self.task_handle().yield_next()
    }

    /** Returns whether the referenced [`Task`] is alive
     *
     * [`Task`]: crate::tasks::Task
     */
    fn is_alive(&self) -> bool {
        self.task_handle().is_alive()
    }
}
