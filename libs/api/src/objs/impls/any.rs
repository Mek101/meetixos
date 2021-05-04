/*! # Any Object Holder
 *
 * Implements an holder that could contain any [`Object`] based type
 *
 * [`Object`]: crate::objs::Object
 */

use core::{
    any::type_name,
    result
};

use crate::{
    bits::obj::{
        ObjType,
        RecvMode
    },
    caller::Result,
    objs::{
        impls::File,
        ObjId,
        Object
    }
};

/** # Any Object Holder
 *
 * Implements an older that could contain any type of [`Object`] based
 * types.
 *
 * The `Any` can be safely downcast to his real type with his methods
 *
 * [`Object`]: crate::objs::Object
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
     * [`panic!()`]: core::panic!
     */
    pub unsafe fn downcast_panic<T: Object>(self) -> T {
        /* check for the static type, converts if the same, panic otherwise */
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
     * [`Drop`]: core::ops::Drop
     * [`RecvMode`]: crate::bits::obj::modes::RecvMode
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
     * [`Any::recv()`]: crate::objs::impls::Any::recv
     * [`RecvMode`]: crate::bits::obj::modes::RecvMode
     */
    pub fn recv_new(mode: RecvMode) -> Result<Self> {
        let mut any = Self::default();
        any.recv(mode).map(|_| any)
    }

    /** Returns the underling [`ObjType`]
     *
     * [`ObjType`]: crate::bits::obj::types::ObjType
     */
    pub fn real_type(&self) -> ObjType {
        self.0.infos::<File>().unwrap_or_default().obj_type()
    }

    /** Returns whether this `Any` is a [`File`]
     *
     * [`File`]: crate::objs::impls::File
     */
    pub fn is_file(&self) -> bool {
        self.real_type() == ObjType::File
    }

    /** Returns whether this `Any` is a [`Dir`]
     *
     * [`Dir`]: crate::objs::impls::Dir
     */
    pub fn is_dir(&self) -> bool {
        self.real_type() == ObjType::Dir
    }

    /** Returns whether this `Any` is a [`Link`]
     *
     * [`Link`]: crate::objs::impls::Link
     */
    pub fn is_link(&self) -> bool {
        self.real_type() == ObjType::Link
    }

    /** Returns whether this `Any` is a [`IpcChan`]
     *
     * [`IpcChan`]: crate::objs::impls::IpcChan
     */
    pub fn is_chan(&self) -> bool {
        self.real_type() == ObjType::IpcChan
    }

    /** Returns whether this `Any` is a [`MMap`]
     *
     * [`MMap`]: crate::objs::impls::MMap
     */
    pub fn is_mmap(&self) -> bool {
        self.real_type() == ObjType::MMap
    }

    /** Returns whether this `Any` is a [`OsRawMutex`]
     *
     * [`OsRawMutex`]: crate::objs::impls::OsRawMutex
     */
    pub fn is_raw_mutex(&self) -> bool {
        self.real_type() == ObjType::OsRawMutex
    }

    /** Returns whether this `Any` is a [`KrnIterator`]
     *
     * [`KrnIterator`]: crate::objs::impls::KrnIterator
     */
    pub fn is_iterator(&self) -> bool {
        self.real_type() == ObjType::KrnIterator
    }

    /** Returns whether this `Any` is a [`Unknown`]
     *
     * [`Unknown`]: crate::bits::obj::types::ObjType::Unknown
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
