/*! # HAL Entry Points
 *
 * Implements the macros that wraps the common kernel's entry points.
 *
 * They constructs too the code that initializes the HAL and the underling
 * architecture dependent hardware
 */

pub use crate::{ap_entry, bsp_entry};
