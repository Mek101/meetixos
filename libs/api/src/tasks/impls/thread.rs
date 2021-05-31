/*! Thread `Task` reference */

use os::sysc::{
    codes::KernThreadFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::task::{
        data::thread::ThreadEntryData,
        modes::WaitFor,
        types::TaskType
    },
    caller::{
        KernCaller,
        Result
    },
    tasks::task::{
        Task,
        TaskId
    },
    time::Duration
};

/**
 * Reference to an execution flow inside a running `Proc`.
 *
 * This represents the execution entity on which the kernel's scheduler
 * operates
 */
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Thread {
    m_handle: TaskId
}

impl Thread {
    /**
     * Puts the `Thread` into the wait state for the given non-zero
     * `Duration`.
     *
     * It is possible to call this method for another `Thread` of the same
     * `Proc` that executes this; in that case the referenced `Thread`
     * will stop for the given `Duration`
     */
    pub fn sleep(&self, duration: Duration) -> Result<()> {
        self.wait_for(WaitFor::Quantum(duration))
    }

    /**
     * Puts this `Thread` into the wait state until the given `target`
     * doesn't terminate.
     *
     * It is denied to call this method with a `Thread` reference that is
     * not the one returned by `Thread::this()` and obviously the
     * `target Thread` must not be same as returned by
     * `Thread::this()`
     */
    pub fn join(&self, target: Thread) -> Result<()> {
        self.wait_for(WaitFor::Join(target))
    }

    /**
     * Puts this `Thread` into the wait state until the given `irq` doesn't
     * throw.
     *
     * It is denied to call this method with a `Thread` reference that is
     * not the one returned by `Thread::this()`
     */
    pub fn wait_irq(&self, irq: u32) -> Result<()> {
        self.wait_for(WaitFor::Irq(irq))
    }

    /**
     * Puts the `Thread` into the waiting state for an amount of time
     * according to the `WaitFor` mode given
     */
    pub fn wait_for(&self, wait_reason: WaitFor) -> Result<()> {
        self.kern_call_1(KernFnPath::Thread(KernThreadFnId::WaitFor),
                         &wait_reason as *const _ as usize)
            .map(|_| ())
    }

    /**
     * Adds a cleanup callback that will be executed using a `LIFO` queue.
     *
     * They are executed when the `Thread` terminates (naturally or
     * because explicitly called `Thread::terminate(true)`).
     *
     * It is possible to register a cleanup callback to for a `Thread` that
     * is different from `Thread::this()`, but the caller
     * `OSUser`/`OSGroup` must be the same or the administrator
     */
    pub fn add_cleaner(&self, cleanup_fn: fn()) -> Result<()> {
        let thread_entry_data = ThreadEntryData::new_cleaner_callback(cleanup_fn);
        self.kern_call_1(KernFnPath::Thread(KernThreadFnId::AddCleaner),
                         &thread_entry_data as *const _ as usize)
            .map(|_| ())
    }

    /**
     * Called by the internal C binding of the cleaner entry point to
     * restore the previous situation
     */
    pub(crate) fn callback_return(&self, return_value: Option<usize>) {
        self.kern_call_2(KernFnPath::Thread(KernThreadFnId::CallbackReturn),
                         return_value.is_some() as usize,
                         return_value.unwrap_or(0))
            .map_err(|_| ())
            .map(|_| ())
            .unwrap()
    }

    /**
     * Returns the right `ThreadEntryData` variant for the situation that
     * calls this
     */
    pub(crate) fn get_entry_data(&self) -> ThreadEntryData {
        let mut entry_data = ThreadEntryData::default();

        /* the kernel know which variant of entry_data fill because of the
         * state of Proc which contains the current execution step
         * (i.e user execution/watch callback/cleanup callback), so no any further
         * arguments are needed for this system call
         */
        self.kern_call_1(KernFnPath::Thread(KernThreadFnId::GetEntryData),
                         &mut entry_data as *mut _ as usize)
            .map_err(|_| ())
            .map(|_| entry_data)
            .unwrap()
    }
}

impl Task for Thread {
    const TASK_TYPE: TaskType = TaskType::Thread;

    fn task_handle(&self) -> &TaskId {
        &self.m_handle
    }

    fn task_handle_mut(&mut self) -> &mut TaskId {
        &mut self.m_handle
    }
}

impl From<TaskId> for Thread {
    fn from(id: TaskId) -> Self {
        Self { m_handle: id }
    }
}

impl KernCaller for Thread {
    fn caller_handle_bits(&self) -> u32 {
        self.task_handle().caller_handle_bits()
    }
}
