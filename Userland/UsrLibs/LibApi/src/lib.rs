/*! # Kernel API library
 *
 * Implements an object oriented interface to interact with the MeetiX's
 * Kernel
 */

#![no_std]
#![feature(asm)]

pub mod arch;
pub mod config;
pub mod handle;
pub mod obj;
pub mod task;
pub mod time;

pub mod bits;
pub mod ents;
pub mod errors;
pub mod objs;
pub mod path;
pub mod tasks;
pub mod time;
