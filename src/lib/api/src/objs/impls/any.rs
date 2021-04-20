/*! # Any Object Holder
 *
 * Implements an holder that could contain any [`Object`] based type
 *
 * [`Object`]: /api/objs/trait.Object.html
 */

use core::{any::type_name, result};

use crate::{
    bits::obj::{ObjType, RecvMode},
    caller::Result,
    objs::{impls::File, ObjId, Object}
};

/** # Any Object Holder
 *
 * Implements an older that could contain any type of [`Object`] based
 * types.
 *
 * The `Any` can be safely downcast to his real type with his methods
 *
 * [`Object`]: /api/objs/trait.Object.html
 */
#[derive(Debug, Default)]
pub struct Any(ObjId);

impl Any {
    /** # Safe downcast
     *
     * Fails whenever the underling real type of the object is not the
     * downcast one.
     *
     * this method consumes the object
     */
    pub fn downcast<T: Object>(self) -> result::Result<T, Self> {
        if self.real_type() == T::TYPE {
            Ok(T::from(self.0))
        } else {
            Err(self)
        }
    }

    /** # Unsafe downcast
     *
     * If the given type mismatch the underling type throws a [`panic!()`]
     *
     * [`panic!()`]: https://doc.rust-lang.org/std/macro.panic.html
     */
    pub unsafe fn downcast_panic<T: Object>(self) -> T {
        // check for the static type, converts if the same, panic otherwise
        if self.real_type() == T::TYPE {
            T::from(self.0)
        } else {
            panic!("Any({})::downcast_panic<{}> - Failed, {} != {}",
                   self.0.as_raw(),
                   type_name::<T>(),
                   self.real_type(),
                   T::TYPE);
        }
    }

    /** # Accepts an incoming `Object`
     *
     * The previous handle is first released with [`Drop`] then overwritten
     * with the new handle received according to the [`RecvMode`] given
     *
     * [`Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html
     * [`RecvMode`]: /api/bits/obj/enum.RecvMode.html
     */
    pub fn recv(&mut self, mode: RecvMode) -> Result<()> {
        self.0.recv(ObjType::Unknown, mode)
    }

    /** # Constructs a new `Any` from the incoming one
     *
     * Convenience method that internally creates an uninitialized object
     * instance then performs an [`Any::recv()`] using the given
     * [`RecvMode`]
     *
     * [`Any::recv()`]: /api/objs/impls/struct.Any.html#method.recv
     * [`RecvMode`]: /api/bits/obj/enum.RecvMode.html
     */
    pub fn recv_new(mode: RecvMode) -> Result<Self> {
        let mut any = Self::default();
        any.recv(mode).map(|_| any)
    }

    /** Returns the underling [`ObjType`]
     *
     * [`ObjType`]: /api/bits/obj/enum.ObjType.html
     */
    pub fn real_type(&self) -> ObjType {
        self.0.infos::<File>().unwrap_or_default().obj_type()
    }

    /** Returns whether this `Any` is a [`File`]
     *
     * [`File`]: /api/objs/impls/struct.File.html
     */
    pub fn is_file(&self) -> bool {
        self.real_type() == ObjType::File
    }

    /** Returns whether this `Any` is a [`Dir`]
     *
     * [`Dir`]: /api/objs/impls/struct.Dir.html
     */
    pub fn is_dir(&self) -> bool {
        self.real_type() == ObjType::Dir
    }

    /** Returns whether this `Any` is a [`Link`]
     *
     * [`Link`]: /api/objs/impls/struct.Link.html
     */
    pub fn is_link(&self) -> bool {
        self.real_type() == ObjType::Link
    }

    /** Returns whether this `Any` is a [`IpcChan`]
     *
     * [`IpcChan`]: /api/objs/impls/struct.IpcChan.html
     */
    pub fn is_chan(&self) -> bool {
        self.real_type() == ObjType::IpcChan
    }

    /** Returns whether this `Any` is a [`MMap`]
     *
     * [`MMap`]: /api/objs/impls/struct.MMap.html
     */
    pub fn is_mmap(&self) -> bool {
        self.real_type() == ObjType::MMap
    }

    /** Returns whether this `Any` is a [`OsRawMutex`]
     *
     * [`OsRawMutex`]: /api/objs/impls/struct.OsRawMutex.html
     */
    pub fn is_raw_mutex(&self) -> bool {
        self.real_type() == ObjType::OsRawMutex
    }

    /** Returns whether this `Any` is a [`KrnIterator`]
     *
     * [`KrnIterator`]: /api/objs/impls/struct.KrnIterator.html
     */
    pub fn is_iterator(&self) -> bool {
        self.real_type() == ObjType::Iterator
    }

    /** Returns whether this `Any` is a [`Unknown`]
     *
     * [`Unknown`]: /api/bits/obj/enum.ObjType.html#variant.Unknown
     */
    pub fn is_unknown(&self) -> bool {
        self.real_type() == ObjType::Unknown
    }
}

impl From<ObjId> for Any {
    /** Performs the conversion.
     */
    fn from(id: ObjId) -> Self {
        Self(id)
    }
}
