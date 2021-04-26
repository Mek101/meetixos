/*! # Task Modes Bits
 *
 * Implements various enumerations that are used for certain [`Task`]
 * related calls
 *
 * [`Task`]: /api/tasks/trait.Task.html
 */

use crate::{tasks::impls::Thread, time::Duration};

c_handy_enum! {
    /** # `Task` Scheduling Policy
     *
     * Lists the available scheduling policies that can be given to
     * [`TaskConfig::with_sched_policy()`]
     *
     * [`TaskConfig::with_sched_policy()`]:
     * /api/tasks/struct.TaskConfig.html#method.with_sched_policy
     */
    pub enum SchedPolicy: u8 {
        /** The default policy when no other policy are specified.
         *
         * The use of this policy enables the default task scheduling
         * algorithm that works with a prioritized RR queue.
         *
         * The task is interrupted each time his time quantum has
         * finished
         */
        Preemptive  = 0,

        /** The use of this policy is recommended only for small and
         * uninterruptible tasks that must have control on when they can
         * be interrupted (Real Time tasks).
         *
         * The tasks that use this policy must release the CPU with
         * [`Task::yield_next()`]
         *
         * [`Task::yield_next()`]: /api/tasks/trait.Task.html#method.yield_next
         */
        Cooperative = 1,
    }
}

c_handy_enum! {
    /** # `Task` Priority
     *
     * Lists the available classes of priorities for a task
     */
    pub enum TaskPrio: u8 {
        Idle     = 0,
        VeryLow  = 1,
        Low      = 2,
        Normal   = 3,
        High     = 4,
        VeryHigh = 5,
        Max      = 6,
    }
}

rust_handy_enum! {
    /** # `Task` CPU Affinity
     *
     * Allow the user to specify whether a [`Task`] must be affine to a
     * restricted set of CPUs in an SMP environment or can be executed
     * on any of the available CPUs.
     *
     * The use of this enumeration is intended with [`TaskConfig::with_cpu()`]
     *
     * [`Task`]: /api/tasks/trait.Task.html
     * [`TaskConfig::with_cpu()`]: /api/tasks/struct.TaskConfig.html#method.with_cpu
     */
    pub enum TaskCpu: u8 {
        /** # No CPU affinity
         *
         * The default affinity when no other are specified.
         *
         * The use of this variant tells to the kernel that the task can
         * be executed on any available CPU according to the kernel's
         * affinity algorithm
         */
        Any = 0,

        /** # Deterministic CPU affinity
         *
         * Usable when the task must be executed by a deterministic subset of the
         * CPUs available (in SMP environment) for optimizations.
         *
         * The variant contains a 64bit unsigned integer usable as bitfield mask to
         * enable the CPU(s) that can execute the task.
         *
         * The less significant bit is the first core, so 01 means: the task will
         * be executed ONLY by the first CPU's core.
         *
         * When enabled more bit than the actually available CPUs these bits are
         * ignored by the kernel.
         */
        Mask(bits: u64) = 1,
    }
}

impl TaskCpu {
    /** Returns [`Some(bitmask)`] when `self` is `TaskCpu::Mask`
     *
     * [`Some(bitmask)`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
     */
    pub fn mask_bits(&self) -> Option<u64> {
        match *self {
            TaskCpu::Any => None,
            TaskCpu::Mask(mask) => Some(mask)
        }
    }
}

/** # `Thread` Wait Reason
 *
 * Lists the available reasons for which a [`Thread`] can wait
 *
 * [`Thread`]: /api/tasks/impls/struct.Thread.html
 */
pub enum WaitFor {
    /** The current [`Thread`] sleeps for a precise quantum of time expressed
     * by the given [`Duration`]
     *
     * [`Thread`]: /api/tasks/impls/struct.Thread.html
     * [`Duration`]: /api/time/struct.Duration.html
     */
    Quantum(Duration),

    /** The current [`Thread`] sleeps until the given one is not terminated.
     *
     * The [`Thread`] must not be the same
     *
     * [`Thread`]: /api/tasks/impls/struct.Thread.html
     */
    Join(Thread),

    /** The current [`Thread`] sleeps until the interrupt identified by the
     * given number not throws
     *
     * [`Thread`]: /api/tasks/impls/struct.Thread.html
     */
    Irq(u32)
}
