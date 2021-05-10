/*! `Task` configuration */

use core::marker::PhantomData;

use os::sysc::{
    codes::KernTaskConfigFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::task::{
        data::spec::TaskSpecData,
        modes::{
            SchedPolicy,
            TaskCpu,
            TaskPrio
        }
    },
    caller::{
        KernCaller,
        Result
    },
    config::{
        ConfigFinderIter,
        ConfigMode,
        CreatMode,
        FindMode
    },
    ents::impls::{
        group::OSGroup,
        user::OSUser
    },
    objs::impls::file::File,
    tasks::{
        impls::{
            proc::Proc,
            thread::Thread
        },
        task::{
            Task,
            TaskId
        }
    }
};

/**
 * Common functional configuration interface to find or create `Task`
 * based objects
 */
#[derive(Debug)]
pub struct TaskConfig<T, M>
    where T: Task,
          M: ConfigMode {
    m_id: Option<u32>,
    m_children_of: Option<T>,
    m_sched_policy: SchedPolicy,
    m_prio: TaskPrio,
    m_cpu: TaskCpu,
    m_spec: TaskSpecData,
    m_os_user: Option<OSUser>,
    m_os_group: Option<OSGroup>,
    _unused: PhantomData<T>,
    _unused2: PhantomData<M>
}

impl<T, M> TaskConfig<T, M>
    where T: Task,
          M: ConfigMode
{
    /**
     * Constructs an empty `TaskConfig`
     */
    pub(crate) fn new() -> Self {
        Self { m_id: None,
               m_children_of: None,
               m_sched_policy: SchedPolicy::Preemptive,
               m_prio: TaskPrio::Normal,
               m_cpu: TaskCpu::Any,
               m_spec: TaskSpecData::None,
               m_os_user: None,
               m_os_group: None,
               _unused: Default::default(),
               _unused2: Default::default() }
    }
}

#[cfg(feature = "enable_kernel_methods")]
impl<T: Task, M: ConfigMode> TaskConfig<T, M> {
    /**
     * Returns the optional id for searching
     */
    pub fn id(&self) -> Option<u32> {
        self.m_id
    }

    /**
     * Returns the `Task` to use as parent for searching
     */
    pub fn children_parent(&self) -> Option<&T> {
        self.m_children_of.as_ref()
    }

    /**
     * Returns the `SchedPolicy` chosen for the new `Task`
     */
    pub fn sched_policy(&self) -> SchedPolicy {
        self.m_sched_policy
    }

    /**
     * Returns the `TaskCpu` chosen for the new `Task`
     */
    pub fn task_cpu(&self) -> TaskCpu {
        self.m_cpu
    }

    /**
     * Returns the reference to the `TaskSpecData`
     */
    pub fn task_spec_data(&self) -> &TaskSpecData {
        &self.m_spec
    }

    /**
     * Returns the optional `OSUser` chosen
     */
    pub fn os_user(&self) -> Option<OSUser> {
        self.m_os_user
    }

    /**
     * Returns the optional `OSGroup` chosen
     */
    pub fn os_group(&self) -> Option<OSGroup> {
        self.m_os_group
    }
}

impl<T> TaskConfig<T, CreatMode> where T: Task {
    /**
     * The variant of `SchedPolicy` given tells to the kernel which
     * scheduling algorithm must be used for the new `Task`
     */
    pub fn with_sched_policy(&mut self, sched_policy: SchedPolicy) -> &mut Self {
        self.m_sched_policy = sched_policy;
        self
    }

    /**
     * The variant of `TaskPrio` given tells to the kernel which is the
     * class of priority that the new `Task` must have for all his
     * execution life
     */
    pub fn with_prio(&mut self, priority: TaskPrio) -> &mut Self {
        self.m_prio = priority;
        self
    }

    /**
     * The variant of `TaskCpu` given tells to the kernel whether and how
     * the new `Task` must be affine to a subset of the available CPUs
     * in a SMP environment
     */
    pub fn with_cpu(&mut self, cpu: TaskCpu) -> &mut Self {
        self.m_cpu = cpu;
        self
    }

    /**
     * Requests to the kernel to apply the given configuration to spawn a
     * new `Task`
     */
    fn run_task(self) -> Result<T> {
        self.kern_call_1(KernFnPath::TaskConfig(KernTaskConfigFnId::CreateTask),
                         &self as *const _ as usize)
            .map(|task_id| T::from(TaskId::from(task_id)))
    }
}

