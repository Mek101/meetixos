/*! # Kernel Loader/Core Shared Code
 *
 * This crate contains all the code shared among the kernel's loader and the
 * kernel's core which is not desirable to be replicated
 */

#![no_std]
#![feature(const_fn,
           asm,
           array_methods,
           step_trait,
           step_trait_ext,
           abi_x86_interrupt,
           const_fn_fn_ptr_basics,
           const_mut_refs)]

#[macro_use]
extern crate macros;

pub mod addr;
pub mod dbg;
pub mod hal;
pub mod infos;
pub mod logger;
pub mod mem;

mod arch;
