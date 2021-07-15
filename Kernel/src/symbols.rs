/*! Kernel code symbols */

use core::str;

use helps::str::str_len;

use crate::dbg::C_MIB;

/* the linker will put the symbols here after the extraction */
#[link_section = ".kernel_symbols"]
static S_KERNEL_SYMBOLS_STORAGE: [u8; 1 * C_MIB] = [0; 1 * C_MIB];

/* initialized with the sub-slice of <S_KERNEL_SYMBOLS_STORAGE> */
static mut SM_KERNEL_SYMBOLS: &'static str = "";

pub fn kernel_symbols_init() {
    /* find the length of the symbols table */
    let symbols_len = str_len(&S_KERNEL_SYMBOLS_STORAGE);
    assert_ne!(symbols_len, 0,
               "S_KERNEL_SYMBOLS_STORAGE is smaller than the real-kernel symbols size, \
                increase the static size!");

    let symbols_slice = &S_KERNEL_SYMBOLS_STORAGE[..symbols_len];
    unsafe {
        SM_KERNEL_SYMBOLS =
            str::from_utf8(symbols_slice).expect("Corrupted kernel symbols");

        crate::dbg_println!(crate::dbg::print::DbgLevel::Trace,
                            "\nSymbols:\n{}",
                            SM_KERNEL_SYMBOLS);
    }
}
