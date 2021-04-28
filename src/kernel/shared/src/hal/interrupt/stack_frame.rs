/*! # HAL Interrupt Stack Frame
 *
 * Implements the hardware independent interrupt stack frame wrapper
 */

use crate::{
    addr::{Address, VirtAddr},
    arch::hal::interrupt::HwInterruptStackFrame
};

/** # Interrupt Stack Frame Wrapper
 *
 * Architecture independent wrapper for a mutable reference of an hardware
 * interrupt stack frame
 */
pub struct InterruptStackFrame<'a> {
    m_inner: &'a mut HwInterruptStackFrame<'a>
}

impl<'a> InterruptStackFrame<'a> {
    /** # Constructs an `InterruptStackFrame`
     *
     * The returned instance wraps the given hardware interrupt stack frame
     */
    pub(crate) fn new(hw_intr_stack: &'a mut HwInterruptStackFrame<'a>) -> Self {
        Self { m_inner: hw_intr_stack }
    }

    /** Returns the [`VirtAddr`] of the last or next instruction
     *
     * [`VirtAddr`]: /hal/addr/struct.VirtAddr.html
     */
    pub fn instruction_ptr(&self) -> VirtAddr {
        unsafe { VirtAddr::new_unchecked(self.m_inner.instruction_ptr()) }
    }

    /** Returns the [`VirtAddr`] of the current stack pointer position
     *
     * [`VirtAddr`]: /hal/addr/struct.VirtAddr.html
     */
    pub fn stack_ptr(&self) -> VirtAddr {
        unsafe { VirtAddr::new_unchecked(self.m_inner.stack_ptr()) }
    }
}

/** # Hardware Interrupt Stack Frame Base Interface
 *
 * Defines a little amount of methods on which the [`InterruptStackFrame`]
 * relies to obtain informations
 *
 * [`InterruptStackFrame`]: /hal/interrupt/struct.InterruptStackFrame.html
 */
pub(crate) trait HwInterruptStackFrameBase {
    /** Returns the raw value of the current/next instruction pointer
     */
    fn instruction_ptr(&self) -> usize;

    /** Returns the raw value of the current stack pointer
     */
    fn stack_ptr(&self) -> usize;
}
