/*! # Operating System Users Group
 *
 * Implements the abstraction of the operating system users group
 */

use os::sysc::{
    codes::KernOSGroupFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::ent::OSEntityType,
    caller::{
        KernCaller,
        Result
    },
    ents::{
        impls::OSUser,
        OSEntity,
        OSEntityId
    }
};

/** # Users Group
 *
 * Specializes the [`OSEntityId`] to act like a set of [`OSUser`]s that have
 * in common a class of permissions over the VFS objects that owns as a
 * group
 *
 * [`OSEntityId`]: crate::ents::OSEntityId
 * [`OSUser`]: crate::ents::impls::OSUser
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct OSGroup(OSEntityId);

impl OSGroup {
    /** # Adds a `user` to this `OSGroup`
     *
     * The inserted user will have group permissions too for resources owned
     * by this group.
     *
     * This call affects only the runtime tables of the kernel, update the
     * `/MeetiX/Configs/users_groups.xml` file to make it permanent
     */
    pub fn add_user(&self, user: &OSUser) -> Result<()> {
        self.kern_call_1(KernFnPath::OSGroup(KernOSGroupFnId::AddUser),
                         user.os_entity_handle().as_raw_usize())
            .map(|_| ())
    }
}

impl KernCaller for OSGroup {
    /** Returns the raw identifier of the underling [`OSEntityId`]
     *
     * [`OSEntityId`]: crate::ents::OSEntityId
     */
    fn caller_handle_bits(&self) -> u32 {
        self.0.caller_handle_bits()
    }
}

impl From<OSEntityId> for OSGroup {
    /** Performs the conversion.
     */
    fn from(ent: OSEntityId) -> Self {
        Self(ent)
    }
}

impl OSEntity for OSGroup {
    /** The value of the [`OSEntityType`] that matches the implementation
     *
     * [`OSEntityType`]: crate::bits::ent::types::OSEntityType
     */
    const TYPE: OSEntityType = OSEntityType::Group;

    /** Returns the immutable reference to the underling [`OSEntityId`]
     * instance
     *
     * [`OSEntityId`]: crate::ents::OSEntityId
     */
    fn os_entity_handle(&self) -> &OSEntityId {
        &self.0
    }
}
