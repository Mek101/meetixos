use core::panic::PanicInfo;

use shared::logger::info;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    info!("PANIC");
    loop {}
}
