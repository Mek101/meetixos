/*! List of symbols */

use alloc::vec::Vec;

use crate::code_symbol::CodeSymbol;

/**
 * Ordered list of code symbols
 */
pub struct CodeSymbolsList {
    m_symbols: Vec<CodeSymbol>
}

impl CodeSymbolsList {
    /**
     * Constructs a `CodeSymbolsList` reading the newline-separated list of
     * raw symbols
     */
    pub fn load_from_raw(raw_symbols: &str) -> Self {
        Self { m_symbols: raw_symbols.split('\n')
                                     .map(CodeSymbol::from_raw_line)
                                     .filter_map(|code_symbol_opt| code_symbol_opt)
                                     .collect() }
    }

    /**
     * Returns the `CodeSymbol` for the given virtual address
     */
    pub fn symbol_at(&self, virt_addr: usize) -> Option<&CodeSymbol> {
        for i in 0..self.m_symbols.len() {
            if virt_addr < self.m_symbols[i + 1].virt_addr() {
                return Some(&self.m_symbols[i]);
            }
        }
        None
    }
}
