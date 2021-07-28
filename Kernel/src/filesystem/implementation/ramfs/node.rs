use alloc::collections::Vec;

use crate::filesystem::implementation::{
    FileNode,
    INode
};

struct RamFileNode<'a> {
    name: &'a str,
    file_data: Vec<u8>
}

impl RamFileNode {
    pub fn new(name: &str) -> Self {
        Self { name,
               file_data: Vec::new() }
    }
}

impl FileNode for RamFileNode {
    fn get_name(&self) -> &str {
        self.name
    }

    fn get_data_stream(&self) -> Result<&dyn Iterator<Item = u8>, &str> {
        Ok(self.file_data.iter())
    }

    fn use_external_cache_hint(&self) -> bool {
        false
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.file_data.len())
    }
}

impl INode for RamFileNode {
}
