/*! `OSUser`'s Group */

use os::sysc::{
    codes::KernOSGroupFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::ent::types::OSEntityType,
    caller::{
        KernCaller,
        Result
    },
    ents::{
        entity::{
            OSEntity,
            OSEntityId
        },
        impls::user::OSUser
    }
};

/**
 * Group of `OSUser`s which have in common a class of permissions over the
 * `Object`s that the group owns
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct OSGroup(OSEntityId);

impl OSGroup {
    /**
     * Adds the given `OSUser` to this `OSGroup`
     *
     * The inserted user will have group permissions too for resources owned
     * by this `OSGroup`.
     *
     * This call affects only the runtime tables of the Kernel, update the
     * `/MeetiX/Configs/users_groups.xml` file to make it permanent
     */
    pub fn add_user(&self, user: &OSUser) -> Result<()> {
        self.kern_call_1(KernFnPath::OSGroup(KernOSGroupFnId::AddUser),
                         user.os_entity_handle().as_raw_usize())
            .map(|_| ())
    }
}

impl KernCaller for OSGroup {
    fn caller_handle_bits(&self) -> u32 {
        self.0.caller_handle_bits()
    }
}

impl From<OSEntityId> for OSGroup {
    fn from(ent: OSEntityId) -> Self {
        Self(ent)
    }
}

impl OSEntity for OSGroup {
    const TYPE: OSEntityType = OSEntityType::Group;

    fn os_entity_handle(&self) -> &OSEntityId {
        &self.0
    }
}
