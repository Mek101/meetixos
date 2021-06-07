/*! `Thread` specific data */

use crate::{
    objs::info::use_instant::ObjUseInstant,
    tasks::{
        impls::thread::Thread,
        task::Task
    }
};

/**
 * Internal C entry point prototype for `Thread`s
 */
pub type CThreadEntry = extern "C" fn() -> !;

/**
 * Rust `Thread`'s user entry point prototype
 */
pub type RUserThreadEntry = fn(usize) -> bool;

/**
 * Rust `Thread`'s entry point for `Object::watch()` callbacks
 */
pub type RWatchCBThreadEntry = fn(ObjUseInstant) -> bool;

/**
 * Rust `Thread`'s entry point for `Thread::add_cleaner()` callbacks
 */
pub type RCleanerCBThreadEntry = fn();

/**
 * Context dependent `Thread`'s execution data.
 *
 * Each variant contains the executable entry-point and the data needed by
 * the context that represents
 */
#[derive(Debug, Clone)]
pub enum ThreadEntryData {
    /**
     * Data to spawn/execute a new user `Thread` using `Thread::spawn()`
     */
    User {
        m_rust_entry_point: RUserThreadEntry,
        m_arg: usize,
        m_c_entry_point: Option<CThreadEntry>
    },

    /**
     * Data to register/execute a new `Object::watch()` callback
     */
    WatchCallback {
        m_rust_entry_point: RWatchCBThreadEntry,
        m_use_instant: ObjUseInstant,
        m_c_entry_point: Option<CThreadEntry>
    },

    /**
     * Data to register/execute a new `Thread::add_cleaner()` callback
     */
    CleanerCallback {
        m_rust_entry_point: RCleanerCBThreadEntry,
        m_c_entry_point: Option<CThreadEntry>
    },

    /**
     * Default value, usable only for un-initialized `ThreadEntryData`
     * instances
     */
    None
}

impl ThreadEntryData {
    /**
     * Constructs a `ThreadEntryData::User` with the given data
     */
    pub fn new_user(entry_point: RUserThreadEntry, arg: usize) -> Self {
        Self::User { m_rust_entry_point: entry_point,
                     m_arg: arg,
                     m_c_entry_point: Some(c_thread_entry) }
    }

    /**
     * Constructs a `ThreadEntryData::WatchCallback` with the given data
     */
    pub fn new_watch_callback(entry_point: RWatchCBThreadEntry) -> Self {
        Self::WatchCallback { m_rust_entry_point: entry_point,
                              m_use_instant: ObjUseInstant::default(),
                              m_c_entry_point: Some(c_thread_entry) }
    }

    /**
     * Constructs a `ThreadEntryData::CleanerCallback` with the given data
     */
    pub fn new_cleaner_callback(entry_point: RCleanerCBThreadEntry) -> Self {
        Self::CleanerCallback { m_rust_entry_point: entry_point,
                                m_c_entry_point: Some(c_thread_entry) }
    }
}

#[cfg(feature = "enable_kernel_methods")]
impl ThreadEntryData {
    /**
     * Constructs a `ThreadEntryData::User` that not fills the C entry point
     */
    pub fn new_user_data(entry_point: RUserThreadEntry, arg: usize) -> Self {
        Self::User { m_rust_entry_point: entry_point,
                     m_arg: arg,
                     m_c_entry_point: None }
    }

    /**
     * Constructs a `ThreadEntryData::WatchCallback` that not fills the C
     * entry point
     */
    pub fn new_watch_data(entry_point: RWatchCBThreadEntry,
                          use_instant: ObjUseInstant)
                          -> Self {
        Self::WatchCallback { m_rust_entry_point: entry_point,
                              m_use_instant: use_instant,
                              m_c_entry_point: None }
    }

    /**
     * Constructs a `ThreadEntryData::CleanerCallback` that not fills the C
     * entry point
     */
    pub fn new_cleaner_data(entry_point: RCleanerCBThreadEntry) -> Self {
        Self::CleanerCallback { m_rust_entry_point: entry_point,
                                m_c_entry_point: None }
    }
}

impl Default for ThreadEntryData {
    fn default() -> Self {
        Self::None
    }
}

/**
 * C-compatible function ensures ABI compatibility among non plain rust
 * environments (like the user and the Kernel, which traverse a little piece
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
             * by the Kernel for the next event that interest the same filter used by
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
            /* ouch, this is a really serious bug into the Kernel.
             * Currently the Kernel knows which variant return when requested via
             * get_entry_data() without additional parameters because the Proc Kernel's
             * struct stores the current execution step
             */
            panic!("Kernel BUG! Returned a non User entry data in c_entry_for_user")
        }
    }

    /* put an unreachable placeholder to silent the compiler warnings */
    unreachable!();
}
