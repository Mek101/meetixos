/*! `Task` modes bits */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use crate::{
    tasks::impls::thread::Thread,
    time::Duration
};

/**
 * Lists the available scheduling policies that can be given to
 * `TaskConfig::with_sched_policy()`
 */
#[repr(u8)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum SchedPolicy {
    /**
     * The default policy when no other policy are specified.
     *
     * The use of this policy enables the default task scheduling
     * algorithm that works with a prioritized RR queue.
     *
     * The task is interrupted each time his time quantum has
     * finished
     */
    Preemptive,

    /**
     * The use of this policy is recommended only for small and
     * uninterruptible tasks that must have control on when they can
     * be interrupted (Real Time tasks).
     *
     * The tasks that use this policy must release the CPU with
     * `Task::yield_next()`
     */
    Cooperative
}

/**
 * Lists the available classes of priorities for a task
 */
#[repr(u8)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum TaskPrio {
    Idle     = 0,
    VeryLow  = 1,
    Low      = 2,
    Normal   = 3,
    High     = 4,
    VeryHigh = 5,
    Max      = 6
}

/**
 * Lists the available options for `TaskConfig::with_cpu()`.
 *
 * Allow the user to specify whether a `Task` must be affine to a
 * restricted set of CPUs in an SMP environment or can be executed
 * on any of the available CPUs
 */
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskCpu {
    /**
     * The default affinity when no other are specified.
     *
     * The use of this variant tells to the Kernel that the task can
     * be executed on any available CPU according to the Kernel's
     * affinity algorithm
     */
    Any,

    /**
     * Usable when the task must be executed by a deterministic subset of
     * the CPUs available (in SMP environment) for optimizations.
     *
     * The variant contains a 64bit unsigned integer usable as bitfield mask
     * to enable the CPU(s) that can execute the task.
     *
     * The less significant bit is the first core, so 01 means: the task
     * will be executed ONLY by the first CPU's core.
     *
     * When enabled more bit than the actually available CPUs these bits are
     * ignored by the Kernel.
     */
    Mask(u64)
}

impl TaskCpu {
    /**
     * Returns the variant cardinal value
     */
    pub fn option(&self) -> usize {
        match self {
            TaskCpu::Any => 0,
            TaskCpu::Mask(_) => 1
        }
    }

    /**
     * Returns `Some(bitmask)` when `self` is `TaskCpu::Mask`
     */
    pub fn mask_bits(&self) -> Option<u64> {
        match *self {
            TaskCpu::Any => None,
            TaskCpu::Mask(mask) => Some(mask)
        }
    }
}

/**
 * Lists the available reasons for which a `Thread` can wait
 */
pub enum WaitFor {
    /**
     * The current `Thread` sleeps for a precise quantum of time expressed
     * by the given `Duration`
     */
    Quantum(Duration),

    /**
     * The current `Thread` sleeps until the given one is not terminated.
     *
     * The `Thread` must not be the same
     */
    Join(Thread),

    /**
     * The current `Thread` sleeps until the interrupt identified by the
     * given number not throws
     */
    Irq(u32)
}

/**
 * Lists the available `Proc::mount()` modes
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum MountMode {
    /**
     * The filesystem is visible to all the processes in any of the active
     * sessions
     */
    OSGlobal,

    /**
     * The filesystem is visible only by the processes in the current
     * session (ancestors of the caller too)
     */
    SessionGlobal,

    /**
     * The filesystem is visible only by the process which have performed
     * the mount and his children
     */
    ChildInheritable,

    /**
     * The filesystem is only visible to the caller process
     */
    PrivateToProc
}
