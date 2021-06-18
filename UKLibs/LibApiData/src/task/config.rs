/*! `Task` configuration */

use core::ptr;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use bits::flags::{
    BitFlags,
    BitFlagsValues
};

use crate::{
    ent::RawOsEntityId,
    obj::RawObjId,
    task::{
        modes::TaskExecCpu,
        thread::{
            CThreadEntry,
            RUserThreadEntry,
            UserThreadArg
        },
        RawTaskId
    }
};

/**
 * Internally used configuration flags
 */
pub type TaskConfigFlags = BitFlags<usize, TaskConfigBits>;

/**
 * Userland/Kernel interchangeable `Task` configuration
 */
pub struct RawTaskConfig<'a> {
    m_id: Option<RawTaskId>,

    /* task execution related fields */
    m_flags: TaskConfigFlags,
    m_exec_cpu: TaskExecCpu,

    /* owner related fields */
    m_os_user: Option<RawOsEntityId>,
    m_os_group: Option<RawOsEntityId>,

    /* process specific parameters */
    m_file_to_exec: RawObjId,
    m_cmdline_args: Option<&'a [&'a str]>,

    /* thread specific parameters */
    m_c_thread_entry: Option<CThreadEntry>,
    m_thread_entry: Option<RUserThreadEntry>,
    m_thread_arg: UserThreadArg
}

impl<'a> RawTaskConfig<'a> {
    /**
     * Constructs an empty `RawTaskConfig`
     */
    pub fn new() -> Self {
        Self { m_id: None,
               m_flags: TaskConfigFlags::new_zero(),
               m_exec_cpu: TaskExecCpu::Any,
               m_os_user: None,
               m_os_group: None,
               m_file_to_exec: 0,
               m_cmdline_args: None,
               m_c_thread_entry: None,
               m_thread_entry: None,
               m_thread_arg: ptr::null() }
    }

    /**
     * Returns the preferred `RawTaskId`
     */
    pub fn id(&self) -> Option<RawTaskId> {
        self.m_id
    }

    /**
     * Sets the preferred `RawTaskId`
     */
    pub fn set_id(&mut self, id: RawTaskId) {
        self.m_id = Some(id);
    }

    /**
     * Returns the reference to the `TaskConfigFlags`
     */
    pub fn flags(&self) -> &TaskConfigFlags {
        &self.m_flags
    }

    /**
     * Returns the mutable reference to the `TaskConfigFlags`
     */
    pub fn flags_mut(&mut self) -> &mut TaskConfigFlags {
        &mut self.m_flags
    }

    /**
     * Returns the `TaskExecCpu` filter
     */
    pub fn exec_cpu(&self) -> TaskExecCpu {
        self.m_exec_cpu
    }

    /**
     * Sets the `TaskExecCpu` filter
     */
    pub fn set_exec_cpu(&mut self, exec_cpu: TaskExecCpu) {
        self.m_exec_cpu = exec_cpu;
    }

    /**
     * Returns the owner user's `RawOsEntityId`
     */
    pub fn os_user(&self) -> Option<RawOsEntityId> {
        self.m_os_user
    }

    /**
     * Sets the owner user's `RawOsEntityId`
     */
    pub fn set_os_user(&mut self, os_user: RawOsEntityId) {
        self.m_os_user = Some(os_user);
    }

    /**
     * Returns the owner group's `RawOsEntityId`
     */
    pub fn os_group(&self) -> Option<RawOsEntityId> {
        self.m_os_group
    }

    /**
     * Sets the owner group's `RawOsEntityId`
     */
    pub fn set_os_group(&mut self, os_group: RawOsEntityId) {
        self.m_os_group = Some(os_group);
    }

    /**
     * Returns the `RawObjId` of the file to execute for the new process
     */
    pub fn file_to_exec(&self) -> RawObjId {
        self.m_file_to_exec
    }

    /**
     * Sets the `RawObjId` of the file to execute for the new process
     */
    pub fn set_file_to_exec(&mut self, file_to_exec: RawObjId) {
        self.m_file_to_exec = file_to_exec;
    }

    /**
     * Returns the reference to the command line arguments of the new
     * process
     */
    pub fn cmdline_args(&self) -> Option<&'a [&'a str]> {
        self.m_cmdline_args
    }

    /**
     * Sets the reference to the command line arguments of the new process
     */
    pub fn set_cmdline_args(&mut self, cmdline_args: &'a [&'a str]) {
        self.m_cmdline_args = Some(cmdline_args);
    }

    /**
     * Returns the `CRawThreadEntry` for the new thread
     */
    pub fn c_thread_entry(&self) -> Option<CThreadEntry> {
        self.m_c_thread_entry
    }

    /**
     * Sets the `CRawThreadEntry` for the new thread
     */
    pub fn set_c_thread_entry(&mut self, c_thread_entry: CThreadEntry) {
        self.m_c_thread_entry = Some(c_thread_entry);
    }

    /**
     * Returns the Rust `RawThreadEntry` for the new thread
     */
    pub fn thread_entry(&self) -> Option<RUserThreadEntry> {
        self.m_thread_entry
    }

    /**
     * Sets the Rust `RawThreadEntry` for the new thread
     */
    pub fn set_thread_entry(&mut self, thread_entry: RUserThreadEntry) {
        self.m_thread_entry = Some(thread_entry);
    }

    /**
     * Returns the `RawThreadArg` for the new thread
     */
    pub fn thread_arg(&self) -> UserThreadArg {
        self.m_thread_arg
    }

    /**
     * Sets the `RawThreadArg` for the new thread
     */
    pub fn set_thread_arg(&mut self, thread_arg: UserThreadArg) {
        self.m_thread_arg = thread_arg;
    }

    /**
     * Returns `&self` as usize pointer value
     */
    pub fn as_syscall_ptr(&self) -> usize {
        self as *const Self as usize
    }
}

/**
 * Lists the internal `RawOsEntityConfig` flags
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum TaskConfigBits {
    /**
     * Forces the use of a cooperative scheduling policy, which means that
     * the `Task` becomes uninterruptible by the kernel until it explicitly
     * releases the CPU
     */
    CoopSched,

    /**
     * Forces the kernel to prefer for this `Task` higher execution
     * priorities for all his life.
     *
     * Useful for highly interactive `Task`s
     */
    HighPrioTask,

    /**
     * Forces the kernel to prefer for this `Task` lower execution
     * priorities for all his life.
     *
     * Useful for background `Task`s or services
     */
    LowPrioTask
}

impl BitFlagsValues for TaskConfigBits {
}
