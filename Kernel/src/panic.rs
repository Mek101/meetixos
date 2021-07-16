/*! Kernel panic handler implementation */

use core::panic::PanicInfo;

use crate::{
    dbg_print::DbgLevel,
    dbg_println,
    vm::mem_manager::MemManager
};
use symbols::code_symbols::CodeSymbols;

/**
 * Kernel panic catcher implementation
 */
#[panic_handler]
fn kernel_panic_handler(panic_info: &PanicInfo) -> ! {
    dbg_println!(DbgLevel::Err, "<< KERNEL PANIC >>");

    /* show the message if provided */
    if let Some(message) = panic_info.message() {
        dbg_println!(DbgLevel::Err, "An unrecoverable error has occurred:");
        dbg_println!(DbgLevel::Err, ">> {}", message);
    }

    /* show the file location when given */
    if let Some(location) = panic_info.location() {
        dbg_println!(DbgLevel::Err,
                     ">> File {} at {}:{}",
                     location.file(),
                     location.line(),
                     location.column());
    }

    /* print the stack backtrace */
    if CodeSymbols::are_available() {
        let kern_text_range = MemManager::instance().layout_manager().kern_text_range();
        let back_tracer_display =
            CodeSymbols::instance().back_tracer_from_here(*kern_text_range.start,
                                                          *kern_text_range.end);

        dbg_println!(DbgLevel::Err, "Kernel stack Backtrace:\n{}", back_tracer_display);
    }

    loop { /* halt forever TODO halt other CPUs */ }
}
