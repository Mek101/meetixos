/*! # x86_64 Architecture Module
 *
 * This module implements and exposes the necessary data
 * structures/functions names internally targeted for the x86_64
 * architecture
 */

pub mod addr;
#[cfg(feature = "loader_stage")]
pub mod infos;
pub mod interrupt;
pub mod mem;
pub mod uart;
