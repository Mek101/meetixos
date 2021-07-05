/*! # Symbols Mangling & Stack Trace Support Library
 *
 * Provides primitives to demangle the rust compiled symbols and to show the
 * stack trace
 */

#![no_std]
#![feature(asm)]

extern crate alloc;

pub use rustc_demangle::*;

pub mod code_symbol;
pub mod list;
pub mod trace;

mod arch;
