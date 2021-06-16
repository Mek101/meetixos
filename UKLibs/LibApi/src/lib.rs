/*! # Kernel API library
 *
 * Implements an object oriented interface to interact with the MeetiX's
 * Kernel
 */

#![no_std]
#![feature(asm,
           array_methods,
           min_specialization,
           core_intrinsics,
           iter_advance_by,
           const_fn_trait_bound)]

pub mod bits;
pub mod caller;
pub mod ents;
pub mod errors;
pub mod objs;
pub mod path;
pub mod tasks;
pub mod time;

mod config;
