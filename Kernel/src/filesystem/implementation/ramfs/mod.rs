mod node;

use crate::filesystem::implementation::{
    DirectoryNode,
    INode
};
use alloc::collections::{
    HashMap,
    Vec
};

struct RamFS<'a> {
    file_map: HashMap<&'a str, RamFileNode>
}

impl Filesystem for RamFS {
}

impl DirectoryNode for RamFS {
    fn get_content(&self) -> Result<&dyn Iterator<Item = &dyn INode>, &str> {
        todo!()
    }
}
