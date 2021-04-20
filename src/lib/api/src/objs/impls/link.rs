/*! # Open Link Object
 *
 * Implements the abstraction of the filesystem link a a a a a a a a a a a a
 * a a a a a a a a
 */

use os::sysc::{codes::KernLinkFnId, fn_path::KernFnPath};

use crate::{
    bits::obj::{ObjType, WithTraversableDataObject},
    caller::{KernCaller, Result},
    objs::{impls::Any, ObjId, Object, UserCreatable}
};

impl_obj_id_object! {
    /** # Open Link
     *
     * Represents a reference to an open link on the VFS.
     *
     * It allows to explicitly dereference the linked element, that is
     * returned as [`Any`], or link a new VFS visible element.
     *
     * The `Link`, under MeetiX OS, acts like:
     * * [an hard link] - because it keeps the reference to the linked
     *   object (so when it is deleted still remain reachable through this
     *   link, only if it isn't a volatile object like the [`MMap`]s and
     *   the [`IpcChan`]nels)
     * * [a soft link] - because allows cross filesystem reference and
     *   can link any object represented into the filesystem (not only
     *   files)
     *
     * When a `Link` is opened (via [`ObjConfig`]) as a non-`Link` object
     * (i.e a [`File`]), the kernel tries to automatically dereference it
     * to the linked object, if the type matches the open returns the
     * object, otherwise fails
     *
     * [`Any`]: /api/objs/impls/struct.Any.html
     * [an hard link]: https://en.wikipedia.org/wiki/Hard_link
     * [`MMap`]: /api/objs/impls/struct.MMap.html
     * [`IpcChan`]: /api/objs/impls/struct.IpcChan.html
     * [a soft link]: https://en.wikipedia.org/wiki/Symbolic_link
     * [`ObjConfig`]: /api/objs/struct.ObjConfig.html
     * [`File`]: /api/objs/impls/struct.File.html
     */
    pub struct Link : impl WithTraversableDataObject,
                           UserCreatable  {
        where TYPE = ObjType::Link;
    }
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
     * [`Any`]: /api/objs/impls/struct.Any.html
     * [`ObjId`]: /api/objs/struct.ObjId.html
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
     * [`ObjConfig::apply_for()`]:
     * /api/objs/struct.ObjConfig.html#method.apply_for
     */
    pub fn refer_to<T: Object>(&self, object: &T) -> Result<()> {
        self.kern_call_2(KernFnPath::Link(KernLinkFnId::ReferTo),
                         object.obj_handle().as_raw_usize(),
                         T::TYPE.into())
            .map(|_| ())
    }
}
