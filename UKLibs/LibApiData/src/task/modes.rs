/*! `Task` modes bits */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the available options for `TaskConfig::with_exec_cpu()`.
 *
 * Allow the user to specify whether a `Task` must be affine to a
 * restricted set of CPUs in an SMP environment or can be executed
 * on any of the available CPUs
 */
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum TaskExecCpu {
    /**
     * The default affinity when no other are specified.
     *
     * The use of this variant tells to the Kernel that the `Task` can be
     * executed on any available CPU according to the Kernel's affinity
     * algorithm
     */
    Any,

    /**
     * Usable when the task must be executed by a deterministic subset of
     * the CPUs available (in SMP environment) for optimizations.
     *
     * The variant contains a 64bit unsigned integer usable as `BitFields`
     * mask to enable the CPU(s) that can execute the `Task`.
     *
     * The less significant bit is the first core, so `b0101` means: the
     * `Task` will be executed ONLY by the first and the third CPU's
     * cores.
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
 * Lists the available `Proc::mount()` modes
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum FsMountMode {
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
