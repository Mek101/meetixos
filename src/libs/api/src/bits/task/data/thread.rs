/*! # Thread Data
 *
 * Implements a [`Thread`] specific enumeration with the data used by the
 * various entry points
 *
 * [`Thread`]: crate::tasks::impls::thread::Thread
 */

use crate::{
    objs::infos::ObjUseInstant,
    tasks::{
        impls::Thread,
        Task
    }
};

/** # C `Thread` Entry Point
 *
 * Identifies the function prototype of the [`Thread`]'s C entry point
 * accepted by the kernel
 *
 * [`Thread`]: crate::tasks::impls::thread::Thread
 */
pub type CThreadEntry = extern "C" fn() -> !;

/** # Rust User `Thread` Entry Point
 *
 * Identifies the function prototype of the [`Thread`]'s rust entry point
 * accepted by the kernel for the user [`Thread`]s
 *
 * [`Thread`]: crate::tasks::impls::thread::Thread
 */
pub type RUserThreadEntry = fn(usize) -> bool;

/** # Rust Watch Callback `Thread` Entry Point
 *
 * Identifies the function prototype of the [`Thread`]'s rust entry point
 * accepted by the kernel for the [watch callback]
 *
 * [`Thread`]: crate::tasks::impls::thread::Thread
 * [watch callback]: crate::objs::object::Object::watch
 */
pub type RWatchCBThreadEntry = fn(ObjUseInstant) -> bool;

/** # Rust Cleaner Callback `Thread` Entry Point
 *
 * Identifies the function prototype of the [`Thread`]'s rust entry point
 * accepted by the kernel for the [cleaner callback]
 *
 * [`Thread`]: crate::tasks::impls::thread::Thread
 * [cleaner callback]: crate::tasks::impls::thread::Thread::add_cleaner
 */
pub type RCleanerCBThreadEntry = fn();

/** # Thread Entry Point Data
 *
 * Represents a variable entry point information used by the user API to
 * spawn new [`Thread`]s, register [watch] and [cleanup] callbacks.
 *
 * This enumeration is used too by the kernel to return back to the
 * userspace [`c_thread_entry`] the rust entry point and eventual
 * additional informations for the execution
 *
 * [`Thread`]: crate::tasks::impls::thread::Thread
 * [watch]: crate::objs::object::Object::watch
 * [cleanup]: crate::tasks::impls::thread::Thread::add_cleaner
 * [`c_thread_entry`]: crate::bits::task::data::c_thread_entry
 */
#[derive(Debug, Clone)]
pub enum ThreadEntryData {
    /** Contains the data to spawn/execute a new [`Thread`] using
     * [`Thread::spawn()`]
     *
     * [`Thread`]: crate::tasks::impls::thread::Thread
     * [`Thread::spawn()`]: crate::tasks::impls::thread::Thread::spawn
     */
    User {
        m_rust_entry_point: RUserThreadEntry,
        m_arg: usize,
        m_c_entry_point: Option<CThreadEntry>
    },

    /** Contains the data to register/execute a new [`Object::watch()`]
     * callback
     *
     * [`Object::watch()`]: crate::objs::object::Object::watch
     */
    WatchCallback {
        m_rust_entry_point: RWatchCBThreadEntry,
        m_use_instant: ObjUseInstant,
        m_c_entry_point: Option<CThreadEntry>
    },

    /** Contains the data to register/execute a new [`cleaner callback`]
     *
     * [cleaner callback]: crate::tasks::impls::thread::Thread::add_cleaner
     */
    CleanerCallback {
        m_rust_entry_point: RCleanerCBThreadEntry,
        m_c_entry_point: Option<CThreadEntry>
    },

    /** Default value, usable only for un-initialized `ThreadEntryData`
     * instances
     */
    None
}

impl ThreadEntryData {
    /** # Constructs a `ThreadEntryData::User`
     *
     * The returned instance is used by the user code and fills up the
     * `m_c_entry_point` field with the private C entry point function in
     * the module
     */
    pub fn new_user(entry_point: RUserThreadEntry, arg: usize) -> Self {
        Self::User { m_rust_entry_point: entry_point,
                     m_arg: arg,
                     m_c_entry_point: Some(c_thread_entry) }
    }

