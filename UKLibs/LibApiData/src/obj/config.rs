/*! `Object` configuration */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use bits::flags::{
    BitFlags,
    BitFlagsValues
};

use crate::obj::{
    grants::RawObjGrants,
    types::ObjType
};

/**
 * Internally used configuration flags
 */
pub type ObjConfigFlags = BitFlags<usize, ObjConfigBits>;

/**
 * Userland/Kernel interchangeable `Object` configuration
 */
pub struct RawObjConfig<'a> {
    m_type: ObjType,
    m_path: Option<&'a str>,
    m_flags: ObjConfigFlags,
    m_grants: RawObjGrants,
    m_data_size: Option<usize>
}

impl<'a> RawObjConfig<'a> {
    /**
     * Constructs and empty `RawObjConfig`
     */
    pub fn new() -> Self {
        Self { m_flags: ObjConfigFlags::new_zero(),
               m_data_size: None,
               m_grants: RawObjGrants::new_zero(),
               m_type: ObjType::default(),
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
    pub fn path(&self) -> Option<&'a str> {
        self.m_path
    }

    /**
     * Sets the path to the `Object` to open
     */
    pub fn set_path(&mut self, path: &'a str) {
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
pub enum ObjConfigBits {
    Creat,
    Read,
    Write,
    Exec,
    Exclusive
}

impl BitFlagsValues for ObjConfigBits {
}
