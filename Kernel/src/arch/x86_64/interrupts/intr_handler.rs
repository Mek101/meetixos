/*! x86_64 interrupt handler */

use crate::arch::interrupts::intr_stack_frame::IntrStackFrame;

extern "C" {
    pub fn syscall_entry();
}

#[no_mangle]
extern "C" fn interrupt_handler(intr_stack_frame: &mut IntrStackFrame) {
    panic!("Interrupt occurred\n{:?}", intr_stack_frame);
}

#[no_mangle]
extern "C" fn syscall_handler(intr_stack_frame: &mut IntrStackFrame) {
    panic!("Interrupt occurred\n{:?}", intr_stack_frame);
}
