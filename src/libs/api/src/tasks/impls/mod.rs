/*! # `Task` Implementations
 *
 * Exports the various [`Task`] implementations
 *
 * [`Task`]: crate::tasks::task::Task
 */

pub use proc::*;
pub use thread::*;

mod proc;
mod thread;
