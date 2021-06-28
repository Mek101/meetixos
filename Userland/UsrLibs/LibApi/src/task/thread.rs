/*! Thread `Task` reference */
use core::time::Duration;

use api_data::{
    sys::{
        codes::KernThreadFnId,
        fn_path::KernFnPath
    },
    task::{
        modes::WaitReason,
        thread::{
            RCleanerThreadEntry,
            ThreadEntryData
        },
        types::TaskType
    }
};

use crate::{
    handle::{
        KernHandle,
        Result
    },
    task::{
        Task,
        TaskHandle
    }
};

#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct Thread {
    m_task_handle: TaskHandle
}

impl Thread {
    pub fn join(&self) -> Result<usize> {
        Self::wait_for(WaitReason::Join(self.task_handle().kern_handle().raw_handle()))
    }

    pub fn pause(&self) -> Result<()> {
        Self::wait_for(WaitReason::Pause(self.task_handle().kern_handle().raw_handle()))
             .map(|_| ())
    }

    pub fn sleep(duration: Duration) -> Result<usize> {
        Self::wait_for(WaitReason::Quantum(duration))
    }

    pub fn wait_irq(irq_number: u32) -> Result<()> {
        Self::wait_for(WaitReason::Irq(irq_number)).map(|_| ())
    }

    pub fn add_cleaner(cleanup_fn: RCleanerThreadEntry) -> Result<()> {
        Self::this().task_handle()
                    .kern_handle()
                    .inst_kern_call_1(KernFnPath::Thread(KernThreadFnId::AddCleaner),
                                      cleanup_fn as *const _ as usize)
                    .map(|_| ())
    }

    pub fn resume(&self) -> Result<Duration> {
        let mut pause_duration = Duration::default();

        self.task_handle()
            .kern_handle()
            .inst_kern_call_1(KernFnPath::Thread(KernThreadFnId::Resume),
                              &mut pause_duration as *mut _ as usize)
            .map(|_| pause_duration)
    }

    pub(crate) fn entry_point_data() -> ThreadEntryData {
        let mut thread_entry_data = ThreadEntryData::default();
        KernHandle::kern_call_1(KernFnPath::Thread(KernThreadFnId::GetEntryData),
                                &mut thread_entry_data as *mut _ as usize)
                   .map(|_| thread_entry_data)
                   .expect("Failed to obtain ThreadEntryData")
    }

    fn callback_return(callback_return: Option<bool>) -> ! {
        let _ =
            KernHandle::kern_call_1(KernFnPath::Thread(KernThreadFnId::CallbackReturn),
                                    &callback_return as *const _ as usize);
        unreachable!()
    }

    fn wait_for(wait_reason: WaitReason) -> Result<usize> {
        KernHandle::kern_call_1(KernFnPath::Thread(KernThreadFnId::WaitFor),
                                &wait_reason as *const _ as usize)
    }
}

impl From<TaskHandle> for Thread {
    fn from(task_handle: TaskHandle) -> Self {
        Self { m_task_handle: task_handle }
    }
}

impl Task for Thread {
    const TASK_TYPE: TaskType = TaskType::Thread;

    fn task_handle(&self) -> &TaskHandle {
        &self.m_task_handle
    }

    fn task_handle_mut(&mut self) -> &mut TaskHandle {
        &mut self.m_task_handle
    }
}

#[inline(never)]
pub(crate) extern "C" fn c_thread_entry() -> ! {
    match Thread::entry_point_data() {
        ThreadEntryData::User { m_entry_point,
                                m_entry_arg,
                                m_thread_id } => {
            let exit_status = m_entry_point(m_entry_arg, m_thread_id);
            Thread::exit(exit_status);
        },

        ThreadEntryData::WatchCallback { m_entry_point,
                                         m_entry_arg,
                                         m_thread_id } => {
            let keep_callback_registered = m_entry_point(m_entry_arg, m_thread_id);
            Thread::callback_return(Some(keep_callback_registered));
        },
        ThreadEntryData::CleanerCallback { m_entry_point,
                                           m_thread_id } => {
            m_entry_point(m_thread_id);
            Thread::callback_return(None);
        },
        _ => unreachable!()
    }

    unreachable!()
}
