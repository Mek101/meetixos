/*! Interrupt Stack Frame */

use crate::{
    addr::{
        virt::VirtAddr,
        Address
    },
    arch::interrupt::stack_frame::HwInterruptStackFrame
};

/**
 * Architecture independent wrapper for a mutable reference of an hardware
 * interrupt stack frame
 */
pub struct InterruptStackFrame<'a> {
    m_inner: &'a mut HwInterruptStackFrame
}

impl<'a> InterruptStackFrame<'a> {
    /**
     * Constructs an `InterruptStackFrame` from the given hardware
     * representation
     */
    pub(crate) fn new(hw_intr_stack: &'a mut HwInterruptStackFrame) -> Self {
        Self { m_inner: hw_intr_stack }
    }

    /**
     * Returns the `VirtAddr` of the last or next instruction
     */
    pub fn instruction_ptr(&self) -> VirtAddr {
        VirtAddr::new(self.m_inner.instruction_ptr())
    }

    /**
     * Sets the given `VirtAddr` as new instruction pointer for the stack
     * frame
     */
    pub unsafe fn set_instruction_ptr(&mut self, virt_addr: VirtAddr) {
        self.m_inner.set_instruction_ptr(virt_addr.as_usize());
    }

    /**
     * Returns the `VirtAddr` of the current stack pointer position
     */
    pub fn stack_ptr(&self) -> VirtAddr {
        VirtAddr::new(self.m_inner.stack_ptr())
    }

    /**
     * Sets the given `VirtAddr` as new stack pointer for the stack frame
     */
    pub unsafe fn set_stack_ptr(&mut self, virt_addr: VirtAddr) {
        self.m_inner.set_stack_ptr(virt_addr.as_usize())
    }
}

/**
 * Interface on which the `InterruptStackFrame` relies to set/get
 * informations
 */
pub(crate) trait HwInterruptStackFrameBase {
    /**
     * Returns the raw value of the current/next instruction pointer
     */
    fn instruction_ptr(&self) -> usize;

    /**
     * Sets the given raw address as new instruction pointer for the stack
     * frame
     */
    unsafe fn set_instruction_ptr(&mut self, raw_addr: usize);

    /**
     * Returns the raw value of the current stack pointer
     */
    fn stack_ptr(&self) -> usize;

    /**
     * Sets the given raw address as new stack pointer for the stack frame
     */
    unsafe fn set_stack_ptr(&mut self, raw_addr: usize);
}
