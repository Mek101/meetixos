#![no_std]
#![no_main]

use api::objs::{
    impls::File,
    UserCreatable
};
use core::panic::PanicInfo;

#[no_mangle]
pub unsafe extern "C" fn _start() {
    let f = File::creat().for_read()
                         .for_write()
                         .apply_for("/Users/Marco/Docs/example.txt")
                         .unwrap();

    let mut _read_buf = [0u8; 512];
    f.read(&mut _read_buf).unwrap();

    /* cannot do anything for now :-( */
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop { /* halt forever */ }
}
