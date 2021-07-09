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
pub mod code_symbols_list;
pub mod stack_back_trace;

mod arch;
