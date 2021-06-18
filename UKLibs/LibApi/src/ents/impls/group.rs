/*! `OSUser`'s Group */

use os::sysc::{
    codes::KernOsGroupFnId,
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
        impls::user::OsUser
    }
};

/**
 * Group of `OSUser`s which have in common a class of permissions over the
 * `Object`s that the group owns
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct OsGroup(OsEntityId);

impl OsGroup {
    /**
     * Adds the given `OSUser` to this `OSGroup`
     *
     * The inserted user will have group permissions too for resources owned
     * by this `OSGroup`.
     *
     * This call affects only the runtime tables of the Kernel, update the
     * `/MeetiX/Configs/users_groups.xml` file to make it permanent
     */
    pub fn add_user(&self, user: &OsUser) -> Result<()> {
        self.kern_call_1(KernFnPath::OsGroup(KernOsGroupFnId::AddUser),
                         user.os_entity_handle().as_raw_usize())
            .map(|_| ())
    }
}

impl KernCaller for OsGroup {
    fn caller_handle_bits(&self) -> u32 {
        self.0.caller_handle_bits()
    }
}

impl From<OsEntityId> for OsGroup {
    fn from(ent: OsEntityId) -> Self {
        Self(ent)
    }
}

impl OsEntity for OsGroup {
    const TYPE: OsEntityType = OsEntityType::Group;

    fn os_entity_handle(&self) -> &OsEntityId {
        &self.0
    }
}
