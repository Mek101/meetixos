/*! # Cumulative Synchronization Library
 *
 * Exposes context independent primitives to implement thread
 * synchronization from different external crates
 */

#![no_std]
#![feature(once_cell, const_fn_trait_bound, const_mut_refs)]

pub use core::lazy::{
    Lazy,
    OnceCell
};

pub mod guards;
pub mod mutex;
pub mod rw_lock;
