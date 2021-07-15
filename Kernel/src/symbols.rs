/*! Kernel code symbols */

use core::str;

use helps::{
    dbg::C_MIB,
    str::str_len
};

const C_STORAGE_SIZE: usize = 1 * C_MIB;

/* the linker will put the symbols here after the extraction */
#[link_section = ".kernel_symbols"]
static S_KERNEL_SYMBOLS_STORAGE: [u8; C_STORAGE_SIZE] = [0; C_STORAGE_SIZE];

/* initialized with the sub-slice of <S_KERNEL_SYMBOLS_STORAGE> */
static mut SM_KERNEL_SYMBOLS: &'static str = "";

pub fn kernel_symbols_early_init() {
    /* find the length of the symbols table */
    let symbols_len = str_len(&S_KERNEL_SYMBOLS_STORAGE);
    assert_ne!(symbols_len, 0,
               "S_KERNEL_SYMBOLS_STORAGE is smaller than the effective symbols size, \
                increase the static size!");

    let symbols_slice = &S_KERNEL_SYMBOLS_STORAGE[..symbols_len];
    unsafe {
        SM_KERNEL_SYMBOLS =
            str::from_utf8(symbols_slice).expect("Corrupted kernel symbols");

        crate::dbg_println!(crate::dbg_print::DbgLevel::Trace,
                            "\nSymbols:\n{}",
                            SM_KERNEL_SYMBOLS);
    }
}
