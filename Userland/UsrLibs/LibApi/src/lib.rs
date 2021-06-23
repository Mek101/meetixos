/*! # Kernel API library
 *
 * Implements an object oriented interface to interact with the MeetiX's
 * Kernel
 */

#![no_std]
#![feature(asm)]

pub mod arch;
pub mod config;
pub mod entity;
pub mod handle;
pub mod obj;
pub mod task;
pub mod time;
