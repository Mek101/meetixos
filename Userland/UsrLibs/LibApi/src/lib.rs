/*! # Kernel API library
 *
 * Implements an object oriented interface to interact with the MeetiX's
 * Kernel
 */

#![no_std]
#![feature(asm)]

pub mod arch;
pub mod caller;
pub mod obj;

pub mod bits;
pub mod ents;
pub mod errors;
pub mod objs;
pub mod path;
pub mod tasks;
pub mod time;

mod config;
