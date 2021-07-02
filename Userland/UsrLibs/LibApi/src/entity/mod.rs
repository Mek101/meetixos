/*! Operating System entities */

use alloc::string::String;

use api_data::{
    entity::{
        types::OsEntityType,
        OsEntityId
    },
    sys::{
        codes::KernOsEntFnId,
        fn_path::KernFnPath
    }
};
use helps::str::u8_slice_to_str_slice;

use crate::{
    config::{
        CreatMode,
        OpenMode
    },
    entity::config::OsEntityConfig,
    handle::{
        KernHandle,
        Result
    }
};
use api_data::limit::OS_ENTITY_NAME_LEN_MAX;

pub mod config;
pub mod impls;

/**
 * Generic opaque `OsEntity` handle
 */
#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct OsEntityHandle {
    m_handle: KernHandle
}

impl OsEntityHandle {
    /**
     * Constructs an `OsEntityHandle` from the `raw_handle` value given
     */
    pub(crate) fn from_raw(raw_handle: usize) -> Self {
        Self { m_handle: KernHandle::from_raw(raw_handle) }
    }

    /**
     * Returns the associated system wide unique `OsEntityId`
     */
    fn id(&self) -> Result<OsEntityId> {
        self.m_handle
            .inst_kern_call_0(KernFnPath::OsEntity(KernOsEntFnId::OsId))
            .map(|raw_os_entity_id| raw_os_entity_id as OsEntityId)
    }

    /**
     * Returns the name of this `OsEntityHandle`
     */
    fn name(&self) -> Result<String> {
        let mut name_str = String::with_capacity(OS_ENTITY_NAME_LEN_MAX);

        self.m_handle
            .inst_kern_call_2(KernFnPath::OsEntity(KernOsEntFnId::Name),
                              name_str.as_mut_ptr() as usize,
                              name_str.capacity())
            .map(|name_len| {
                let mut byte_vec = name_str.into_bytes();
                unsafe {
                    byte_vec.set_len(name_len);
                    String::from_utf8_unchecked(byte_vec)
                }
            })
    }

    /**
     * Returns the reference to the underling `KernHandle`
     */
    #[inline]
    pub fn kern_handle(&self) -> &KernHandle {
        &self.m_handle
    }
}

/**
 * Common interface implemented by all the `OsEntityHandle` based objects.
 *
 * It mainly exposes the private methods of the `OsEntityHandle` for safe
 * calling
 */
pub trait OsEntity: From<OsEntityHandle> + Default {
    /**
     * Value of the `OsEntityType` that matches the implementation
     */
    const TYPE: OsEntityType;

    /**
     * Returns the immutable reference to the underling `OsEntityHandle`
     * handle
     */
    fn os_entity_handle(&self) -> &OsEntityHandle;

    /**
     * Returns an `OsEntityConfig` for `OsEntity` creation
     */
    fn creat() -> OsEntityConfig<Self, CreatMode> {
        OsEntityConfig::<Self, CreatMode>::new()
    }

    /**
     * Returns an `OsEntityConfig` for `OsEntity` opening
     */
    fn open() -> OsEntityConfig<Self, OpenMode> {
        OsEntityConfig::<Self, OpenMode>::new()
    }

    /**
     * Returns the associated system wide unique `OsEntityId`
     */
    fn id(&self) -> Result<OsEntityId> {
        self.os_entity_handle().id()
    }

    /**
     * Puts into `buf` the name of this `OsEntity`
     */
    fn name<'a>(&self) -> Result<String> {
        self.os_entity_handle().name()
    }
}
