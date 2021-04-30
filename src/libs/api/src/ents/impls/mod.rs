/*! # Operating System Entities
 *
 * Exports the various implementations of the [`OSEntityId`] based objects
 *
 * [`OSEntityId`]: crate::ents::entity::OSEntityId
 */

pub use group::*;
pub use user::*;

mod group;
mod user;
