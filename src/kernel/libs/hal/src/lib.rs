/*! # Kernel Hardware Abstraction Layer
 *
 * This crate implements all the necessary data structures and functions
 * that makes a nearly complete interface for different hardware
 * architectures.
 *
 * This is used by the kernel to access stuffs like physical/virtual
 * addresses, page directories, page tables, serial/video drivers, common
 * bootloader informations without use arch-specific crates
 */

#![no_std]
#![feature(asm,
           array_methods,
           step_trait,
           step_trait_ext,
           abi_x86_interrupt,
           const_fn_fn_ptr_basics,
           const_mut_refs)]

#[macro_use]
extern crate macros;

pub mod addr;
pub mod boot_infos;
#[cfg(feature = "kernel_stage")]
pub mod interrupt;
pub mod paging;
pub mod uart;

mod arch;
