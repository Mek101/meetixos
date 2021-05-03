/*! # `Task` Types
 *
 * Implements the variants that identifies the various [`TaskId`]
 * implementations
 *
 * [`TaskId`]: crate::tasks::task::TaskId
 */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/** # `Task` Types
 *
 * Lists the available object types represented by an [`TaskId`]
 *
 * [`TaskId`]: crate::tasks::task::TaskId
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum TaskType {
    /** No real uses, used as default value
     */
    Unknown,

    /** Identifies a [`Thread`] task
     *
     * [`Thread`]: crate::tasks::impls::thread::Thread
     */
    Thread,

    /** Identifies a [`Proc`] task
     *
     * [`Proc`]: crate::tasks::impls::proc::Proc
     */
    Proc
}
