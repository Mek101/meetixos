/*! # Kernel Interrupt Manager
 *
 * Implements the kernel interrupt manager
 */

use hal::interrupt::{InterruptManager, InterruptManagerException, InterruptStackFrame};
use logger::info;

static mut INTERRUPT_MANAGER: InterruptManager = InterruptManager::new_uninitialized();

pub fn init_interrupts() {
    unsafe {
        INTERRUPT_MANAGER.set_except_handler(InterruptManagerException::PageFault,
                                             page_fault_except_handler);

        INTERRUPT_MANAGER.enable_as_global();
    }
    info!("Interrupts initialized");
}

fn page_fault_except_handler(_stack_frame: InterruptStackFrame,
                             _exception: InterruptManagerException)
                             -> bool {
    panic!("page fault exception merdeeeeee");
}
