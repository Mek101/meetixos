/*! Page table */

use crate::vm::page_table_entry::PageTableEntry;

pub struct PageTable {
    m_entries: [PageTableEntry; 512]
}

impl PageTable /* Constructors */ {
    pub fn new() -> Self {
        Self { m_entries: [PageTableEntry::new(); 512] }
    }
}
