/*! # Operating System User
 *
 * Implements the abstraction of the operating system user
 */

use os::sysc::{
    codes::KernOSUserFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::ent::OSEntityType,
    caller::{
        KernCaller,
        Result
    },
    ents::{
        impls::OSGroup,
        OSEntity,
        OSEntityId
    }
};

/** # Operating System User Entity
 *
 * Specializes the [`OSEntityId`] to act like a logged user with his class
 * of permissions over the VFS objects that owns
 *
 * [`OSEntityId`]: crate::ents::OSEntityId
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct OSUser(OSEntityId);

impl OSUser {
    /** # Obtains the `OSUser`'s groups
     *
     * Puts into the `groups` buffer the [`OSGroup`]s instances that this
     * `OSUser` joins
     *
     * [`OSGroup`]: crate::ents::impls::OSGroup
     */
    pub fn groups<'a>(&self, groups: &'a mut [OSGroup]) -> Result<&'a [OSGroup]> {
        self.kern_call_2(KernFnPath::OSUser(KernOSUserFnId::Groups),
                         groups.as_mut_ptr() as usize,
                         groups.len())
            .map(move |count| &groups[..count])
    }
}

impl KernCaller for OSUser {
    /** Returns the raw identifier of the underling [`OSEntityId`]
     *
     * [`OSEntityId`]: crate::ents::OSEntityId
     */
    fn caller_handle_bits(&self) -> u32 {
        self.0.caller_handle_bits()
    }
}

impl From<OSEntityId> for OSUser {
    /** Performs the conversion.
     */
    fn from(ent: OSEntityId) -> Self {
        Self(ent)
    }
}

impl OSEntity for OSUser {
    /** The value of the [`OSEntityType`] that matches the implementation
     *
     * [`OSEntityType`]: crate::bits::ent::types::OSEntityType
     */
    const TYPE: OSEntityType = OSEntityType::User;

    /** Returns the immutable reference to the underling [`OSEntityId`]
     * instance
     *
     * [`OSEntityId`]: crate::ents::OSEntityId
     */
    fn os_entity_handle(&self) -> &OSEntityId {
        &self.0
    }
}
