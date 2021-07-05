/*! `Object` configuration */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use bits::flags::{
    BitFlags,
    BitFlagsValues
};

use crate::{
    obj::{
        grants::RawObjGrants,
        types::ObjType
    },
    path::PathComponent,
    sys::AsSysCallPtr
};

/**
 * Internally used configuration flags
 */
pub type ObjConfigFlags = BitFlags<usize, ObjConfigBits>;

/**
 * Userland/Kernel interchangeable `Object` configuration
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct RawObjConfig<'a> {
    m_type: ObjType,
    m_path: Option<&'a [PathComponent]>,
    m_flags: ObjConfigFlags,
    m_grants: RawObjGrants,
    m_data_size: Option<usize>
}

impl<'a> RawObjConfig<'a> {
    /**
     * Constructs and empty `RawObjConfig`
     */
    pub fn new(obj_type: ObjType, is_creat: bool) -> Self {
        /* construct the init */
        let config_flags = if is_creat {
            ObjConfigFlags::new_zero() | ObjConfigBits::Creat
        } else {
            ObjConfigFlags::new_zero()
        };

        Self { m_flags: config_flags,
               m_data_size: None,
               m_grants: RawObjGrants::new_zero(),
               m_type: obj_type,
               m_path: None }
    }

    /**
     * Returns the `ObjType`
     */
    pub fn obj_type(&self) -> ObjType {
        self.m_type
    }

    /**
     * Sets the `ObjType`    
     */
    pub fn set_obj_type(&mut self, obj_type: ObjType) {
        self.m_type = obj_type;
    }

    /**
     * Returns the optionally stored path
     */
    pub fn path(&self) -> Option<&'a [PathComponent]> {
        self.m_path
    }

    /**
     * Sets the path to the `Object` to open
     */
    pub fn set_path(&mut self, path: &'a [PathComponent]) {
        self.m_path = Some(path);
    }

    /**
     * Returns the reference to te `ObjConfigFlags`
     */
    pub fn flags(&self) -> &ObjConfigFlags {
        &self.m_flags
    }

    /**
     * Returns the mutable reference to te `ObjConfigFlags`
     */
    pub fn flags_mut(&mut self) -> &mut ObjConfigFlags {
        &mut self.m_flags
    }

    /**
     * Returns the reference to te `RawObjGrants`
     */
    pub fn grants(&self) -> &RawObjGrants {
        &self.m_grants
    }

    /**
     * Returns the mutable reference to te `RawObjGrants`
     */
    pub fn grants_mut(&mut self) -> &mut RawObjGrants {
        &mut self.m_grants
    }

    /**
     * Returns the optional truncation size
     */
    pub fn data_size(&self) -> Option<usize> {
        self.m_data_size
    }

    /**
     * Sets the truncation size for the `Object` to open
     */
    pub fn set_data_size(&mut self, data_size: usize) {
        self.m_data_size = Some(data_size);
    }
}

impl<'a> AsSysCallPtr for RawObjConfig<'a> {
    /* No methods to implement */
}

/**
 * Lists the internal `RawOsEntityConfig` flags
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum ObjConfigBits {
    /**
     * Enabled when called `Object::creat()`
     */
    Creat,

    /**
     * Enables data-read operations on the `Object`
     */
    Read,

    /**
     * Enables data-write operations on the `Object`
     */
    Write,

    /**
     * Enables data-exec operations on the `Object`
     */
    Exec,

    /**
     * Ensures that the `Object` is opened by one thread a time
     */
    Exclusive
}

impl BitFlagsValues for ObjConfigBits {
}
