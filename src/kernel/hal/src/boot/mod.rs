/*! # HAL Bootstrap Data
 *
 * Due to the impossibility to use a coherent bootloader interface across
 * the different architectures supported, this module exposes common boot
 * informations and architecture dependent wrapper macros for the various
 * kernel's entry points (BSP & AP)
 */

pub use entry::*;
pub use init::*;
pub use tasks::*;

mod entry;
mod init;
mod tasks;

pub mod infos;
