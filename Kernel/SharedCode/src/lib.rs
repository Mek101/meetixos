/*! # Kernel Loader/Core Shared Code
 *
 * This crate contains wrappers, conveniences, and more in general all
 * the code which is in common with the HHLoader and the Kernel's core
 */

#![no_std]
#![feature(asm,
           array_methods,
           step_trait,
           const_fn_fn_ptr_basics,
           const_mut_refs,
           const_fn_trait_bound)]

pub mod addr;
pub mod dbg;
pub mod elf;
pub mod info;
pub mod logger;
pub mod mem;
pub mod random;
pub mod uart;

mod arch;
