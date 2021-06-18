/*! `Object` metadata information */

use crate::{
    bits::obj::types::ObjType,
    caller::Result,
    objs::{
        info::{
            mem::ObjMemInfo,
            name::ObjNameInfo,
            owner::ObjOwner,
            time::ObjTimeInfo
        },
        object::{
            ObjId,
            Object
        }
    }
};

/**
 * Metadata descriptor common to all the `Object` Implementations.
 *
 * This struct takes the place of the old-style Unix-like's `stat struct`
 * with a little improvement:
 * * There is no more need of tons of single system calls to update small
 *   piece of obj's metadata, there is only one cumulative info update call
 *   that overwrites the previous one
 * * Using `RAII` the information are updated when the struct goes out of
 *   scope or when called explicitly `ObjInfo::update()`
 */
#[derive(Debug, Default)]
pub struct ObjInfo<T>
    where T: Object {
    m_obj: ObjId,
    m_type: ObjType,
    m_name_info: Option<ObjNameInfo>,
    m_mem_info: ObjMemInfo,
    m_owner: ObjOwner<T>,
    m_time_info: ObjTimeInfo,
    m_to_update: bool
}

#[cfg(feature = "enable_kernel_methods")]
impl<T> ObjInfo<T> where T: Object {
    /**
     * Constructs an `ObjInfo` with the given data
     */
    pub fn new(obj_type: ObjType,
               name_info: Option<ObjNameInfo>,
               mem_info: ObjMemInfo,
               owner: ObjOwner<T>,
               time_info: ObjTimeInfo)
               -> Self {
        Self { m_obj: ObjId::default(),
               m_type: obj_type,
               m_name_info: name_info,
               m_mem_info: mem_info,
               m_owner: owner,
               m_time_info: time_info,
               m_to_update: false }
    }
}

impl<T> ObjInfo<T> where T: Object {
    /**
     * Overwrites the stored `ObjId` cloning the given one
     */
    pub(crate) fn set_obj(&mut self, obj: &ObjId) {
        self.m_obj = obj.clone()
    }

    /**
     * Returns the concrete `ObjType` of the obj
     */
    pub fn obj_type(&self) -> ObjType {
        self.m_type
    }

    /**
     * Returns whether this `Object` has a VFS name
     */
    pub fn has_name(&self) -> bool {
        self.m_name_info.is_some()
    }

    /**
     * Returns the `ObjNameInfo` reference if any
     */
    pub fn name_info(&self) -> Option<&ObjNameInfo> {
        self.m_name_info.as_ref()
    }

    /**
     * Returns the mutable `ObjNameInfo` reference if any
     */
    pub fn name_info_mut(&mut self) -> Option<&mut ObjNameInfo> {
        if self.has_name() {
            self.m_to_update = true;
        }
        self.m_name_info.as_mut()
    }

    /**
     * Returns the `ObjMemInfo` reference
     */
    pub fn mem_info(&self) -> &ObjMemInfo {
        &self.m_mem_info
    }

    /**
     * Returns the `ObjOwner` reference
     */
    pub fn owner(&self) -> &ObjOwner<T> {
        &self.m_owner
    }

    /**
     * Returns the mutable `ObjOwner` reference
     */
    pub fn owner_mut(&mut self) -> &mut ObjOwner<T> {
        self.m_to_update = true;
        &mut self.m_owner
    }

    /**
     * Returns the `ObjTimeInfo` reference
     */
    pub fn time_info(&self) -> &ObjTimeInfo {
        &self.m_time_info
    }

    /**
     * Returns the mutable `ObjTimeInfo` reference
     */
    pub fn time_info_mut(&mut self) -> &mut ObjTimeInfo {
        self.m_to_update = true;
        &mut self.m_time_info
    }

    /**
     * Commits the changed fields to the Kernel.
     *
     * This operation can be performed only if the referenced obj is
     * valid and the caller have information write grants.
     */
    pub fn update(&mut self) -> Result<()> {
        if self.m_to_update {
            self.m_obj.update_info(self).map(|_| self.reset_update())
        } else {
            /* the obj is not modified, so return okay anyway */
            Ok(())
        }
    }

    /**
     * Discard to update fields
     */
    pub fn reset_update(&mut self) -> () {
        self.m_to_update = false;
    }
}

impl<T> Drop for ObjInfo<T> where T: Object {
    /**
     * Calls `self.update()` to update the modified stuffs
     */
    fn drop(&mut self) {
        self.update()
            .unwrap_or_else(|err| panic!("Failed to update obj infos: cause: {}", err))
    }
}
