/*! Page table */

use crate::vm::paging::page_table_entry::PageTableEntry;

pub struct PageTable {
    m_entries: [PageTableEntry; 512]
}

impl PageTable /* Constructors */ {
}
