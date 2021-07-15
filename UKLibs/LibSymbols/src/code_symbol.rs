/*! Code symbol definition */

use core::{
    cmp::Ordering,
    fmt
};

use crate::{
    demangle,
    Demangle
};

/**
 * Callable code function symbol
 */
pub struct CodeSymbol<'a> {
    m_virt_addr: usize,
    m_demangled: Demangle<'a>
}

impl<'a> CodeSymbol<'a> /* Constructors */ {
    /**
     * Constructs a `CodeSymbol` from the given `raw_line` string.
     *
     * Expects the following format:
     * 0xxxxxxxxxxxx symbol_name
     */
    pub fn from_raw_line(raw_line: &'a str) -> Option<Self> {
        /* split the line using whitespaces */
        let mut line_parts = raw_line.split_ascii_whitespace();

        /* obtain and parse as integer the first part */
        let virt_addr = {
            let str_virt_addr = line_parts.next()?;
            if str_virt_addr.is_empty() {
                return None;
            }

            usize::from_str_radix(str_virt_addr, 16).ok()?
        };

        /* obtains and puts the demangled symbol name into a <String> object */
        let symbol_name = {
            let str_symbol_name = line_parts.next()?;
            if str_symbol_name.is_empty() {
                return None;
            }

            demangle(str_symbol_name)
        };

        Some(Self { m_virt_addr: virt_addr,
                    m_demangled: symbol_name })
    }
}

impl<'a> CodeSymbol<'a> /* Getters */ {
    /**
     * Returns the virtual address on which this symbol starts
     */
    pub fn virt_addr(&self) -> usize {
        self.m_virt_addr
    }

    /**
     * Returns the demangled symbol name `String`
     */
    pub fn symbol_name(&self) -> &Demangle<'a> {
        &self.m_demangled
    }
}

impl<'a> PartialEq for CodeSymbol<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.m_virt_addr == other.m_virt_addr
    }
}

impl<'a> Eq for CodeSymbol<'a> {
}

impl<'a> PartialOrd for CodeSymbol<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.m_virt_addr.partial_cmp(&other.m_virt_addr)
    }
}

impl<'a> Ord for CodeSymbol<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.m_virt_addr.cmp(&other.m_virt_addr)
    }
}

impl<'a> fmt::Display for CodeSymbol<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#018x} - {:#}", self.m_virt_addr, self.m_demangled)
    }
}
