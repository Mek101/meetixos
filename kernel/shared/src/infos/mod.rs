/*! # HAL Boot Informations
 *
 * Implements the structures that contains common bootloader's informations
 * used by all the architectures
 */

pub mod args;
pub mod info;
#[cfg(feature = "loader_stage")]
pub mod mem_area;
pub mod vm_layout;
