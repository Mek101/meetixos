/*! HH_Loader panic handler implementation */

use core::panic::PanicInfo;

use shared::logger::error;

/**
 * Writes to the log output the Kernel panic message and halts the Kernel
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

    loop { /* halt forever */ }
}
