/*! `Object` information related data structures */

use helps::str::{
    copy_str_to_u8_buf,
    u8_slice_to_str_slice
};

use crate::{
    entity::OsEntityId,
    limit::VFS_NAME_LEN_MAX,
    obj::{
        grants::RawObjGrants,
        types::ObjType,
        uses::ObjUseBits
    },
    task::TaskId,
    time::RawInstant
};

/**
 * Userland/Kernel interchangeable `Object` metadata information
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct RawObjInfo {
    m_type: ObjType,
    m_ref_count: usize,

    /* name related field */
    m_has_name: bool,
    m_name_id: u64,
    m_name_buffer: [u8; VFS_NAME_LEN_MAX],
    m_name_len: usize,
    m_links: u32,

    /* data memory consumption related field */
    m_data_block_size: usize,
    m_data_blocks_used: usize,
    m_data_bytes_used: usize,

    /* protection related fields */
    m_os_user_id: OsEntityId,
    m_os_group_id: OsEntityId,
    m_prot_grants: RawObjGrants,

    /* timestamps related fields */
    m_creat_inst: RawInstant,
    m_last_data_access_inst: RawInstant,
    m_last_data_modify_inst: RawInstant,
    m_last_info_access_inst: RawInstant,
    m_last_info_modify_inst: RawInstant
}

impl RawObjInfo {
    /**
     * Convenience constant to create zero-filled vfs name buffers
     */
    const EMPTY_VFS_NAME: [u8; VFS_NAME_LEN_MAX] = [0; VFS_NAME_LEN_MAX];

    /**
     * Constructs a `RawObjInfo` filled with the given parameters
     */
    pub fn new(obj_type: ObjType,
               ref_count: usize,
               name_id: u64,
               name: Option<&str>,
               links: u32,
               data_block_size: usize,
               data_blocks_used: usize,
               data_bytes_used: usize,
               os_user_id: OsEntityId,
               os_group_id: OsEntityId,
               prot_grants: RawObjGrants,
               creat_inst: RawInstant,
               last_data_access_inst: RawInstant,
               last_data_modify_inst: RawInstant,
               last_info_access_inst: RawInstant,
               last_info_modify_inst: RawInstant)
               -> Self {
        Self { m_type: obj_type,
               m_ref_count: ref_count,
               m_has_name: name.is_some(),
               m_name_id: name_id,
               m_name_buffer: name.map_or(Self::EMPTY_VFS_NAME, |str_buf| {
                                      let mut name_buffer = Self::EMPTY_VFS_NAME;
                                      copy_str_to_u8_buf(&mut name_buffer, str_buf);

                                      name_buffer
                                  }),
               m_name_len: name.map_or(0, str::len),
               m_links: links,
               m_data_block_size: data_block_size,
               m_data_blocks_used: data_blocks_used,
               m_data_bytes_used: data_bytes_used,
               m_os_user_id: os_user_id,
               m_os_group_id: os_group_id,
               m_prot_grants: prot_grants,
               m_creat_inst: creat_inst,
               m_last_data_access_inst: last_data_access_inst,
               m_last_data_modify_inst: last_data_modify_inst,
               m_last_info_access_inst: last_info_access_inst,
               m_last_info_modify_inst: last_info_modify_inst }
    }

    /**
     * Returns the `ObjType` of the `Object`
     */
    pub fn obj_type(&self) -> ObjType {
        self.m_type
    }

    /**
     * Returns the current amount of references which keep busy the `Object`
     * (i.e the `Thread`s which keep the `Object` open)
     */
    pub fn ref_count(&self) -> usize {
        self.m_ref_count
    }

    /**
     * Returns whether the object is a named `Object` or an anonymous one
     */
    pub fn has_name(&self) -> bool {
        self.m_has_name
    }

    /**
     * Returns the unique name identifier if the `Object` has name
     */
    pub fn name_id(&self) -> Option<u64> {
        if self.has_name() {
            Some(self.m_name_id)
        } else {
            None
        }
    }

    /**
     * Returns the human readable UTF-8 name of the `Object` if any
     */
    pub fn name(&self) -> Option<&str> {
        if self.has_name() {
            Some(u8_slice_to_str_slice(&self.m_name_buffer[..self.m_name_len]))
        } else {
            None
        }
    }

    /**
     * Sets a new name for the `Object`.
     *
     * Asserts on `self.has_name()`
     */
    pub fn set_name(&mut self, name: &str) {
        assert!(self.has_name(), "Tried to set a name for an anonymous Object");

        copy_str_to_u8_buf(&mut self.m_name_buffer, name);
    }

    /**
     * Returns the number of links to the name of the `Object` (i.e the
     * parent directory and the links which links this object)
     */
    pub fn links(&self) -> Option<u32> {
        if self.has_name() {
            Some(self.m_links)
        } else {
            None
        }
    }

    /**
     * Returns the allocation block unit used by the back device to store
     * the `Object` data
     */
    pub fn data_block_size(&self) -> usize {
        self.m_data_block_size
    }

    /**
     * Returns the amount of allocation block unit used to store the current
     * amount of data
     */
    pub fn data_blocks_used(&self) -> usize {
        self.m_data_blocks_used
    }

