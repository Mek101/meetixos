/*! # Open Directory Object
 *
 * Implements the open directory object
 */

use core::{
    fmt,
    fmt::{Display, Formatter},
    ops::Deref,
    str
};

use os::{
    limits::VFS_NAME_LEN_MAX,
    str_utils,
    sysc::{codes::KernDirFnId, fn_path::KernFnPath}
};

use crate::{
    bits::obj::{ObjType, WithTraversableDataObject},
    caller::{KernCaller, Result},
    objs::{impls::KrnIterator, ObjId, Object, UserCreatable}
};

impl_obj_id_object! {
    /** # Open Directory
     *
     * Represents a reference to an open directory on the VFS.
     *
     * It allows to iterate the directory's children or gain/modify metadata
     * informations about this directory if the caller have the right
     * permissions.
     *
     * The `Dir` provides an [`Iterator`] implementation, so it is possible
     * to iterate the children in a for loop using the [`Dir::iter()`] method
     *
     * [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
     * [`Dir::iter()`]: /api/objs/impls/struct.Dir.html#method.iter
     */
    pub struct Dir : impl WithTraversableDataObject,
                          UserCreatable {
        where TYPE = ObjType::Dir;
    }
}

impl Dir {
    /** # Constructs an `Iterator`
     *
     * The returned iterator starts from the current index until reached
     * [`ErrorClass::EndOfDataReached`]
     *
     * [`ErrorClass::EndOfDataReached`]:
     * /api/errors/class/enum.ErrorClass.html#variant.EndOfDataReached
     */
    pub fn iter(&self) -> Result<impl Iterator<Item = DirEntry>> {
        self.kern_call_0(KernFnPath::Dir(KernDirFnId::InitIter))
            .map(|iter_id| DirIter(KrnIterator::from(ObjId::from(iter_id))))
    }
}

/** # Directory Iterator
 *
 * Allows to iterate with a foreach each [`DirEntry`] of the referenced
 * directory
 *
 * [`DirEntry`]: /api/objs/impls/struct.DirEntry.html
 */
pub struct DirIter(KrnIterator);

impl Deref for DirIter {
    /** The resulting type after dereference.
     */
    type Target = KrnIterator;

    /** Dereferences the value.
     */
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Iterator for DirIter {
    /** The type of the elements being iterated over
     */
    type Item = DirEntry;

    /** It is possible to reuse the same `Dir` iterator rewinding it using
     * [`DirIter::set_pos()`]
     *
     * [`DirIter::set_pos()`]:
     * /api/objs/impls/struct.KrnIterator.html#method.set_pos
     */
    fn next(&mut self) -> Option<Self::Item> {
        self.0.find_next().unwrap()
    }
}

impl DoubleEndedIterator for DirIter {
    /** Removes and returns an element from the end of the iterator.
     */
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.find_next_back().unwrap()
    }
}

/** # Directory Entry
 *
 * Represents the child unit inside a directory
 */
#[derive(Debug, Copy, Clone)]
pub struct DirEntry {
    m_name: [u8; VFS_NAME_LEN_MAX],
    m_name_len: usize,
    m_type: ObjType
}

impl DirEntry {
    /** Returns the name of the child as string slice
     */
    pub fn name(&self) -> &str {
        str_utils::u8_ptr_to_str_slice(self.m_name.as_ptr(), self.m_name_len)
    }

    /** Returns the [`ObjType`] of the child
     *
     * [`ObjType`]: /api/bits/obj/enum.ObjType.html
     */
    pub fn obj_type(&self) -> ObjType {
        self.m_type
    }
}

#[cfg(feature = "enable_kernel_methods")]
impl DirEntry {
    /** # Constructs a new `DirEntry`
     *
     * The returned instance is filled with the given data
     */
    pub fn new(name: &str, obj_type: ObjType) -> Self {
        let mut buf = [0; VFS_NAME_LEN_MAX];
        str_utils::copy_str_to_u8_buf(&mut buf, name);

        Self { m_name: buf,
               m_name_len: name.len(),
               m_type: obj_type }
    }
}

impl Default for DirEntry {
    /** Returns the "default value" for a type.
     */
    fn default() -> Self {
        Self { m_name: [0; VFS_NAME_LEN_MAX],
               m_name_len: 0,
               m_type: ObjType::default() }
    }
}

impl Display for DirEntry {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.obj_type(), self.name())
    }
}
