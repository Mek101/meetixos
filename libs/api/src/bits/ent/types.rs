/*! # `OSEntity` Types
 *
 * Implements the variants that identifies the various [`OSEntityId`]
 * based implementations
 *
 * [`OSEntityId`]: crate::ents::OSEntityId
 */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/** # `OSEntity` Types
 *
 * Lists the available object types represented by an [`OSEntityId`]
 *
 * [`OSEntityId`]: crate::ents::OSEntityId
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum OSEntityType {
    /** No real uses, used as default or error value
     */
    Unknown,

    /** Identifies an [`OSUser`] entity
     *
     * [`OSUser`]: crate::ents::impls::OSUser
     */
    User,

    /** Identifies an [`OSGroup`] entity
     *
     * [`OSGroup`]: crate::ents::impls::OSGroup.html
     */
    Group
}
