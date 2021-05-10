#![no_std]
#![no_main]

use core::panic::PanicInfo;

use api::objs::{
    impls::file::File,
    object::{
        Object,
        UserCreatable
    }
};

#[no_mangle]
pub unsafe extern "C" fn _start() {
    let f = File::creat().for_read()
                         .for_write()
                         .apply_for("/Users/Marco/Docs/example.txt")
                         .unwrap();

    let mmap = f.map_to_memory(None, 0, f.size() as u64, true)
                .expect("Failed to map file to memory");

    let mut ptr_box = mmap.get_ptr_mut::<u8>().expect("Failed to obtain MMap pointer");
    for byte in ptr_box.iter_mut() {
        *byte = 0;
    }

    /* cannot do anything for now :-( */
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop { /* halt forever */ }
}
