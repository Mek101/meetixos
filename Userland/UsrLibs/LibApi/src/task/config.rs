/*! `Task` configuration */

use core::marker::PhantomData;

use api_data::task::{
    config::{
        RawTaskConfig,
        TaskConfigBits
    },
    modes::TaskExecCpu,
    TaskId
};

use crate::{
    config::{
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
    handle::{
        KernHandle,
        Result
    },
    task::{
        Task,
        TaskHandle
    }
};
use api_data::sys::{
    codes::KernTaskConfigFnId,
    fn_path::KernFnPath
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

impl<'a, T, M> TaskConfig<'a, T, M>
    where T: Task,
          M: ConfigMode
{
    pub(crate) fn new() -> Self {
        Self { m_raw_config: RawTaskConfig::new(T::TASK_TYPE),
               _unused: PhantomData }
    }

    pub fn with_id(&mut self, task_id: TaskId) -> &mut Self {
        self.m_raw_config.set_id(task_id);
        self
    }

    fn apply(&self) -> Result<T> {
        KernHandle::kern_call_1(KernFnPath::TaskConfig(KernTaskConfigFnId::ApplyConfig),
                                self.m_raw_config.as_syscall_ptr())
                   .map(|raw_task_handle| T::from(TaskHandle::from_raw(raw_task_handle)))
    }
}

impl<'a, T> TaskConfig<'a, T, OpenMode> where T: Task {
    pub fn find(&self) -> Result<T> {
        self.apply()
    }
}

impl<'a, T> TaskConfig<'a, T, CreatMode> where T: Task {
    pub fn with_coop_scheduler(&mut self) -> &mut Self {
        self.m_raw_config.flags_mut().set_enabled(TaskConfigBits::CoopSched);
        self
    }

    pub fn with_high_prio(&mut self) -> &mut Self {
        assert!(self.m_raw_config.flags().is_disabled(TaskConfigBits::LowPrioTask));

        self.m_raw_config.flags_mut().set_enabled(TaskConfigBits::HighPrioTask);
        self
    }

    pub fn with_low_prio(&mut self) -> &mut Self {
        assert!(self.m_raw_config.flags().is_disabled(TaskConfigBits::HighPrioTask));

        self.m_raw_config.flags_mut().set_enabled(TaskConfigBits::LowPrioTask);
        self
    }

    pub fn with_exec_cpu(&mut self, exec_cpu: TaskExecCpu) -> &mut Self {
        self.m_raw_config.set_exec_cpu(exec_cpu);
        self
    }
}

impl<'a> TaskConfig<'a, Proc, CreatMode> {
    pub fn with_os_user(&mut self, os_user: &OsUser) -> &mut Self {
        self.m_raw_config
            .set_os_user(os_user.os_entity_handle().kern_handle().raw_handle());
        self
    }

    pub fn with_os_group(&mut self, os_group: &OsGroup) -> &mut Self {
        self.m_raw_config
            .set_os_user(os_group.os_entity_handle().kern_handle().raw_handle());
        self
    }
}

impl<'a> TaskConfig<'a, Thread, CreatMode> {
}
