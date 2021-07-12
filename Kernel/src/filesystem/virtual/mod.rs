use core::any::Any;

use crate::filesystem::FsResult;

pub enum NodeType {
    File,
    Directory
}

pub trait INode: Any + Send + Sync {
    fn is_readable(&self) -> bool;
    fn is_writable(&self) -> bool;

    fn get_name(&self) -> &str;

    fn as_any(&self) -> &dyn Any;
}

impl dyn INode {
    pub fn as_type<T>(&self) -> Option<&T> where T: INode {
        self.as_any().downcast_ref::<T>()
    }
}

pub trait FileNode: INode {
    fn read_at(&self, offset: usize, buffer: &mut [u8]) -> FsResult<usize>;
    fn write_at(&self, offset: usize, buffer: &[u8]) -> FsResult<usize>;

    fn resize(&self, length: usize) -> FsResult<()>;
}

pub trait DirectoryNode: INode {
    fn get_nodes(&self) -> &dyn Iterator<Item=&dyn INode>;

    fn node_count(&self) -> Option<usize>;
}