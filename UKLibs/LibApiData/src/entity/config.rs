/*! `OsEntity` configuration */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use bits::flags::{
    BitFlags,
    BitFlagsValues
};

use crate::entity::{
    types::OsEntityType,
    OsEntityId
};

/**
 * Internally used configuration flags
 */
pub type OsEntityConfigFlags = BitFlags<usize, OsEntityConfigBits>;

/**
 * Userland/Kernel interchangeable `OsEntity` configuration
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct RawOsEntityConfig<'a> {
    m_id: Option<OsEntityId>,
    m_type: OsEntityType,
    m_name: Option<&'a str>,
    m_flags: OsEntityConfigFlags
}

impl<'a> RawOsEntityConfig<'a> {
    /**
     * Constructs and empty `RawOsEntityConfig`
     */
    pub fn new(os_ent_type: OsEntityType, is_creat: bool) -> Self {
        let config_flags = if is_creat {
            OsEntityConfigFlags::new_zero() | OsEntityConfigBits::Creat
        } else {
            OsEntityConfigFlags::new_zero()
        };

        Self { m_flags: config_flags,
               m_id: None,
               m_name: None,
               m_type: os_ent_type }
    }

    /**
     * Returns the optionally stored `OsEntityId`
     */
    pub fn id(&self) -> Option<OsEntityId> {
        self.m_id
    }

    /**
     * Sets an explicit `OsEntityId`
     */
    pub fn set_id(&mut self, raw_id: OsEntityId) {
        self.m_id = Some(raw_id);
    }

    /**
     * Returns the `OsEntityType`
     */
    pub fn entity_type(&self) -> OsEntityType {
        self.m_type
    }

    /**
     * Sets the `OsEntityType`    
     */
    pub fn set_entity_type(&mut self, ent_type: OsEntityType) {
        self.m_type = ent_type;
    }

    /**
     * Returns the optionally stored `OsEntity` name
     */
    pub fn name(&self) -> Option<&'a str> {
        self.m_name
    }

    /**
     * Sets an explicit `OsEntity` name
     */
    pub fn set_name(&mut self, name: &'a str) {
        self.m_name = Some(name);
    }

    /**
     * Returns the reference to te `OsEntityConfigFlags`
     */
    pub fn flags(&self) -> &OsEntityConfigFlags {
        &self.m_flags
    }

    /**
     * Returns the mutable reference to te `OsEntityConfigFlags`
     */
    pub fn flags_mut(&mut self) -> &mut OsEntityConfigFlags {
        &mut self.m_flags
    }

    /**
     * Returns `&self` as usize pointer value
     */
    pub fn as_syscall_ptr(&self) -> usize {
        self as *const Self as usize
    }
}

/**
 * Lists the internal `RawOsEntityConfig` flags
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum OsEntityConfigBits {
    /**
     * Enabled when called `OsEntity::creat()`
     */
    Creat,

    /**
     * Marks the `OsEntity` as administrative entity
     */
    Admin
}

impl BitFlagsValues for OsEntityConfigBits {
}
