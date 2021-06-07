/*! Any `Object` holder */

use core::{
    any::type_name,
    result
};

use crate::{
    bits::obj::{
        modes::RecvMode,
        types::ObjType
    },
    caller::Result,
    objs::{
        impls::file::File,
        object::{
            ObjId,
            Object
        }
    }
};

/**
 * Wrapper that can contains any type of `Object` based object.
 *
 * The `Any` can be safely downcast to his real type with his methods
 */
#[derive(Debug, Default)]
pub struct Any {
    m_obj: ObjId
}

impl Any {
    /**
     * Safe downcast fails whenever the underling real type of the object is
     * not the downcast destination one.
     */
    pub fn downcast<T: Object>(self) -> result::Result<T, Self> {
        if self.real_type() == T::TYPE {
            Ok(T::from(self.m_obj))
        } else {
            Err(self)
        }
    }

    /**
     * Unsafe downcast `panic!()`s if the real type and the downcast
     * destination type mismatches
     */
    pub unsafe fn downcast_panic<T: Object>(self) -> T {
        /* check for the static type, converts if the same, panic otherwise */
        if self.real_type() == T::TYPE {
            T::from(self.m_obj)
        } else {
            panic!("Any({})::downcast_panic<{}> - Failed, {} != {}",
                   self.m_obj.as_raw(),
                   type_name::<T>(),
                   self.real_type(),
                   T::TYPE);
        }
    }

    /**  
     * Accepts an incoming `Object`
     *
     * The previous handle is first released with `Drop` then overwritten
     * with the new handle received according to the `RecvMode` given
     */
    pub fn recv(&mut self, mode: RecvMode) -> Result<()> {
        self.m_obj.recv(ObjType::Unknown, mode)
    }

    /**
     * Convenience method that internally creates an uninitialized object
     * instance then performs an `Any::recv()` using the given `RecvMode`
     */
    pub fn recv_new(mode: RecvMode) -> Result<Self> {
        let mut any = Self::default();
        any.recv(mode).map(|_| any)
    }

    /**
     * Returns the underling `ObjType`
     */
    pub fn real_type(&self) -> ObjType {
        self.m_obj.info::<File>().unwrap_or_default().obj_type()
    }

    /**
     * Returns whether this `Any` is a `File`
     */
    pub fn is_file(&self) -> bool {
        self.real_type() == ObjType::File
    }

    /**
     * Returns whether this `Any` is a `Dir`
     */
    pub fn is_dir(&self) -> bool {
        self.real_type() == ObjType::Dir
    }

    /**
     * Returns whether this `Any` is a `Link`
     */
    pub fn is_link(&self) -> bool {
        self.real_type() == ObjType::Link
    }

    /**
     * Returns whether this `Any` is a `IpcChan`
     */
    pub fn is_chan(&self) -> bool {
        self.real_type() == ObjType::IpcChan
    }

    /**
     * Returns whether this `Any` is a `MMap`
     */
    pub fn is_mmap(&self) -> bool {
        self.real_type() == ObjType::MMap
    }

    /**
     * Returns whether this `Any` is a `OsRawMutex`
     */
    pub fn is_raw_mutex(&self) -> bool {
        self.real_type() == ObjType::OsRawMutex
    }

    /**
     * Returns whether this `Any` is a `KrnIterator`
     */
    pub fn is_iterator(&self) -> bool {
        self.real_type() == ObjType::KrnIterator
    }

    /**
     * Returns whether this `Any` is a `Unknown`
     */
    pub fn is_unknown(&self) -> bool {
        self.real_type() == ObjType::Unknown
    }
}

impl From<ObjId> for Any {
    fn from(obj_id: ObjId) -> Self {
        Self { m_obj: obj_id }
    }
}
