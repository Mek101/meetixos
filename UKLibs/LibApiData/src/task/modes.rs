/*! `Task` modes bits */

use core::time::Duration;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use crate::task::RawTaskId;

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
pub enum TaskExecCpu {
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
     * The less significant bit is the first core, so b0101 means: the task
     * will be executed ONLY by the first and the third CPU's cores.
     *
     * When enabled more bit than the actually available CPUs these bits are
     * ignored by the Kernel.
     */
    Mask(u64)
}

impl TaskExecCpu {
    /**
     * Returns the variant cardinal value
     */
    pub fn option(&self) -> usize {
        match self {
            TaskExecCpu::Any => 0,
            TaskExecCpu::Mask(_) => 1
        }
    }

    /**
     * Returns `Some(bitmask)` when `self` is `TaskCpu::Mask`
     */
    pub fn mask_bits(&self) -> Option<u64> {
        match *self {
            TaskExecCpu::Any => None,
            TaskExecCpu::Mask(mask) => Some(mask)
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
    Join(RawTaskId),

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
    OsGlobal,

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
