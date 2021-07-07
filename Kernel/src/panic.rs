/*! Kernel panic handler implementation */

use core::panic::PanicInfo;

use crate::{
    dbg::print::DbgLevel,
    dbg_println
};

/**
 * Kernel panic catcher implementation
 */
#[panic_handler]
fn kernel_panic_handler(panic_info: &PanicInfo) -> ! {
    dbg_println!(DbgLevel::Err, "<< KERNEL PANIC >>");

    /* show the message if provided */
    if let Some(message) = panic_info.message() {
        dbg_println!(DbgLevel::Err, "An unrecoverable error has occurred:");
        dbg_println!(DbgLevel::Err, "> {}", message);
    }

    /* show the file location when given */
    if let Some(location) = panic_info.location() {
        dbg_println!(DbgLevel::Err,
                     "> {} at {}:{}",
                     location.file(),
                     location.line(),
                     location.column());
    }

    /* TODO print the backtrace */
    // symbols_print_backtrace();

    loop { /* halt forever TODO halt other CPUs */ }
}
