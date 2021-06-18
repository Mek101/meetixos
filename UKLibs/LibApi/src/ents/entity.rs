/*! Operating System base ent */

use core::str;

use helps::str::u8_slice_to_str_slice;
use os::sysc::{
    codes::KernOsEntFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::ent::types::OsEntityType,
    caller::{
        KernCaller,
        Result
    },
    config::{
        CreatMode,
        FindMode
    },
    ents::config::OsEntConfig
};

/**
 * Registered operating system ent handle.
 *
 * It intended as something to which permissions can be applied and have
 * relation with other entities.
 *
 * Itself this obj doesn't have much utilities because most of his
 * methods are private, but exposed via the `OSEntity`] trait and
 * implemented by the `OSUser` and the `OSGroup`
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct OsEntityId {
    m_raw: u16
}

impl OsEntityId {
    /**
     * Puts into `buf` the name of this `OSEntityId`
     */
    fn name<'a>(&self, buf: &'a mut [u8]) -> Result<&'a str> {
        self.kern_call_2(KernFnPath::OsEntity(KernOsEntFnId::Name),
                         buf.as_mut_ptr() as usize,
                         buf.len())
            .map(move |len| u8_slice_to_str_slice(&buf[..len]))
    }

    /**
     * Returns the raw identifier of this `OSEntityId`
     */
    pub fn as_raw(&self) -> u16 {
        self.m_raw
    }

    /**
     * Returns the raw identifier of this `OSEntityId` as `usize`
     */
    pub fn as_raw_usize(&self) -> usize {
        self.as_raw() as usize
    }
}

impl From<u16> for OsEntityId {
    fn from(raw_id: u16) -> Self {
        Self { m_raw: raw_id }
    }
}

impl From<usize> for OsEntityId {
    fn from(raw_id: usize) -> Self {
        Self::from(raw_id as u16)
    }
}

impl KernCaller for OsEntityId {
    fn caller_handle_bits(&self) -> u32 {
        self.as_raw() as u32
    }
}

/**
 * Common interface implemented by all the `OSEntityId` based objects.
 *
 * It mainly exposes the private methods of the `OSEntityId` for safe
 * calling
 */
pub trait OsEntity: From<OsEntityId> + Default {
    /**
     * Value of the `OSEntityType` that matches the implementation
     */
    const TYPE: OsEntityType;

    /**
     * Returns the immutable reference to the underling `OSEntityId` handle
     */
    fn os_entity_handle(&self) -> &OsEntityId;

    /**
     * Returns an uninitialized `OSEntConfig` to create a new `OSEntity`
     */
    fn creat() -> OsEntConfig<Self, CreatMode> {
        OsEntConfig::<Self, CreatMode>::new()
    }

    /**
     * Returns an uninitialized `OSEntConfig` to find existing `OSEntity`
     */
    fn find() -> OsEntConfig<Self, FindMode> {
        OsEntConfig::<Self, FindMode>::new()
    }

    /**
     * Puts into `buf` the name of this `OSEntity`
     */
    fn name<'a>(&self, buf: &'a mut [u8]) -> Result<&'a str> {
        self.os_entity_handle().name(buf)
    }
}
