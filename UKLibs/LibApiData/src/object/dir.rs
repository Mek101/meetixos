/*! `Dir` specific data structures */

use core::fmt;

use helps::str::{
    copy_str_to_u8_buf,
    u8_slice_to_str_slice
};

use crate::{
    limit::VFS_NAME_LEN_MAX,
    object::types::ObjType,
    sys::AsSysCallPtr
};

/**
 * Child unit inside a `Dir` object
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct DirEntry {
    m_name: [u8; VFS_NAME_LEN_MAX],
    m_name_len: usize,
    m_obj_type: ObjType
}

impl DirEntry /* Constructors */ {
    /**
     * Constructs a new `DirEntry` with the given values
     */
    pub fn new(entry_name: &str, obj_type: ObjType) -> Self {
        let mut name_buffer = [0; VFS_NAME_LEN_MAX];
        copy_str_to_u8_buf(&mut name_buffer, entry_name);

        Self { m_name: name_buffer,
               m_name_len: entry_name.len(),
               m_obj_type: obj_type }
    }
}

impl DirEntry /* Getters */ {
    /**
     * Returns the name of the child as string slice
     */
    pub fn name(&self) -> &str {
        u8_slice_to_str_slice(&self.m_name[..self.m_name_len])
    }

    /**
     * Returns the `ObjType` of the child
     */
    pub fn obj_type(&self) -> ObjType {
        self.m_obj_type
    }
}

impl AsSysCallPtr for DirEntry {
    /* No methods to implement */
}

impl Default for DirEntry {
    fn default() -> Self {
        Self { m_name: [0; VFS_NAME_LEN_MAX],
               m_name_len: 0,
               m_obj_type: ObjType::default() }
    }
}

impl fmt::Display for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.obj_type(), self.name())
    }
}
