/*! `OSEntity` configuration */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use bits::flags::{
    BitFlags,
    BitFlagsValues
};

use crate::ent::{
    types::OsEntityType,
    RawOsEntityId
};

/**
 * Internally used configuration flags
 */
pub type OsEntityConfigFlags = BitFlags<usize, OsEntityConfigBits>;

/**
 * Userland/Kernel interchangeable `OsEntity` configuration
 */
pub struct RawOsEntityConfig<'a> {
    m_id: Option<RawOsEntityId>,
    m_type: OsEntityType,
    m_name: Option<&'a str>,
    m_flags: OsEntityConfigFlags
}

impl<'a> RawOsEntityConfig<'a> {
    /**
     * Constructs and empty `RawOsEntityConfig`
     */
    pub fn new() -> Self {
        Self { m_flags: OsEntityConfigFlags::new_zero(),
               m_id: None,
               m_name: None,
               m_type: OsEntityType::default() }
    }

    /**
     * Returns the optionally stored `OsEntity` identifier
     */
    pub fn id(&self) -> Option<RawOsEntityId> {
        self.m_id
    }

    /**
     * Sets an explicit `OsEntity` identifier
     */
    pub fn set_id(&mut self, raw_id: RawOsEntityId) {
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
    Creat,
    Admin
}

impl BitFlagsValues for OsEntityConfigBits {
}
