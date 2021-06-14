/*! `Object` name information descriptor */

use helps::str::{
    copy_str_to_u8_buf,
    u8_ptr_to_str_slice
};
use os::limits::VFS_NAME_LEN_MAX;

/**
 * Contains various information that are related to the VFS representation
 * of an `Object` based type
 */
#[derive(Debug, Copy, Clone)]
pub struct ObjNameInfo {
    m_name_id: u64,
    m_name_buf: [u8; VFS_NAME_LEN_MAX],
    m_name_len: usize,
    m_links: u32
}

#[cfg(feature = "enable_kernel_methods")]
impl ObjNameInfo {
    /**
     * Constructs an `ObjNameInfo` with the given parameters
     */
    pub fn new(name_id: u64, name: &str, links: u32) -> Self {
        let mut buf = [0; VFS_NAME_LEN_MAX];
        copy_str_to_u8_buf(&mut buf, name);

        Self { m_name_id: name_id,
               m_name_buf: buf,
               m_name_len: name.len(),
               m_links: links }
    }
}

impl ObjNameInfo {
    /**
     * Returns the name unique identifier
     */
    pub fn name_id(&self) -> u64 {
        self.m_name_id
    }

    /**
     * Returns the name as encoded string slice
     */
    pub fn name(&self) -> &str {
        u8_ptr_to_str_slice(self.m_name_buf.as_ptr(), self.m_name_len)
    }

    /**
     * The new name will be applied to the object when `ObjInfo::update()`
     * will be called
     */
    pub fn set_name(&mut self, new_name: &str) {
        copy_str_to_u8_buf(&mut self.m_name_buf, new_name)
    }

    /**
     * Returns the number of `Link`s that refers this object
     */
    pub fn links_count(&self) -> u32 {
        self.m_links
    }
}
