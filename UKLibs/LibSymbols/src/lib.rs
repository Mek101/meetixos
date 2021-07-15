/*! # Symbols Mangling & Stack Trace Support Library
 *
 * Provides primitives to demangle the rust compiled symbols and to show the
 * stack trace
 */

#![no_std]
#![feature(asm)]

pub use rustc_demangle::*;

pub mod code_symbol;
pub mod code_symbols;

mod arch;
mod stack_back_trace;
