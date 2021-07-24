/*! # Synchronization Library
 *
 * Exposes generic primitives to implement thread synchronization
 */

#![no_std]
#![feature(once_cell, const_fn_trait_bound, const_mut_refs)]

use crate::{
    mutex::{
        spin_mutex::RawSpinMutex,
        Mutex
    },
    rw_lock::{
        spin_rw_lock::RawSpinRwLock,
        RwLock
    }
};

pub use core::lazy::{
    Lazy,
    OnceCell
};

pub mod guards;
pub mod mutex;
pub mod rw_lock;

/**
 * Convenient type alias for `Mutex<RawSpinMutex, T>`
 */
pub type SpinMutex<T> = Mutex<RawSpinMutex, T>;

/**
 * Convenient type alias for `RwLock<RawSpinRwLock, T>`
 */
pub type SpinRwLock<T> = RwLock<RawSpinRwLock, T>;
