pub mod implementation;
pub mod r#virtual;
mod loaded_nodes;
mod node_structs;

use alloc::rc::Rc;
use alloc::sync::Arc;

use sync::SpinRwLock;

pub use crate::filesystem::r#virtual::INode;

pub type FsResult<T> = Result<T, FsError>;

pub enum FsError {
    NotSupported,
    InvalidLink
}

/**
 * All supported filesystems.
 */
pub enum Filesystems {
    // No known filesystem.
    None,
    // The Simple FileSystem, per it's original 2006 specification.
    SFS,
    // The Simple FileSystem Extended by Forever Young Software in 2018.
    SFSE,
    // An original, feature-rich, inode-based filesystem.
    MXFS
}

trait FilesystemProvider {
    fn as_enum(&self) -> Filesystems;

    fn verify_superblock(&self) -> Result<(), &str>;

    //fn mount_filesystem() -> Result<&dyn Filesystem, str>;
}

trait Filesystem {
    fn validate_path_in_namespace(&self, path: Rc<&str>) -> Result<&str, &str>;

    fn get_root_node(&self) -> Arc<&dyn INode>;

    fn get_filesystem_state(&self) -> SpinRwLock<()>;
}
