//pub mod mxfs;
//pub mod ramfs;
//pub mod sfs;

use core::any::{
    Any,
    TypeId
};

trait INode: Any {}

impl<T: INode> Any for T where T: INode {
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

trait FileNode: INode {
    fn get_name(&self) -> &str;

    fn get_data_stream(&self) -> Result<&dyn Iterator<Item = u8>, &str>;

    /**
     * Hints at the external vnode whatever to cache this file's stream or
     * not.
     */
    fn use_external_cache_hint(&self) -> bool;

    /**
     * Hints at the data size.
     */
    fn size_hint(&self) -> Option<usize>;
}

trait DirectoryNode: INode {
    fn get_content(&self) -> Result<&dyn Iterator<Item = &dyn INode>, &str>;
}
