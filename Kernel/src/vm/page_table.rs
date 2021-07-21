/*! Page table */

use core::ops::{
    Index,
    IndexMut
};

use crate::vm::page_table_entry::PageTableEntry;

#[repr(C)]
#[repr(align(4096))]
pub struct PageTable {
    m_entries: [PageTableEntry; 512]
}

impl PageTable /* Constructors */ {
    pub fn new() -> Self {
        Self { m_entries: [PageTableEntry::new(); 512] }
    }
}

impl PageTable /* Methods */ {
    pub fn clear(&mut self) {
        for entry in self.m_entries.iter_mut() {
            entry.set_unused();
        }
    }
}

impl Index<PageTableIndex> for PageTable {
    type Output = PageTableEntry;

    fn index(&self, index: PageTableIndex) -> &Self::Output {
        let index: usize = index.into();
        &self.m_entries[index]
    }
}

impl IndexMut<PageTableIndex> for PageTable {
    fn index_mut(&mut self, index: PageTableIndex) -> &mut Self::Output {
        let index: usize = index.into();
        &mut self.m_entries[index]
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
pub struct PageTableIndex {
    m_index: u16
}

impl From<u16> for PageTableIndex {
    fn from(raw_index: u16) -> Self {
        Self { m_index: raw_index % 512 }
    }
}

impl Into<usize> for PageTableIndex {
    fn into(self) -> usize {
        self.m_index as usize
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
pub enum PageTableLevel {
    Root,
    OneGiB,
    TwoMiB,
    FourKiB
}
