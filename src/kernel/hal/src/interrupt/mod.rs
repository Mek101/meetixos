/*! # HAL Interrupts Handling
 *
 * Implements an high level architecture independent way to manage
 * interrupts through asynchronous routines called when the architecture
 * specific interrupts are thrown
 */

pub use manager::*;
pub use stack_frame::*;

mod manager;
mod stack_frame;
