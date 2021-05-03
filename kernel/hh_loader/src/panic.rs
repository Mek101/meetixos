use core::panic::PanicInfo;

use logger::info;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    info!("PANIC");
    loop {}
}
