/*! # Operating System Entities
 *
 * Exports the various implementations of the [`OSEntityId`] based objects
 *
 * [`OSEntityId`]: /api/ents/struct.OSEntityId.html
 */

pub use group::*;
pub use user::*;

mod group;
mod user;
