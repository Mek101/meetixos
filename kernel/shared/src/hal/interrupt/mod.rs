/*! # HAL Interrupts Handling
 *
 * Implements an high level architecture independent way to manage
 * interrupts through asynchronous routines called when the architecture
 * specific interrupts are thrown
 */

pub mod manager;
pub mod stack_frame;
