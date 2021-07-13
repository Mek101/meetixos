use core::any::Any;

use api_data::path::PathComponent;

use crate::filesystem::FsResult;

pub enum NodeType {
    File,
    Directory,
    HardLink,
    SoftLink
}

/**
 * A generic INode used to build the Virtual FileSystem tree.
 */
pub trait INode: Any + Send + Sync {
    /**
     * Returns whatever the INode is readable.
     */
    fn is_readable(&self) -> bool;

    /**
     * Returns whatever the INode is writable.
     */
    fn is_writable(&self) -> bool;

    /**
     * Syncs the INode changes on the underlaying device.
     */
    fn sync(&self) -> FsResult<()>;

    /**
     * Get the name of this INode.
     */
    fn get_name(&self) -> &PathComponent;

    /**
     * Get type of this INode
     */
    fn get_type(&self) -> NodeType;

    /**
     * Internally used for casting.
     */
    fn as_any(&self) -> &dyn Any;
}

impl dyn INode {
    /**
     * Try cast this INode to the `T` type.
     */
    pub fn as_type<T>(&self) -> Option<&T> where T: INode {
        self.as_any().downcast_ref::<T>()
    }
}

pub trait FileNode: INode {
    /**
     * Read bytes from the given offset up to the buffer's length.
     * Returns the number of bytes read if successful.
     */
    fn read_at(&self, offset: usize, buffer: &mut [u8]) -> FsResult<usize>;

    /**
     * Write bytes at the given offset.
     * Returns the number of bytes written successfully.
     */
    fn write_at(&self, offset: usize, buffer: &[u8]) -> FsResult<usize>;

    /**
     * If the `length` is less than the current file's size, truncate it, otherwise extend it with 0s.
     */
    fn resize(&self, length: usize) -> FsResult<()>;
}

pub trait DirectoryNode: INode {
    /**
     * Get the INodes inside this directory.
     * This does NOT include the parent and self directories.
     */
    fn get_nodes(&self) -> FsResult<&dyn Iterator<Item=&dyn INode>>;

    /**
     * Get the number of INodes in this directory.
     */
    fn node_count(&self) -> FsResult<usize>;
}

//pub trait HardLinkNode: INode {
//    fn get_node() -> &dyn INode;
//}

//pub trait SoftLinkNode: INode {
//    fn get_node_path() -> Path
//}