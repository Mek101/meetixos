/*! # Userland/Kernel Shared Data Types And Structures Library
 *
 * Collection of all the data structures which are shared among the userland
 * (used by the `Userland/Libs/LibApi`) and the `Kernel` for system calls
 */

#![no_std]

extern crate alloc;

pub mod entity;
pub mod error;
pub mod instant;
pub mod limit;
pub mod object;
pub mod path;
pub mod sys;
pub mod task;