impl<T> TaskConfig<T, FindMode> where T: Task {
    /**
     * Tells to the kernel exactly which task is requested.
     *
     * Any other filter is ignored when this one is enabled
     */
    pub fn with_id(&mut self, id: u32) -> &mut Self {
        self.m_id = Some(id);
        self
    }

    /**
     * Enables the parent `Task` filter to tell the kernel on which
     * children iterate
     */
    pub fn children_of(&mut self, task: T) -> &mut Self {
        self.m_children_of = Some(task);
        self
    }

    /**
     * Enables the owner `OSUser` filter to tell the kernel which
     * `Task`s select
     */
    pub fn owned_by_user(&mut self, user: OSUser) -> &mut Self {
        self.m_os_user = Some(user);
        self
    }

    /**
     * Enables the owner `OSGroup` filter to tell the kernel which
     * `Task`s select
     */
    pub fn owned_by_group(&mut self, group: OSGroup) -> &mut Self {
        self.m_os_group = Some(group);
        self
    }

    /**
     * Dispatches the configuration to the kernel to validate and initialize
     * an iteration pool on which the returned `Iterator` will fetch
     * the results.
     *
     * If the given configuration have no filters, the kernel initializes an
     * iteration pool with **ALL** the active tasks of the `T` type
     * (`Proc` or `Thread`)
     */
    pub fn search(self) -> Result<impl Iterator<Item = T>> {
        self.kern_call_1(KernFnPath::TaskConfig(KernTaskConfigFnId::InitFind),
                         &self as *const _ as usize)
            .map(|iter_id| ConfigFinderIter::from(iter_id))
    }
}

impl TaskConfig<Proc, CreatMode> {
    /**
     * Overrides the owner `OSUser` which, otherwise, will be inherited by
     * the parent `Proc`
     */
    pub fn owned_by_user(&mut self, user: OSUser) -> &mut Self {
        self.m_os_user = Some(user);
        self
    }

    /**
     * Overrides the owner `OSGroup` which, otherwise, will be inherited
     * by the parent `Proc`
     */
    pub fn owned_by_group(&mut self, group: OSGroup) -> &mut Self {
        self.m_os_group = Some(group);
        self
    }

    /**
     * Dispatches this spawner configuration to the kernel that creates a
     * new `Proc`.
     *
     * The new `Proc` executes the given `File` with the given optional
     * arguments.
     *
     * The `File` must be a valid executable file format, and must be
     * opened with read/execute options enabled
     */
    pub fn run(mut self, file: File, args: Option<&[&str]>) -> Result<Proc> {
        self.m_spec = TaskSpecData::new_proc(file, args);
        self.run_task()
    }
}

impl TaskConfig<Proc, FindMode> {
    /**
     * Enables the executed `File` filter to tell the kernel on which
     * `Proc` iterate
     */
    pub fn executor_of(&mut self, file: File) -> &mut Self {
        self.m_spec = TaskSpecData::new_proc(file, None);
        self
    }
}

impl TaskConfig<Thread, CreatMode> {
    /**
     * Dispatches this spawner configuration to the kernel that creates a
     * new `Thread`.
     *
     * The new `Thread` starts his execution from the given `entry_point`
     * function, and receives the given `arg` as `entry_point`'s argument.
     *
     * The function's returns value is used as `Task::terminate()`'s
     * argument
     *
     * The newly created `Thread` shares the same address space of the
     * `Proc` that spawns it
     */
    pub fn run(mut self,
               entry_point: fn(usize) -> bool,
               arg: usize,
               name: Option<&str>)
               -> Result<Thread> {
        self.m_spec = TaskSpecData::new_thread(Some(entry_point), Some(arg), name);
        self.run_task()
    }
}

impl TaskConfig<Thread, FindMode> {
    /**
     * Enables the `Thread` name filter to tell the kernel on which name
     * iterate
     */
    pub fn with_name(&mut self, name: &str) -> &mut Self {
        self.m_spec = TaskSpecData::new_thread(None, None, Some(name));
        self
    }
}

impl<T, M> KernCaller for TaskConfig<T, M>
    where T: Task,
          M: ConfigMode
{
    /* Nothing to implement */
}
