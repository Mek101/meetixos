/*! Operating System user ent */

use os::sysc::{
    codes::KernOsUserFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::ent::types::OsEntityType,
    caller::{
        KernCaller,
        Result
    },
    ents::{
        entity::{
            OsEntity,
            OsEntityId
        },
        impls::group::OsGroup
    }
};

/**
 * Human (or not) user of the system that could be logged in, or not, which
 * have grants over the `Object`s that owns
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct OsUser(OsEntityId);

impl OsUser {
    /**
     * Obtains the `OSUser`'s groups
     */
    pub fn groups<'a>(&self, groups: &'a mut [OsGroup]) -> Result<&'a [OsGroup]> {
        self.kern_call_2(KernFnPath::OsUser(KernOsUserFnId::Groups),
                         groups.as_mut_ptr() as usize,
                         groups.len())
            .map(move |count| &groups[..count])
    }
}

impl KernCaller for OsUser {
    fn caller_handle_bits(&self) -> u32 {
        self.0.caller_handle_bits()
    }
}

impl From<OsEntityId> for OsUser {
    fn from(ent: OsEntityId) -> Self {
        Self(ent)
    }
}

impl OsEntity for OsUser {
    const TYPE: OsEntityType = OsEntityType::User;

    fn os_entity_handle(&self) -> &OsEntityId {
        &self.0
    }
}
