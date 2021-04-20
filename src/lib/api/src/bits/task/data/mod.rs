/*! # `Task` Data
 *
 * Implements various data structures and constants used by the kernel and
 * the internals api to exchange informations about tasks
 */

pub use spec::*;
pub use thread::*;

mod spec;
mod thread;
