/*! OldKernel panic handler implementation */

use core::panic::PanicInfo;

use crate::symbols::symbols_print_backtrace;
use shared::logger::error;

/**
 * Writes to the log output the OldKernel panic message and halts the
 * OldKernel
 */
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    error!("<< KERNEL PANIC >>");

    /* show the message if provided */
    if let Some(message) = info.message() {
        error!("An unrecoverable error has occurred:");
        error!("> {}", message);
    }

    /* show the file location when given */
    if let Some(location) = info.location() {
        error!("> {} at {}:{}", location.file(), location.line(), location.column());
    }

    /* print the backtrace */
    symbols_print_backtrace();

    loop { /* halt forever TODO halt other CPUs */ }
}
