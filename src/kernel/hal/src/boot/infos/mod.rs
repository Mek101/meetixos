/*! # HAL Boot Informations
 *
 * Implements the structures that contains common bootloader's informations
 * used by all the architectures
 */

pub use args::*;
pub use info::*;
pub use mem_area::*;

mod args;
mod info;
mod mem_area;
