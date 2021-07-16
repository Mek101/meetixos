/*! Code symbols wrapper */

use alloc::vec::Vec;
use core::{
    fmt::Display,
    str
};

use helps::{
    dbg::C_MIB,
    str::str_len
};

use crate::{
    code_symbol::CodeSymbol,
    stack_back_trace::StackBackTrace
};

/**
 * The initial reserved size for the `S_EXE_SYMBOLS_STORAGE`
 */
const C_STORAGE_SIZE_MAX: usize = 1 * C_MIB;

/**
 * Dedicated section inside the ELF where the build-process stores the code
 * symbols from llvm-nm
 */
#[link_section = ".code_symbols"]
static S_EXE_SYMBOLS_STORAGE: [u8; C_STORAGE_SIZE_MAX] = [0; C_STORAGE_SIZE_MAX];

/**
 * Global instance of the `CodeSymbols`
 */
static mut SM_CODE_SYMBOLS: Option<CodeSymbols> = None;

/**
 * Singleton container for `CodeSymbol`s
 */
pub struct CodeSymbols {
    m_code_symbols: Vec<CodeSymbol<'static>>
}

impl CodeSymbols /* Constructors */ {
    /**
     * Constructs the global `SM_CODE_SYMBOLS` instance
     */
    pub fn init_instance() {
        unsafe {
            assert!(SM_CODE_SYMBOLS.is_none(),
                    "Tried to re-initialize global CodeSymbols instance");

            SM_CODE_SYMBOLS = Some(Self::new());
        }
    }

    /**
     * Constructs a filled `CodeSymbols`
     */
    fn new() -> Self {
        /* calculate the length of the symbols */
        let symbols_len = str_len(&S_EXE_SYMBOLS_STORAGE);
        assert_ne!(symbols_len, 0,
                   "The executable symbols occupies more than the reserved storage, \
                    increase C_STORAGE_SIZE_MAX");

        /* extract the sub-slice of the storage with the symbols */
        let symbols_str_slice = {
            let storage_sub_slice = &S_EXE_SYMBOLS_STORAGE[..symbols_len];

            str::from_utf8(storage_sub_slice).expect("Invalid symbols")
        };

        /* return the instance */
        Self { m_code_symbols:
                   symbols_str_slice.split("\n")
                                    .map(CodeSymbol::from_raw_line)
                                    .filter_map(|opt_code_symbol| opt_code_symbol)
                                    .collect() }
    }
}

impl CodeSymbols /* Methods */ {
    /**
     * Returns a `Display` implementation which shows the stack back-trace
     */
    #[inline(always)]
    pub fn back_tracer_from_here(&self,
                                 text_begin: usize,
                                 text_end: usize)
                                 -> impl Display {
        StackBackTrace::new(text_begin, text_end)
    }

    /**
     * Returns the `CodeSymbol` for the given virtual address
     */
    pub fn symbol_at(&self, virt_addr: usize) -> Option<&CodeSymbol<'static>> {
        for code_symbol in self.code_symbols().iter().rev() {
            if virt_addr >= code_symbol.virt_addr() {
                return Some(code_symbol);
            }
        }
        None
    }
}

impl CodeSymbols /* Getters */ {
    /**
     * Returns the global `CodeSymbols` instance
     */
    pub fn instance() -> &'static Self {
        unsafe {
            SM_CODE_SYMBOLS.as_ref().expect("Requested global CodeSymbols instance \
                                             before initialization")
        }
    }

    /**
     * Returns whether the global `CodeSymbols` instance is initialized
     */
    pub fn are_available() -> bool {
        unsafe { SM_CODE_SYMBOLS.is_some() }
    }

    /**
     * Returns the inner `Vec` with all the `CodeSymbol`s
     */
    pub fn code_symbols(&self) -> &Vec<CodeSymbol<'static>> {
        &self.m_code_symbols
    }
}
