/*! Open Directory `Object` */

use core::{
    fmt,
    ops::Deref,
    str
};

use helps::str::{
    copy_str_to_u8_buf,
    u8_ptr_to_str_slice
};
use os::{
    limits::VFS_NAME_LEN_MAX,
    sysc::{
        codes::KernDirFnId,
        fn_path::KernFnPath
    }
};

use crate::{
    bits::obj::types::ObjType,
    caller::{
        KernCaller,
        Result
    },
    objs::{
        impls::iter::KrnIterator,
        object::{
            ObjId,
            Object,
            UserCreatable
        }
    }
};

/**
 * Reference to an open directory on the VFS.
 *
 * It allows to iterate the directory's children or gain/modify metadata
 * information about this directory if the caller have the right
 * permissions.
 *
 * The `Dir` provides an `Iterator` implementation, so it is possible
 * to iterate the children in a for loop using the `Dir::iter()` method
 */
#[repr(transparent)]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Dir {
    m_handle: ObjId
}

impl Dir {
    /**
     * Returns an iterator that starts from the current index until reaches
     * `ErrorClass::EndOfDataReached`
     */
    pub fn iter(&self) -> Result<impl Iterator<Item = DirEntry>> {
        self.kern_call_0(KernFnPath::Dir(KernDirFnId::InitIter))
            .map(|iter_id| DirIter(KrnIterator::from(ObjId::from(iter_id))))
    }
}

impl Object for Dir {
    const TYPE: ObjType = ObjType::Dir;

    fn obj_handle(&self) -> &ObjId {
        &self.m_handle
    }

    fn obj_handle_mut(&mut self) -> &mut ObjId {
        &mut self.m_handle
    }
}

impl From<ObjId> for Dir {
    fn from(id: ObjId) -> Self {
        Self { m_handle: id }
    }
}

impl KernCaller for Dir {
    fn caller_handle_bits(&self) -> u32 {
        self.obj_handle().caller_handle_bits()
    }
}

impl UserCreatable for Dir {
    /* No methods to implement */
}

/**
 * `Dir` Iterator
 *
 * Allows to iterate with a for each `DirEntry` of the referenced
 * directory
 */
pub struct DirIter(KrnIterator);

impl Deref for DirIter {
    type Target = KrnIterator;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Iterator for DirIter {
    type Item = DirEntry;

    /** It is possible to reuse the same `Dir` iterator rewinding it using
     * `DirIter::set_pos()`
     */
    fn next(&mut self) -> Option<Self::Item> {
        self.0.find_next().unwrap()
    }
}

impl DoubleEndedIterator for DirIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.find_next_back().unwrap()
    }
}

/**
 * Child unit inside a `Dir`
 */
#[derive(Debug, Copy, Clone)]
pub struct DirEntry {
    m_name: [u8; VFS_NAME_LEN_MAX],
    m_name_len: usize,
    m_type: ObjType
}

impl DirEntry {
    /**
     * Returns the name of the child as string slice
     */
    pub fn name(&self) -> &str {
        u8_ptr_to_str_slice(self.m_name.as_ptr(), self.m_name_len)
    }

    /**
     * Returns the `ObjType` of the child
     */
    pub fn obj_type(&self) -> ObjType {
        self.m_type
    }
}

#[cfg(feature = "enable_kernel_methods")]
impl DirEntry {
    /**
     * Constructs a new `DirEntry` with the given values
     */
    pub fn new(name: &str, obj_type: ObjType) -> Self {
        let mut name_buffer = [0; VFS_NAME_LEN_MAX];
        copy_str_to_u8_buf(&mut name_buffer, name);

        Self { m_name: name_buffer,
               m_name_len: name.len(),
               m_type: obj_type }
    }
}

impl Default for DirEntry {
    fn default() -> Self {
        Self { m_name: [0; VFS_NAME_LEN_MAX],
               m_name_len: 0,
               m_type: ObjType::default() }
    }
}

impl fmt::Display for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.obj_type(), self.name())
    }
}
