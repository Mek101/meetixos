/*! `Task` implementation's specific data */

use core::cmp::min;

use os::{
    limits::{
        PROC_ARG_COUNT_MAX,
        PROC_ARG_LEN_MAX,
        THREAD_NAME_LEN_MAX
    },
    str_utils
};

use crate::{
    bits::task::data::thread::{
        RUserThreadEntry,
        ThreadEntryData
    },
    objs::impls::file::File
};

/**
 * Initializes a standard `RawProcArgs`
 */
pub const RAW_PROC_ARGS_INIT: RawProcArgs = [[0; PROC_ARG_LEN_MAX]; PROC_ARG_COUNT_MAX];

/**
 * Raw process arguments collection
 */
pub type RawProcArgs = [[u8; PROC_ARG_LEN_MAX]; PROC_ARG_COUNT_MAX];

/**
 * Lists the variants that contains the task implementation specific data
 */
#[derive(Debug, Clone)]
pub enum TaskSpecData {
    /**
     * Contains the specific data to construct and execute a `Thread`
     */
    Thread {
        m_user_thread: ThreadEntryData,
        m_name: Option<[u8; THREAD_NAME_LEN_MAX]>
    },

    /**
     * Contains the specific data to construct and execute a `Proc`
     */
    Proc {
        m_executable: File,
        m_args: Option<RawProcArgs>
    },

    /**
     * Default value usable only as uninitialized value
     */
    None
}

impl TaskSpecData {
    /**
     * Constructs a `TaskSpecData::Thread` filling with the given data
     */
    pub fn new_thread(entry_point: Option<RUserThreadEntry>,
                      arg: Option<usize>,
                      name: Option<&str>)
                      -> Self {
        Self::Thread { m_user_thread: entry_point.map(|entry_point| {
                                          /* construct the ThreadEntryData for the user
                                           * thread when the entry_point is given with
                                           * Option::Some
                                           */
                                          ThreadEntryData::new_user(entry_point,
                                                                    arg.unwrap())
                                      })
                                      .unwrap_or(ThreadEntryData::None),
                       m_name: name.map(|str_name| {
                                       /* create a temporary byte buffer where store
                                        * the string slice with the name when given
                                        * with Option::Some
                                        */
                                       let mut name_buf = [0; THREAD_NAME_LEN_MAX];
                                       str_utils::copy_str_to_u8_buf(&mut name_buf,
                                                                     str_name);
                                       name_buf
                                   }) }
    }

    /**
     * Constructs a `TaskSpecData::Proc` filling with the given data.
     *
     * Arguments that exceed `PROC_ARG_LEN_MAX` and the `PROC_ARG_COUNT_MAX`
     * are truncated
     */
    pub fn new_proc(exe: File, args: Option<&[&str]>) -> Self {
        Self::Proc { m_executable: exe,
                     m_args: args.map(|args| {
                                     /* create a temporary matrix byte buffer where
                                      * store the array of string slices with the
                                      * arguments when given with Option::Some
                                      */
                                     let mut args_buf = RAW_PROC_ARGS_INIT;
                                     for i in 0..min(PROC_ARG_COUNT_MAX, args.len()) {
                                         str_utils::copy_str_to_u8_buf(&mut args_buf[i],
                                                                       args[i])
                                     }
                                     args_buf
                                 }) }
    }
}
