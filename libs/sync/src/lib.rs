/*! # Cumulative Synchronization Library
 *
 * Exposes context independent primitives to implement thread
 * synchronization from different external crates
 */

#![no_std]
#![feature(const_fn, once_cell)]

pub use core::lazy::{
    Lazy,
    OnceCell
};

pub use lock_api::{
    GuardNoSend,
    GuardSend,
    Mutex,
    MutexGuard,
    RawMutex,
    RawRwLock,
    RwLock,
    RwLockReadGuard,
    RwLockUpgradableReadGuard,
    RwLockWriteGuard
};

pub use spinning_top::{
    RawSpinlock as RawSpinMutex,
    Spinlock as SpinMutex,
    SpinlockGuard as SpinMutexGuard
};
