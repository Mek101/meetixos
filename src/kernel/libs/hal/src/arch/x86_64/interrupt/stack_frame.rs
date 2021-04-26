/*! # x86_64 Interrupt Stack Frame
 *
 * Implements the x86_64 interrupt stack frame
 */

use x86_64::structures::idt::InterruptStackFrame;

use crate::interrupt::HwInterruptStackFrameBase;

pub struct X64InterruptStackFrame<'a> {
    m_inner: &'a InterruptStackFrame
}

impl<'a> HwInterruptStackFrameBase for X64InterruptStackFrame<'a> {
    fn instruction_ptr(&self) -> usize {
        self.m_inner.instruction_pointer.as_u64() as usize
    }

    fn stack_ptr(&self) -> usize {
        self.m_inner.stack_pointer.as_u64() as usize
    }
}

impl<'a> From<&'a InterruptStackFrame> for X64InterruptStackFrame<'a> {
    fn from(raw_intr_stack_frame: &'a InterruptStackFrame) -> Self {
        Self { m_inner: raw_intr_stack_frame }
    }
}
