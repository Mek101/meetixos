/*! # x86_64 Interrupt
 *
 * Implements the architecture specific structures used by the HAL's
 * interrupt management
 */

pub use manager::X64InterruptManager as HwInterruptManager;
pub use stack_frame::X64InterruptStackFrame as HwInterruptStackFrame;

mod manager;
mod stack_frame;
