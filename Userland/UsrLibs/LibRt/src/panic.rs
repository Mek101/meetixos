/*! Userspace panic handler */

use core::panic::PanicInfo;

/**
 * Userspace panic handler
 */
#[panic_handler]
fn userland_panic_handler(_info: &PanicInfo) -> ! {
    loop { /* TODO */ }
}
