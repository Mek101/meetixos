/*! # Kernel Loader/Core Shared Code
 *
 * This crate contains wrappers, conveniences, and more in general all
 * the code which is in common with the hh_loader and the kernel's core
 */

#![no_std]
#![feature(asm,
           array_methods,
           step_trait,
           step_trait_ext,
           abi_x86_interrupt,
           const_fn_fn_ptr_basics,
           const_mut_refs,
           const_fn_trait_bound)]

#[macro_use]
extern crate macros;

pub mod addr;
pub mod dbg;
pub mod elf;
pub mod info;
pub mod interrupt;
pub mod logger;
pub mod mem;
pub mod random;
pub mod uart;

mod arch;
