/*! x86_64 stack frame implementation */

use core::mem;

use x86_64::structures::idt::InterruptStackFrame;

use crate::interrupt::stack_frame::HwInterruptStackFrameBase;

/**
 * x86_64 `HwInterruptStackFrameBase` implementation
 */
#[repr(transparent)]
pub struct X64InterruptStackFrame {
    m_inner: InterruptStackFrame
}

impl X64InterruptStackFrame {
    pub fn wrap_ptr(frame_ptr: &mut InterruptStackFrame) -> &mut Self {
        unsafe { mem::transmute(frame_ptr) }
    }
}

impl HwInterruptStackFrameBase for X64InterruptStackFrame {
    fn instruction_ptr(&self) -> usize {
        self.m_inner.instruction_pointer.as_u64() as usize
    }

    unsafe fn set_instruction_ptr(&mut self, _raw_addr: usize) {
        todo!()
    }

    fn stack_ptr(&self) -> usize {
        self.m_inner.stack_pointer.as_u64() as usize
    }

    unsafe fn set_stack_ptr(&mut self, _raw_addr: usize) {
        todo!()
    }
}
