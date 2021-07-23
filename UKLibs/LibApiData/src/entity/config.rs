/*! `OsEntity` configuration */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use bits::bit_flags::{
    BitFlags,
    TBitFlagsValues
};

use crate::{
    entity::{
        types::OsEntityType,
        OsEntityId
    },
    sys::TAsSysCallPtr
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

impl<'a> RawOsEntityConfig<'a> /* Constructors */ {
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
}

impl<'a> RawOsEntityConfig<'a> /* Getters */ {
    /**
     * Returns the optionally stored `OsEntityId`
     */
    pub fn id(&self) -> Option<OsEntityId> {
        self.m_id
    }

    /**
     * Returns the `OsEntityType`
     */
    pub fn entity_type(&self) -> OsEntityType {
        self.m_type
    }

    /**
     * Returns the optionally stored `OsEntity` name
     */
    pub fn name(&self) -> Option<&'a str> {
        self.m_name
    }

    /**
     * Returns the reference to te `OsEntityConfigFlags`
     */
    pub fn flags(&self) -> &OsEntityConfigFlags {
        &self.m_flags
    }
}

impl<'a> RawOsEntityConfig<'a> /* Setters */ {
    /**
     * Sets an explicit `OsEntityId`
     */
    pub fn set_id(&mut self, raw_id: OsEntityId) {
        self.m_id = Some(raw_id);
    }

    /**
     * Sets the `OsEntityType`    
     */
    pub fn set_entity_type(&mut self, ent_type: OsEntityType) {
        self.m_type = ent_type;
    }

    /**
     * Sets an explicit `OsEntity` name
     */
    pub fn set_name(&mut self, name: &'a str) {
        self.m_name = Some(name);
    }

    /**
     * Returns the mutable reference to te `OsEntityConfigFlags`
     */
    pub fn flags_mut(&mut self) -> &mut OsEntityConfigFlags {
        &mut self.m_flags
    }
}

impl<'a> TAsSysCallPtr for RawOsEntityConfig<'a> {
    /* No methods to implement */
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

impl TBitFlagsValues for OsEntityConfigBits {
}
