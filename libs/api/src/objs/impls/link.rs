/*! # Open Link Object
 *
 * Implements the abstraction of the filesystem link
 */

use os::sysc::{
    codes::KernLinkFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::obj::{
        ObjType,
        WithTraversableDataObject
    },
    caller::{
        KernCaller,
        Result
    },
    objs::{
        impls::Any,
        ObjId,
        Object,
        UserCreatable
    }
};

/** # Open Link
 *
 * Represents a reference to an open link on the VFS.
 *
 * It allows to explicitly dereference the linked element, that is
 * returned as [`Any`], or link a new VFS visible element.
 *
 * The `Link`, under MeetiX OS, acts like:
 * * [an hard link] - because it keeps the reference to the linked object
 *   (so when it is deleted still remain reachable through this link, only
 *   if it isn't a volatile object like the [`MMap`]s and the [`Link`]nels)
 * * [a soft link] - because allows cross filesystem reference and can link
 *   any object represented into the filesystem (not only files)
 *
 * When a `Link` is opened (via [`ObjConfig`]) as a non-`Link` object
 * (i.e a [`File`]), the kernel tries to automatically dereference it
 * to the linked object, if the type matches the open returns the
 * object, otherwise fails
 *
 * [`Any`]: crate::objs::impls::Any
 * [an hard link]: https://en.wikipedia.org/wiki/Hard_link
 * [`MMap`]: crate::objs::impls::MMap
 * [`Link`]: crate::objs::impls::Link
 * [a soft link]: https://en.wikipedia.org/wiki/Symbolic_link
 * [`ObjConfig`]: crate::objs::ObjConfig
 * [`File`]: crate::objs::impls::File
 */
#[repr(transparent)]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Link {
    m_handle: ObjId
}

impl Link {
    /** # Dereferences this `Link`
     *
     * Returns an [`Any`] instance that can be downcast to the real type to
     * perform the operations on it.
     *
     * The [`Any`] contains a valid and opened [`ObjId`] that can perform
     * system calls; it is opened with the same configuration of this link
     *
     * [`Any`]: crate::objs::impls::Any
     * [`ObjId`]: crate::objs::ObjId
     */
    pub fn deref_link(&self) -> Result<Any> {
        self.kern_call_0(KernFnPath::Link(KernLinkFnId::Deref))
            .map(|obj_id| Any::from(ObjId::from(obj_id)))
    }

    /** # Reference a new `Object`
     *
     * The given object must have a name (created with
     * [`ObjConfig::apply_for()`]), otherwise an error is returned.
     *
     * If the `Link` already references an object it is overwritten
     * (definitively deleted if it have no more references).
     *
     * [`ObjConfig::apply_for()`]: crate::objs::ObjConfig::apply_for
     */
    pub fn refer_to<T: Object>(&self, object: &T) -> Result<()> {
        self.kern_call_2(KernFnPath::Link(KernLinkFnId::ReferTo),
                         object.obj_handle().as_raw_usize(),
                         T::TYPE.into())
            .map(|_| ())
    }
}

impl Object for Link {
    /** The value of the [`ObjType`] that matches the implementation
     *
     * [`ObjType`]: crate::bits::obj::types::ObjType
     */
    const TYPE: ObjType = ObjType::Link;

    /** Returns the immutable reference to the underling [`ObjId`] instance
     *
     * [`ObjId`]: crate::objs::ObjId
     */
    fn obj_handle(&self) -> &ObjId {
        &self.m_handle
    }

    /** Returns the mutable reference to the underling [`ObjId`] instance
     *
     * [`ObjId`]: crate::objs::ObjId
     */
    fn obj_handle_mut(&mut self) -> &mut ObjId {
        &mut self.m_handle
    }
}

impl From<ObjId> for Link {
    /** Performs the conversion
     */
    fn from(id: ObjId) -> Self {
        Self { m_handle: id }
    }
}

impl KernCaller for Link {
    /** Returns the upper 32bits of the 64bit identifier of a system call
     */
    fn caller_handle_bits(&self) -> u32 {
        self.obj_handle().caller_handle_bits()
    }
}

impl WithTraversableDataObject for Link {
    /* No methods to implement */
}

impl UserCreatable for Link {
    /* No methods to implement */
}
