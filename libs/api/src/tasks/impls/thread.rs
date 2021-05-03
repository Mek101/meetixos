/*! # Thread Management
 *
 * Implements the running thread reference
 */

use os::sysc::{
    codes::KernThreadFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::task::{
        TaskType,
        ThreadEntryData,
        WaitFor
    },
    caller::{
        KernCaller,
        Result
    },
    tasks::{
        Task,
        TaskId
    },
    time::Duration
};

impl_task_id_task! {
    /** # Running `Thread`
     *
     * Represents a reference to an execution flow inside a running
     * [`Proc`]ess.
     *
     * This represents the execution entity on which the kernel's scheduler
     * operates
     *
     * [`Proc`]: crate::tasks::impls::proc::Proc
     */
    pub struct Thread(TaskType::Thread);
}

impl Thread {
    /** # Sleeps for a time quantum
     *
     * Puts the `Thread` into the wait state for the given non-zero
     * [`Duration`].
     *
     * It is possible to call this method for another `Thread` of the same
     * [`Proc`] that executes this; in that case the referenced `Thread`
     * will stop for the given [`Duration`].
     *
     * [`Duration`]: crate::time::Duration
     * [`Proc`]: crate::tasks::impls::proc::Proc
     * [`Thread::this()`]: crate::tasks::task::Task::this
     */
    pub fn sleep(&self, duration: Duration) -> Result<()> {
        self.wait_for(WaitFor::Quantum(duration))
    }

    /** # Join a `Thread` execution
     *
     * Puts this `Thread` into the wait state until the given `target`
     * doesn't terminate.
     *
     * It is denied to call this method with a `Thread` reference that is
     * not the one returned by [`Thread::this()`] and obviously the
     * `target Thread` must not be same as returned by
     * [`Thread::this()`]
     *
     * [`Thread::this()`]: crate::tasks::task::Task::this
     */
    pub fn join(&self, target: Thread) -> Result<()> {
        self.wait_for(WaitFor::Join(target))
    }

    /** # Wait an IRQ throw
     *
     * Puts this `Thread` into the wait state until the given `irq` doesn't
     * throw.
     *
     * It is denied to call this method with a `Thread` reference that is
     * not the one returned by [`Thread::this()`]
     *
     * [`Thread::this()`]: crate::tasks::task::Task::this
     */
    pub fn wait_irq(&self, irq: u32) -> Result<()> {
        self.wait_for(WaitFor::Irq(irq))
    }

    /** # Wait for an event
     *
     * Puts the `Thread` into the waiting state for an amount of time
     * according to the [`WaitFor`] mode given
     *
     * [`WaitFor`]: crate::bits::task::modes::WaitFor
     */
    pub fn wait_for(&self, wait_reason: WaitFor) -> Result<()> {
        self.kern_call_1(KernFnPath::Thread(KernThreadFnId::WaitFor),
                         &wait_reason as *const _ as usize)
            .map(|_| ())
    }

    /** # Prepends a cleanup callback
     *
     * The given callback(s) is/are executed using a [`LIFO`] queue.
     *
     * They are executed when the [`Thread`] terminates (naturally or
     * because explicitly called [`Thread::terminate(true)`]).
     *
     * It is possible to register a cleanup callback to for a `Thread` that
     * is different from [`Thread::this()`], but the caller
     * [`OSUser`]/[`OSGroup`] must be the same or the administrator.
     *
     * [`LIFO`]: https://en.wikipedia.org/wiki/Stack_(abstract_data_type)
     * [`Thread`]: crate::tasks::impls::thread::Thread
     * [`Thread::terminate(true)`]: crate::tasks::task::Task::terminate
     * [`Thread::this()`]: crate::tasks::task::Task::this
     * [`OSUser`]: crate::ents::impls::user::OSUser
     * [`OSGroup`]: crate::ents::impls::group::OSGroup
     */
    pub fn add_cleaner(&self, cleanup_fn: fn()) -> Result<()> {
        let thread_entry_data = ThreadEntryData::new_cleaner_callback(cleanup_fn);
        self.kern_call_1(KernFnPath::Thread(KernThreadFnId::AddCleaner),
                         &thread_entry_data as *const _ as usize)
            .map(|_| ())
    }

    /** # Callback exit point
     *
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

    /** # Thread entry point data
     *
     * Returns the right [`ThreadEntryData`] variant for the situation that
     * calls this
     *
     * [`ThreadEntryData`]: crate::bits::task::data::thread::ThreadEntryData
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
