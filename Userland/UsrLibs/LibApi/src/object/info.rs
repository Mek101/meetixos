/*! `Object` metadata information structures */

use core::ops::{
    Deref,
    DerefMut
};

use api_data::object::info::RawObjInfo;

use crate::{
    entity::{
        impls::{
            group::OsGroup,
            user::OsUser
        },
        OsEntity
    },
    kern_handle::Result,
    object::{
        grants::ObjGrants,
        ObjHandle,
        Object
    }
};

/**
 * High-level type-safe `Object`'s metadata
 */
#[derive(Debug)]
#[derive(Default)]
pub struct ObjInfo<T>
    where T: Object {
    m_raw_info: RawObjInfo,
    m_obj_handle: ObjHandle,
    m_obj_prot_grants: ObjGrants<T>,
    m_modified: bool
}

impl<T> ObjInfo<T> where T: Object /* Constructors */ {
    /**
     * Constructs a `ObjInfo` from the given arguments
     */
    pub(crate) fn new(raw_obj_info: RawObjInfo, obj_handle: ObjHandle) -> Self {
        Self { m_raw_info: raw_obj_info,
               m_obj_handle: obj_handle,
               m_obj_prot_grants: ObjGrants::from(raw_obj_info.prot_grants().clone()),
               m_modified: false }
    }
}

impl<T> ObjInfo<T> where T: Object /* Methods */ {
    /**
     * Updates back the modified metadata for the `Object` which originates
     * this
     */
    pub fn update(&mut self) -> Result<()> {
        if self.m_modified {
            *self.m_raw_info.prot_grants_mut() = *self.m_obj_prot_grants;
            self.m_obj_handle.update_info(&mut self.m_raw_info)
        } else {
            Ok(())
        }
    }
}

impl<T> ObjInfo<T> where T: Object /* Setters */ {
    /**
     * Sets the `OsUser` which owns the `Object`
     */
    pub fn set_os_user(&mut self, os_user: &OsUser) {
        self.m_modified = true;
        self.m_raw_info
            .set_os_user(os_user.os_entity_handle().kern_handle().raw_handle());
    }

    /**
     * Sets the `OsGroup` which owns the `Object`
     */
    pub fn set_os_group(&mut self, os_group: &OsGroup) {
        self.m_modified = true;
        self.m_raw_info
            .set_os_group(os_group.os_entity_handle().kern_handle().raw_handle());
    }

    /**
     * Returns the reference to the `ObjGrants`
     */
    pub fn prot_grants(&self) -> &ObjGrants<T> {
        &self.m_obj_prot_grants
    }

    /**
     * Returns the mutable reference to the `ObjGrants`
     */
    pub fn prot_grants_mut(&mut self) -> &mut ObjGrants<T> {
        self.m_modified = true;
        &mut self.m_obj_prot_grants
    }
}

impl<T> Deref for ObjInfo<T> where T: Object {
    type Target = RawObjInfo;

    fn deref(&self) -> &Self::Target {
        &self.m_raw_info
    }
}

impl<T> DerefMut for ObjInfo<T> where T: Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.m_modified = true;
        &mut self.m_raw_info
    }
}
