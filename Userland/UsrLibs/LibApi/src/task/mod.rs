/*! Tasks management */

use api_data::{
    sys::{
        codes::KernTaskFnId,
        fn_path::KernFnPath,
        RawKernHandle
    },
    task::{
        types::TaskType,
        RawTaskHandle
    }
};

use crate::caller::{
    KernCaller,
    Result
};

#[repr(transparent)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
pub struct TaskHandle {
    m_raw_handle: RawTaskHandle
}

impl TaskHandle {
    /**
     * Obtains the current `TaskHandle` of the `TaskType` given
     */
    fn this(task_type: TaskType) -> Self {
        Self::kern_call_1(KernFnPath::Task(KernTaskFnId::This), task_type.into())
             .map(|raw_task_handle| {
                 Self { m_raw_handle: raw_task_handle as RawTaskHandle }
             })
             .expect("Failed to obtain current task handle")
    }

    /**
     * Terminates this `TaskId`
     */
    fn terminate(&self, call_cleanup: bool) -> Result<()> {
        self.inst_kern_call_1(KernFnPath::Task(KernTaskFnId::Terminate),
                              call_cleanup as usize)
            .map(|_| ())
    }

    /**  
     * Yields the execution
     */
    fn yield_next(&self) {
        self.inst_kern_call_0(KernFnPath::Task(KernTaskFnId::Yield))
            .expect("Failed to yield to next task");
    }

    /**
     * Returns whether the referenced task is alive
     */
    fn is_alive(&self) -> bool {
        self.m_raw_handle != 0
        && self.inst_kern_call_0(KernFnPath::Task(KernTaskFnId::IsAlive))
               .map(|res| res != 0)
               .expect("Failed to obtain task handle validity")
    }
}

impl KernCaller for TaskHandle {
    fn raw_handle(&self) -> RawKernHandle {
        self.m_raw_handle
    }
}
