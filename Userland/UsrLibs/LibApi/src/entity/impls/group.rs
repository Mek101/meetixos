/*! Group of `OsUser`s */

use alloc::vec::Vec;

use api_data::{
    entity::{
        types::OsEntityType,
        OsEntityId
    },
    sys::{
        codes::KernOsGroupFnId,
        fn_path::KernFnPath
    }
};

use crate::{
    entity::{
        impls::user::OsUser,
        OsEntityHandle,
        TOsEntity
    },
    kern_handle::Result
};

/**
 * Group of `OsUser`s which have in common a class of permissions over the
 * `Object`s that the group owns
 */
#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct OsGroup {
    m_ent_handle: OsEntityHandle
}

impl OsGroup /* Methods */ {
    /**
     * Adds the given `OsUser` to this `OsGroup`
     *
     * The inserted user will have group permissions too for resources owned
     * by this `OsGroup`.
     *
     * This call affects only the runtime tables of the Kernel, update
     * the `/MeetiX/Configs/users_groups.xml` file to make it permanent
     */
    pub fn add_user(&self, os_user: &OsUser) -> Result<()> {
        self.os_entity_handle()
            .kern_handle()
            .inst_kern_call_1(KernFnPath::OsGroup(KernOsGroupFnId::AddUser),
                              os_user.os_entity_handle().kern_handle().raw_handle()
                              as usize)
            .map(|_| ())
    }

    /**
     * Returns the `Vec` of `OsEntityId`s of the `OsUsers` which joins this
     * `OsGroup`
     */
    pub fn users_ids(&self) -> Result<Vec<OsEntityId>> {
        let mut users_ids_vec = Vec::with_capacity(self.users_count()?);

        self.os_entity_handle()
            .kern_handle()
            .inst_kern_call_2(KernFnPath::OsGroup(KernOsGroupFnId::UsersIds),
                              users_ids_vec.as_mut_ptr() as usize,
                              users_ids_vec.capacity())
            .map(|users_count| {
                unsafe {
                    users_ids_vec.set_len(users_count);
                }
                users_ids_vec
            })
    }

    /**
     * Returns the amount of `OsUsers` which joins this `OsGroup`
     */
    pub fn users_count(&self) -> Result<usize> {
        self.os_entity_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::OsGroup(KernOsGroupFnId::UsersCount))
    }
}

impl From<OsEntityHandle> for OsGroup {
    fn from(ent_handle: OsEntityHandle) -> Self {
        Self { m_ent_handle: ent_handle }
    }
}

impl TOsEntity for OsGroup {
    const TYPE: OsEntityType = OsEntityType::Group;

    fn os_entity_handle(&self) -> &OsEntityHandle {
        &self.m_ent_handle
    }
}
