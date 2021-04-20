/*! # x86_64 Bootstrap Data
 *
 * This module exposes x86_64 boot informations and architecture dependent
 * wrapper macros for the various kernel's entry points (BSP & AP)
 */

pub use infos::X64BootInfos as HwBootInfos;

#[macro_use]
mod entry;
mod infos;
