/*! # Objects Management
 *
 * Exports the submodules related to the kernel's managed objects
 */

pub use config::*;
pub use object::*;

#[macro_use]
mod object;

mod config;
pub mod impls;
pub mod infos;
