/*! Operating System base entity */

use core::str;

use os::{
    str_utils,
    sysc::{
        codes::KernOSEntFnId,
        fn_path::KernFnPath
    }
};

use crate::{
    bits::ent::types::OSEntityType,
    caller::{
        KernCaller,
        Result
    },
    config::{
        CreatMode,
        FindMode
    },
    ents::config::OSEntConfig
};

/**
 * Registered operating system entity handle.
 *
 * It intended as something to which permissions can be applied and have
 * relation with other entities.
 *
 * Itself this object doesn't have much utilities because most of his
 * methods are private, but exposed via the `OSEntity`] trait and
 * implemented by the `OSUser` and the `OSGroup`
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct OSEntityId(u16);

impl OSEntityId {
    /**
     * Puts into `buf` the name of this `OSEntityId`
     */
    fn name<'a>(&self, buf: &'a mut [u8]) -> Result<&'a str> {
        self.kern_call_2(KernFnPath::OSEntity(KernOSEntFnId::Name),
                         buf.as_mut_ptr() as usize,
                         buf.len())
            .map(move |len| str_utils::u8_slice_to_str_slice(&buf[..len]))
    }

    /**
     * Returns the raw identifier of this `OSEntityId`
     */
    pub fn as_raw(&self) -> u16 {
        self.0
    }

    /**
     * Returns the raw identifier of this `OSEntityId` as `usize`
     */
    pub fn as_raw_usize(&self) -> usize {
        self.as_raw() as usize
    }
}

impl From<u16> for OSEntityId {
    fn from(raw_id: u16) -> Self {
        Self(raw_id)
    }
}

impl From<usize> for OSEntityId {
    fn from(raw_id: usize) -> Self {
        Self::from(raw_id as u16)
    }
}

impl KernCaller for OSEntityId {
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
pub trait OSEntity: From<OSEntityId> + Default {
    /**
     * Value of the `OSEntityType` that matches the implementation
     */
    const TYPE: OSEntityType;

    /**
     * Returns the immutable reference to the underling `OSEntityId` handle
     */
    fn os_entity_handle(&self) -> &OSEntityId;

    /**
     * Returns an uninitialized `OSEntConfig` to create a new `OSEntity`
     */
    fn creat() -> OSEntConfig<Self, CreatMode> {
        OSEntConfig::<Self, CreatMode>::new()
    }

    /**
     * Returns an uninitialized `OSEntConfig` to find existing `OSEntity`
     */
    fn find() -> OSEntConfig<Self, FindMode> {
        OSEntConfig::<Self, FindMode>::new()
    }

    /**
     * Puts into `buf` the name of this `OSEntity`
     */
    fn name<'a>(&self, buf: &'a mut [u8]) -> Result<&'a str> {
        self.os_entity_handle().name(buf)
    }
}
