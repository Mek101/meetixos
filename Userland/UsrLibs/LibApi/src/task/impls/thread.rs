/*! Thread `Task` reference */

use core::time::Duration;

use api_data::{
    sys::{
        codes::KernThreadFnId,
        fn_path::KernFnPath,
        AsSysCallPtr
    },
    task::{
        exit::TaskExitStatus,
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

/**
 * Reference to an execution flow inside a running `Proc`
 */
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
    /**
     * Puts the caller `Thead` in wait-state until this `Thread` doesn't
     * terminate.
     *
     * When terminates the `TaskExitStatus` is returned
     */
    pub fn join(&self) -> Result<TaskExitStatus> {
        let mut task_exit_status = TaskExitStatus::default();

        self.task_handle()
            .kern_handle()
            .inst_kern_call_1(KernFnPath::Thread(KernThreadFnId::Join),
                              task_exit_status.as_syscall_ptr_mut())
            .map(|_| task_exit_status)
    }

    /**
     * Pauses this `Thread` until is called `Thread::resume()`
     */
    pub fn pause(&self) -> Result<()> {
        self.task_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::Thread(KernThreadFnId::Join))
            .map(|_| ())
    }

    /**
     * Puts the caller `Thread` in a sleep-state for the given `Duration` of
     * time.
     *
     * Returns the un-slept time `Duration`
     */
    pub fn sleep(duration: Duration) -> Result<usize> {
        let mut unslept_duration = Duration::default();

        KernHandle::kern_call_2(KernFnPath::Thread(KernThreadFnId::Sleep),
                                &duration as *const _ as usize,
                                &mut unslept_duration as *mut _ as usize)
    }

    /**
     * Puts the caller `Thread` in a wait-state until the given IRQ doesn't
     * throws
     */
    pub fn wait_irq(irq_number: u32) -> Result<()> {
        KernHandle::kern_call_1(KernFnPath::Thread(KernThreadFnId::WaitIrq),
                                irq_number as usize).map(|_| ())
    }

    /**
     * Appends a cleanup function which will be executed at the `Thread`'s
     * exit
     */
    pub fn add_cleaner(cleanup_fn: RCleanerThreadEntry) -> Result<()> {
        KernHandle::kern_call_2(KernFnPath::Thread(KernThreadFnId::AddCleaner),
                                cleanup_fn as usize,
                                c_thread_entry as usize).map(|_| ())
    }

    /**
     * Resumes this `Thread` and returns the `Duration` of his pause
     */
    pub fn resume(&self) -> Result<Duration> {
        let mut pause_duration = Duration::default();

        self.task_handle()
            .kern_handle()
            .inst_kern_call_1(KernFnPath::Thread(KernThreadFnId::Resume),
                              &mut pause_duration as *mut _ as usize)
            .map(|_| pause_duration)
    }

    /**
     * Returns the `ThreadEntryData`.
     *
     * Used by the `c_thread_entry()` routine when called by the kernel
     */
    pub(crate) fn entry_point_data() -> ThreadEntryData {
        let mut thread_entry_data = ThreadEntryData::default();

        KernHandle::kern_call_1(KernFnPath::Thread(KernThreadFnId::GetEntryData),
                                &mut thread_entry_data as *mut _ as usize)
                   .map(|_| thread_entry_data)
                   .expect("Failed to obtain ThreadEntryData")
    }

    /**
     * Restores the previous execution flow after a callback
     */
    fn callback_return(callback_return: Option<bool>) -> ! {
        let _ =
            KernHandle::kern_call_1(KernFnPath::Thread(KernThreadFnId::CallbackReturn),
                                    &callback_return as *const _ as usize);
        unreachable!()
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

/**
 * Entry-point internally passed to the kernel for user `Thread` and
 * callback execution
 */
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
}
