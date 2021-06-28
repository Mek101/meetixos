/*! # Userland/Kernel Shared Data Types And Structures
 *
 * This library collects all the data structures which are shared among the
 * userland (used by the `Userland/Libs/LibApi`) and the `Kernel`
 */

#![no_std]

pub mod entity;
pub mod error;
pub mod limit;
pub mod obj;
pub mod path;
pub mod sys;
pub mod task;
pub mod time;
