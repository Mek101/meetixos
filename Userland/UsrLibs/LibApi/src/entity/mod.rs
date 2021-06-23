/*! Operating System entities */

use api_data::{
    ent::{
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
     * Puts into `buf` the name of this `OsEntityHandle`
     */
    fn name<'a>(&self, buf: &'a mut [u8]) -> Result<&'a str> {
        self.inst_kern_call_2(KernFnPath::OsEntity(KernOsEntFnId::Name),
                              buf.as_mut_ptr() as usize,
                              buf.len())
            .map(move |buf_len| u8_slice_to_str_slice(&buf[..buf_len]))
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
     * Value of the `OSEntityType` that matches the implementation
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
     * Puts into `buf` the name of this `OSEntity`
     */
    fn name<'a>(&self, buf: &'a mut [u8]) -> Result<&'a str> {
        self.os_entity_handle().name(buf)
    }
}
