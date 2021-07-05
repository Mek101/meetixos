/*! Kernel panic handler implementation */

use core::panic::PanicInfo;

#[panic_handler]
fn kernel_panic_handler(_panic_info: &PanicInfo) -> ! {
    loop { /* halt forever TODO halt other CPUs */ }
}
