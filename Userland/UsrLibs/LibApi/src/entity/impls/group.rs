/*! Group of `OSUser`s */

use crate::{
    entity::{
        impls::user::OsUser,
        OsEntity,
        OsEntityHandle
    },
    handle::Result
};
use api_data::{
    ent::types::OsEntityType,
    sys::{
        codes::KernOsGroupFnId,
        fn_path::KernFnPath
    }
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
pub struct OsGroup {
    m_ent_handle: OsEntityHandle
}

impl OsGroup {
    /**
     * Adds the given `OsUser` to this `OsGroup`
     *
     * The inserted user will have group permissions too for resources owned
     * by this `OsGroup`.
     *
     * This call affects only the runtime tables of the Kernel, update the
     * `/MeetiX/Configs/users_groups.xml` file to make it permanent
     */
    pub fn add_user(&self, os_user: &OsUser) -> Result<()> {
        self.m_ent_handle
            .m_handle
            .inst_kern_call_1(KernFnPath::OsGroup(KernOsGroupFnId::AddUser),
                              os_user.os_entity_handle().m_handle.raw_handle() as usize)
            .map(|_| ())
    }
}

impl From<OsEntityHandle> for OsGroup {
    fn from(ent_handle: OsEntityHandle) -> Self {
        Self { m_ent_handle: ent_handle }
    }
}

impl OsEntity for OsGroup {
    const TYPE: OsEntityType = OsEntityType::Group;

    fn os_entity_handle(&self) -> &OsEntityHandle {
        &self.m_ent_handle
    }
}
