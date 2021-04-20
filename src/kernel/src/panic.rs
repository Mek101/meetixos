/*! # Kernel Panic Handler
 *
 * Implements the kernel panic handler
 */

use core::panic::PanicInfo;

use crate::{log::error, write_video};

/** # Panic Handler
 *
 * Writes to the log output the kernel panic message and halts the kernel
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

    write_video("[KernelPanic] ");
    loop { /* halt forever: TODO halt other CPUs */ }
}