    /**
     * Returns the currently real amount of bytes used to store the `Object`
     * data
     */
    pub fn data_bytes_used(&self) -> usize {
        self.m_data_bytes_used
    }

    /**
     * Returns the `OsEntityId` of the `OsUser` which owns the `Object`
     */
    pub fn os_user(&self) -> OsEntityId {
        self.m_os_user_id
    }

    /**
     * Sets the `OsEntityId` of the `OsUser` which owns the `Object`
     */
    pub fn set_os_user(&mut self, os_user_id: OsEntityId) {
        self.m_os_user_id = os_user_id;
    }

    /**
     * Returns the `OsEntityId` of the `OsGroup` which owns the `Object`
     */
    pub fn os_group(&self) -> OsEntityId {
        self.m_os_group_id
    }

    /**
     * Sets the `OsEntityId` of the `OsGroup` which owns the `Object`
     */
    pub fn set_os_group(&mut self, os_group_id: OsEntityId) {
        self.m_os_group_id = os_group_id;
    }

    /**
     * Returns the reference to the `RawObjGrants`
     */
    pub fn prot_grants(&self) -> &RawObjGrants {
        &self.m_prot_grants
    }

    /**
     * Returns the mutable reference to the `RawObjGrants`
     */
    pub fn prot_grants_mut(&mut self) -> &mut RawObjGrants {
        &mut self.m_prot_grants
    }

    /**
     * Returns the `Object` creation `RawInstant`
     */
    pub fn creat_inst(&self) -> RawInstant {
        self.m_creat_inst
    }

    /**
     * Sets the `Object` creation `RawInstant`
     */
    pub fn set_creat_inst(&mut self, new_inst: RawInstant) {
        self.m_creat_inst = new_inst;
    }

    /**
     * Returns the `Object` last data access `RawInstant`
     */
    pub fn last_data_access_inst(&self) -> RawInstant {
        self.m_last_data_access_inst
    }

    /**
     * Sets the `Object` last data access `RawInstant`
     */
    pub fn set_last_data_access_inst(&mut self, new_inst: RawInstant) {
        self.m_last_data_access_inst = new_inst;
    }

    /**
     * Returns the `Object` last info access `RawInstant`
     */
    pub fn last_data_modify_inst(&self) -> RawInstant {
        self.m_last_data_modify_inst
    }

    /**
     * Sets the `Object` last info access `RawInstant`
     */
    pub fn set_last_data_modify_inst(&mut self, new_inst: RawInstant) {
        self.m_last_data_modify_inst = new_inst;
    }

    /**
     * Returns the `Object` last data modification `RawInstant`
     */
    pub fn last_info_access_inst(&self) -> RawInstant {
        self.m_last_info_access_inst
    }

    /**
     * Sets the `Object` last data modification `RawInstant`
     */
    pub fn set_last_info_access_inst(&mut self, new_inst: RawInstant) {
        self.m_last_info_access_inst = new_inst;
    }

    /**
     * Returns the `Object` last info modification `RawInstant`
     */
    pub fn last_info_modify_inst(&self) -> RawInstant {
        self.m_last_info_modify_inst
    }

    /**
     * Sets the `Object` last info modification `RawInstant`
     */
    pub fn set_last_info_modify_inst(&mut self, new_inst: RawInstant) {
        self.m_last_info_modify_inst = new_inst;
    }

    /**
     * Returns `&self` as usize pointer value
     */
    pub fn as_syscall_ptr(&self) -> usize {
        self as *const Self as usize
    }
}

impl Default for RawObjInfo {
    fn default() -> Self {
        Self { m_type: ObjType::default(),
               m_ref_count: 0,
               m_has_name: false,
               m_name_id: 0,
               m_name_buffer: Self::EMPTY_VFS_NAME,
               m_name_len: 0,
               m_links: 0,
               m_data_block_size: 0,
               m_data_blocks_used: 0,
               m_data_bytes_used: 0,
               m_os_user_id: 0,
               m_os_group_id: 0,
               m_prot_grants: RawObjGrants::new_zero(),
               m_creat_inst: RawInstant::default(),
               m_last_data_access_inst: RawInstant::default(),
               m_last_data_modify_inst: RawInstant::default(),
               m_last_info_access_inst: RawInstant::default(),
               m_last_info_modify_inst: RawInstant::default() }
    }
}

/**
 * Data container with usage instant related to an `Object`
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct ObjUseInstant {
    m_obj_use: ObjUseBits,
    m_thread_id: TaskId,
    m_use_start: RawInstant
}

impl ObjUseInstant {
    /**
     * Constructs an `ObjUseInstant` with the given parameters
     */
    pub fn new(obj_use: ObjUseBits, thread_id: TaskId, use_start: RawInstant) -> Self {
        Self { m_obj_use: obj_use,
               m_thread_id: thread_id,
               m_use_start: use_start }
    }

    /**
     * Returns the `ObjUseBits` performed by the referred user
     */
    pub fn obj_use(&self) -> ObjUseBits {
        self.m_obj_use
    }

    /**
     * Returns the raw thread identifier that have performed the usage
     */
    pub fn thread_id(&self) -> TaskId {
        self.m_thread_id
    }

    /**
     * Returns the `RawInstant` of the operation
     */
    pub fn use_start(&self) -> RawInstant {
        self.m_use_start
    }
}
