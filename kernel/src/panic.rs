/*! Kernel panic handler implementation */

use core::panic::PanicInfo;

use shared::logger::log_error;

/**
 * Writes to the log output the kernel panic message and halts the kernel
 */
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    log_error!("<< KERNEL PANIC >>");

    /* show the message if provided */
    if let Some(message) = info.message() {
        log_error!("An unrecoverable error has occurred:");
        log_error!("> {}", message);
    }

    /* show the file location when given */
    if let Some(location) = info.location() {
        log_error!("> {} at {}:{}", location.file(), location.line(), location.column());
    }

    loop { /* halt forever TODO halt other CPUs */ }
}
