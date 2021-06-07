/*! Operating System user entity */

use os::sysc::{
    codes::KernOSUserFnId,
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
        impls::group::OSGroup
    }
};

/**
 * Human (or not) user of the system that could be logged in, or not, which
 * have grants over the `Object`s that owns
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct OSUser(OSEntityId);

impl OSUser {
    /**
     * Obtains the `OSUser`'s groups
     */
    pub fn groups<'a>(&self, groups: &'a mut [OSGroup]) -> Result<&'a [OSGroup]> {
        self.kern_call_2(KernFnPath::OSUser(KernOSUserFnId::Groups),
                         groups.as_mut_ptr() as usize,
                         groups.len())
            .map(move |count| &groups[..count])
    }
}

impl KernCaller for OSUser {
    fn caller_handle_bits(&self) -> u32 {
        self.0.caller_handle_bits()
    }
}

impl From<OSEntityId> for OSUser {
    fn from(ent: OSEntityId) -> Self {
        Self(ent)
    }
}

impl OSEntity for OSUser {
    const TYPE: OSEntityType = OSEntityType::User;

    fn os_entity_handle(&self) -> &OSEntityId {
        &self.0
    }
}
