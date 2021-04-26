/*! # `Task` Implementations
 *
 * Exports the various [`Task`] implementations
 *
 * [`Task`]: /api/tasks/trait.Task.html
 */

pub use proc::*;
pub use thread::*;

mod proc;
mod thread;
