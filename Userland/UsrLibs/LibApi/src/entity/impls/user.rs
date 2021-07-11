/*! Operating System user entity */

use alloc::vec::Vec;

use api_data::{
    entity::{
        types::OsEntityType,
        OsEntityId
    },
    sys::{
        codes::KernOsUserFnId,
        fn_path::KernFnPath
    }
};

use crate::{
    entity::{
        OsEntity,
        OsEntityHandle
    },
    kern_handle::Result
};

/**
 * System user which could be logged in, or not, and which have grants over
 * the `Object`s that owns
 */
#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct OsUser {
    m_ent_handle: OsEntityHandle
}

impl OsUser /* Methods */ {
    /**
     * Returns a `Vec` with the `OsEntityId`s of the `OsGroups` joined by
     * this `OsUser`
     */
    pub fn joined_groups<'a>(&self) -> Result<Vec<OsEntityId>> {
        let mut groups_ids_vec = Vec::with_capacity(self.groups_count()?);

        self.os_entity_handle()
            .kern_handle()
            .inst_kern_call_2(KernFnPath::OsUser(KernOsUserFnId::GroupsIds),
                              groups_ids_vec.as_mut_ptr() as usize,
                              groups_ids_vec.capacity())
            .map(|joined_groups| {
                unsafe {
                    groups_ids_vec.set_len(joined_groups);
                }
                groups_ids_vec
            })
    }

    /**
     * Returns the amount `OsGroup` joined by this `OsUser`
     */
    pub fn groups_count(&self) -> Result<usize> {
        self.os_entity_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::OsUser(KernOsUserFnId::GroupsCount))
    }
}

impl From<OsEntityHandle> for OsUser {
    fn from(ent_handle: OsEntityHandle) -> Self {
        Self { m_ent_handle: ent_handle }
    }
}

impl OsEntity for OsUser {
    const TYPE: OsEntityType = OsEntityType::User;

    fn os_entity_handle(&self) -> &OsEntityHandle {
        &self.m_ent_handle
    }
}