    /** # Constructs a `ThreadEntryData::WatchCallback`
     *
     * The returned instance is used by the user code and fills up the
     * `m_c_entry_point` field with the private C entry point function in
     * the module
     */
    pub fn new_watch_callback(entry_point: RWatchCBThreadEntry) -> Self {
        Self::WatchCallback { m_rust_entry_point: entry_point,
                              m_use_instant: ObjUseInstant::default(),
                              m_c_entry_point: Some(c_thread_entry) }
    }

    /** # Constructs a `ThreadEntryData::CleanerCallback`
     *
     * The returned instance is used by the user code and fills up the
     * `m_c_entry_point` field with the private C entry point function in
     * the module
     */
    pub fn new_cleaner_callback(entry_point: RCleanerCBThreadEntry) -> Self {
        Self::CleanerCallback { m_rust_entry_point: entry_point,
                                m_c_entry_point: Some(c_thread_entry) }
    }
}

#[cfg(feature = "enable_kernel_methods")]
impl ThreadEntryData {
    /** # Constructs a `ThreadEntryData::User`
     *
     * The returned instance is used by the kernel code and doesn't fills
     * the `m_c_entry_point` field
     */
    pub fn new_user_data(entry_point: RUserThreadEntry, arg: usize) -> Self {
        Self::User { m_rust_entry_point: entry_point,
                     m_arg: arg,
                     m_c_entry_point: None }
    }

    /** # Constructs a `ThreadEntryData::WatchCallback`
     *
     * The returned instance is used by the kernel code and doesn't fills
     * the `m_c_entry_point` field
     */
    pub fn new_watch_data(entry_point: RWatchCBThreadEntry,
                          use_instant: ObjUseInstant)
                          -> Self {
        Self::WatchCallback { m_rust_entry_point: entry_point,
                              m_use_instant: use_instant,
                              m_c_entry_point: None }
    }

    /** # Constructs a `ThreadEntryData::CleanerCallback`
     *
     * The returned instance is used by the kernel code and doesn't fills
     * the `m_c_entry_point` field
     */
    pub fn new_cleaner_data(entry_point: RCleanerCBThreadEntry) -> Self {
        Self::CleanerCallback { m_rust_entry_point: entry_point,
                                m_c_entry_point: None }
    }
}

impl Default for ThreadEntryData {
    /** Returns the "default value" for a type.
     */
    fn default() -> Self {
        Self::None
    }
}

/** # Thread C entry point
 *
 * This C-compatible function ensures ABI compatibility among non plain rust
 * environments (like the user and the kernel, which traverse a little piece
 * of code that is not pure rust)
 */
#[inline(never)]
extern "C" fn c_thread_entry() -> ! {
    /* obtain the handle to the executing thread */
    let this_thread = Thread::this();

    /* obtain the task entry point data for the execution */
    match this_thread.get_entry_data() {
        ThreadEntryData::User { m_rust_entry_point,
                                m_arg,
                                .. } => {
            /* call the rust entry point function returned by the entry point data and
             * use the return value to decide whether the termination must include the
             * call of the cleanups functions (if any)
             */
            let call_cleanup = m_rust_entry_point(m_arg);
            this_thread.terminate(call_cleanup).unwrap();
        },
        ThreadEntryData::WatchCallback { m_rust_entry_point,
                                         m_use_instant,
                                         .. } => {
            /* call the rust entry point function returned by the entry point data and
             * use the return value to decide whether current callback must be re-used
             * by the kernel for the next event that interest the same filter used by
             * this one
             */
            let re_use_callback = m_rust_entry_point(m_use_instant);
            this_thread.callback_return(Some(re_use_callback as usize));
        },
        ThreadEntryData::CleanerCallback { m_rust_entry_point,
                                           .. } => {
            /* call the rust entry point function returned by the entry point data */
            m_rust_entry_point();
            this_thread.callback_return(None);
        },
        ThreadEntryData::None => {
            /* ouch, this is a really serious bug into the kernel.
             * Currently the kernel knows which variant return when requested via
             * get_entry_data() without additional parameters because the Proc kernel's
             * struct stores the current execution step
             */
            panic!("Kernel BUG! Returned a non User entry data in c_entry_for_user")
        }
    }

    /* put an unreachable placeholder to silent the compiler warnings */
    unreachable!();
}
