/*! x86_64 interrupt handler */

use crate::arch::x86_64::intr_stack_frame::IntrStackFrame;

#[no_mangle]
extern "C" fn interrupt_handler(_intr_stack_frame: &mut IntrStackFrame) {
}

#[no_mangle]
extern "C" fn syscall_handler(_intr_stack_frame: &mut IntrStackFrame) {
}
