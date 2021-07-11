/*! `Task` configuration */

use core::marker::PhantomData;

use api_data::{
    sys::{
        codes::KernTaskConfigFnId,
        fn_path::KernFnPath,
        AsSysCallPtr
    },
    task::{
        config::{
            RawTaskConfig,
            TaskConfigBits
        },
        modes::TaskExecCpu,
        thread::{
            RUserThreadEntry,
            UserThreadArg
        },
        TaskId
    }
};

use crate::{
    config_mode::{
        ConfigMode,
        CreatMode,
        OpenMode
    },
    entity::{
        impls::{
            group::OsGroup,
            user::OsUser
        },
        OsEntity
    },
    kern_handle::{
        KernHandle,
        Result
    },
    object::{
        impls::file::File,
        Object
    },
    task::{
        impls::{
            proc::Proc,
            thread::{
                c_thread_entry,
                Thread
            }
        },
        Task,
        TaskHandle
    }
};

/**
 * High level type-safe `Task` configuration
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct TaskConfig<'a, T, M>
    where T: Task,
          M: ConfigMode {
    m_raw_config: RawTaskConfig<'a>,
    _unused: PhantomData<(T, M)>
}

impl<'a, T> TaskConfig<'a, T, CreatMode> where T: Task /* Constructors */ {
    /**
     * Constructs `TaskConfig` for `Task` spawning
     */
    pub(crate) fn new() -> Self {
        Self { m_raw_config: RawTaskConfig::new(T::TASK_TYPE, true),
               _unused: PhantomData }
    }
}

impl<'a, T> TaskConfig<'a, T, OpenMode> where T: Task /* Constructors */ {
    /**
     * Constructs a `TaskConfig` for `Task` opening
     */
    pub(crate) fn new() -> Self {
        Self { m_raw_config: RawTaskConfig::new(T::TASK_TYPE, false),
               _unused: PhantomData }
    }
}

impl<'a, T> TaskConfig<'a, T, OpenMode> where T: Task /* Methods */ {
    /**
     * Dispatches the configuration to the kernel to find the desired `Task`
     */
    pub fn find(&self) -> Result<T> {
        self.dispatch_config()
    }
}

impl<'a> TaskConfig<'a, Proc, CreatMode> /* Methods */ {
    /**
     * Spawns a new `Proc` from the given executable `File`
     */
    pub fn run(&mut self,
               file_to_exec: File,
               cmdline_args: &'a [&'a str])
               -> Result<Proc> {
        self.m_raw_config
            .set_file_to_exec(file_to_exec.obj_handle().kern_handle().raw_handle());
        self.m_raw_config.set_cmdline_args(cmdline_args);

        self.dispatch_config()
    }
}

impl<'a> TaskConfig<'a, Thread, CreatMode> /* Methods */ {
    /**
     * Spawns a new `Thread` from the given entry-point
     */
    pub fn run(&mut self,
               entry_point: RUserThreadEntry,
               thread_arg: UserThreadArg)
               -> Result<Thread> {
        self.m_raw_config.set_c_thread_entry(c_thread_entry);
        self.m_raw_config.set_thread_entry(entry_point);
        self.m_raw_config.set_thread_arg(thread_arg);

        self.dispatch_config()
    }
}

impl<'a, T, M> TaskConfig<'a, T, M>
    where T: Task,
          M: ConfigMode /* Setters */
{
    /**
     * Sets the explicit `TaskId` for creation or for opening
     */
    pub fn with_id(&mut self, task_id: TaskId) -> &mut Self {
        self.m_raw_config.set_id(task_id);
        self
    }
}

impl<'a, T> TaskConfig<'a, T, CreatMode> where T: Task /* Setters */ {
    /**
     * Enables cooperative scheduling for the new `Task`
     */
    pub fn with_coop_scheduler(&mut self) -> &mut Self {
        self.m_raw_config.flags_mut().set_enabled(TaskConfigBits::CoopSched);
        self
    }

    /**
     * Forces the kernel to choose higher priority classes for the new
     * `Task`
     */
    pub fn with_high_prio(&mut self) -> &mut Self {
        assert!(self.m_raw_config.flags().is_disabled(TaskConfigBits::LowPrioTask));

        self.m_raw_config.flags_mut().set_enabled(TaskConfigBits::HighPrioTask);
        self
    }

    /**
     * Forces the kernel to choose lower priority classes for the new `Task`
     */
    pub fn with_low_prio(&mut self) -> &mut Self {
        assert!(self.m_raw_config.flags().is_disabled(TaskConfigBits::HighPrioTask));

        self.m_raw_config.flags_mut().set_enabled(TaskConfigBits::LowPrioTask);
        self
    }

    /**
     * Forces the kernel to not start as soon-as-possible the new `Task`
     */
    pub fn start_paused(&mut self) -> &mut Self {
        self.m_raw_config.flags_mut().set_enabled(TaskConfigBits::StartPaused);
        self
    }

    /**
     * Sets the `TaskExecCpu` filter for the new `Task`
     */
    pub fn with_exec_cpu(&mut self, exec_cpu: TaskExecCpu) -> &mut Self {
        self.m_raw_config.set_exec_cpu(exec_cpu);
        self
    }
}

impl<'a> TaskConfig<'a, Proc, CreatMode> /* Setters */ {
    /**
     * Sets the `OsUser` which will own the new `Proc`
     */
    pub fn with_os_user(&mut self, os_user: &OsUser) -> &mut Self {
        self.m_raw_config
            .set_os_user(os_user.os_entity_handle().kern_handle().raw_handle());
        self
    }

    /**
     * Sets the `OsGroup` which will own the new `Proc`
     */
    pub fn with_os_group(&mut self, os_group: &OsGroup) -> &mut Self {
        self.m_raw_config
            .set_os_user(os_group.os_entity_handle().kern_handle().raw_handle());
        self
    }
}

impl<'a, M> TaskConfig<'a, Thread, M> where M: ConfigMode /* Setters */ {
    /**
     * Sets the name to the new thread or for searching
     */
    pub fn with_name(&mut self, thread_name: &'a str) -> &mut Self {
        self.m_raw_config.set_thread_name(thread_name);
        self
    }
}

impl<'a, T, M> TaskConfig<'a, T, M>
    where T: Task,
          M: ConfigMode /* Privates */
{
    /**
     * Dispatches the configuration to the kernel
     */
    fn dispatch_config(&self) -> Result<T> {
        KernHandle::kern_call_1(KernFnPath::TaskConfig(KernTaskConfigFnId::ApplyConfig),
                                self.m_raw_config.as_syscall_ptr())
            .map(|raw_task_handle| T::from(TaskHandle::from_raw(raw_task_handle)))
    }
}
