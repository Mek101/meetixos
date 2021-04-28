/*! # HAL Boot Informations
 *
 * Implements the structures that contains common bootloader's informations
 * used by all the architectures
 */

pub use args::*;
pub use info::*;
#[cfg(feature = "loader_stage")]
pub use mem_area::*;
pub use vm_layout::*;

mod args;
mod info;
#[cfg(feature = "loader_stage")]
mod mem_area;
mod vm_layout;
