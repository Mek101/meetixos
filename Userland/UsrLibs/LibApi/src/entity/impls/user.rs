/*! Operating System user entity */

use api_data::{
    ent::types::OsEntityType,
    sys::{
        codes::KernOsUserFnId,
        fn_path::KernFnPath
    }
};

use crate::{
    entity::{
        impls::group::OsGroup,
        OsEntity,
        OsEntityHandle
    },
    handle::Result
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
pub struct OsUser {
    m_ent_handle: OsEntityHandle
}

impl OsUser {
    /**
     * Fills `groups_buf` with the joined `OsGroup`s by this `OsUser`
     */
    pub fn joined_groups<'a>(&self,
                             groups_buf: &'a mut [OsGroup])
                             -> Result<&'a [OsGroup]> {
        self.m_ent_handle
            .m_handle
            .inst_kern_call_1(KernFnPath::OsUser(KernOsUserFnId::Groups),
                              groups_buf.as_mut_ptr() as usize)
            .map(move |count| &groups_buf[..count])
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
