/*! # Kernel API library
 *
 * Implements an object oriented interface to interact with the MeetiX's
 * Kernel
 */

#![no_std]
#![feature(asm)]
#![feature(vec_into_raw_parts)]

extern crate alloc;

pub mod arch;
pub mod config;
pub mod entity;
pub mod handle;
pub mod obj;
pub mod path;
pub mod path_2;
pub mod task;
pub mod time;
