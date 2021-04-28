/*! # Kernel Loader/Core Shared Code
 *
 * This crate contains all the code shared among the kernel's loader and the
 * kernel's core which is not desirable to be replicated, only to change
 * little things.
 *
 * The code includes:
 * * HAL: an hardware abstraction layer (virtual/physical addresses, paging
 *   management, UART),
 * * Logger: which relies on the UART to write the formatted messages,
 * * various debug utilities
 * * simple bitmap allocator
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

pub mod bitmap_allocator;
pub mod dbg;
pub mod hal;
pub mod logger;
