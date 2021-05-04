/*! # Operating System Base Entity
 *
 * Implements the base struct that represents the OS entities
 */

use core::str;

use os::{
    str_utils,
    sysc::{
        codes::KernOSEntFnId,
        fn_path::KernFnPath
    }
};

use crate::{
    bits::ent::OSEntityType,
    caller::{
        KernCaller,
        Result
    },
    config::{
        CreatMode,
        FindMode
    },
    ents::OSEntConfig
};

/** # Operating System Entity Handle
 *
 * Represents a registered operating system entity intended as something to
 * which permissions can be applied and have relation with other entities.
 *
 * Itself this object doesn't have much utilities because most of his
 * methods are private, but exposed via the [`OSEntity`] trait and
 * implemented by the [`OSUser`] and the [`OSGroup`]
 *
 * [`OSEntity`]: crate::ents::OSEntity
 * [`OSUser`]: crate::ents::impls::OSUser
 * [`OSGroup`]: crate::ents::impls::OSGroup
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct OSEntityId(u16);

impl OSEntityId {
    /** # Obtains the `OSEntityId`'s name
     *
     * Puts into `buf` the name of this `OSEntityId`
     */
    fn name<'a>(&self, buf: &'a mut [u8]) -> Result<&'a str> {
        self.kern_call_2(KernFnPath::OSEntity(KernOSEntFnId::Name),
                         buf.as_mut_ptr() as usize,
                         buf.len())
            .map(move |len| str_utils::u8_slice_to_str_slice(&buf[..len]))
    }

    /** Returns the raw identifier of this `OSEntityId`
     */
    pub fn as_raw(&self) -> u16 {
        self.0
    }

    /** Returns the raw identifier of this `OSEntityId` as `usize`
     */
    pub fn as_raw_usize(&self) -> usize {
        self.as_raw() as usize
    }
}

impl From<u16> for OSEntityId {
    /** Performs the conversion
     */
    fn from(raw_id: u16) -> Self {
        Self(raw_id)
    }
}

impl From<usize> for OSEntityId {
    /** Performs the conversion
     */
    fn from(raw_id: usize) -> Self {
        Self::from(raw_id as u16)
    }
}

impl KernCaller for OSEntityId {
    /** Returns the raw identifier of this `OSEntityId`
     */
    fn caller_handle_bits(&self) -> u32 {
        self.as_raw() as u32
    }
}

/** # Operating System Entity Interface
 *
 * Defines a common interface implemented by all the [`OSEntityId`] based
 * objects.
 *
 * It mainly exposes the private methods of the [`OSEntityId`] for safe
 * calling.
 *
 * [`OSEntityId`]: crate::ents::OSEntityId
 */
pub trait OSEntity: From<OSEntityId> + Default {
    /** The value of the [`OSEntityType`] that matches the implementation
     *
     * [`OSEntityType`]: crate::bits::ent::types::OSEntityType
     */
    const TYPE: OSEntityType;

    /** Returns the immutable reference to the underling [`OSEntityId`]
     * instance
     *
     * [`OSEntityId`]: crate::ents::OSEntityId
     */
    fn os_entity_handle(&self) -> &OSEntityId;

    /** Returns an uninitialized [`OSEntConfig`] to create a new [`OSEntity`]
     *
     * [`OSEntConfig`]: crate::ents::OSEntConfig
     * [`OSEntity`]: crate::ents::OSEntity
     */
    fn creat() -> OSEntConfig<Self, CreatMode> {
        OSEntConfig::<Self, CreatMode>::new()
    }

    /** Returns an uninitialized [`OSEntConfig`] to find existing
     * [`OSEntity`]
     *
     * [`OSEntConfig`]: crate::ents::OSEntConfig
     * [`OSEntity`]: crate::ents::OSEntity
     */
    fn find() -> OSEntConfig<Self, FindMode> {
        OSEntConfig::<Self, FindMode>::new()
    }

    /** # Obtains the `OSEntity`'s name
     *
     * Puts into `buf` the name of this [`OSEntity`]
     *
     * [`OSEntity`]: crate::ents::OSEntity
     */
    fn name<'a>(&self, buf: &'a mut [u8]) -> Result<&'a str> {
        self.os_entity_handle().name(buf)
    }
}
