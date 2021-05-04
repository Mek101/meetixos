/*! # Cumulative Synchronization Library
 *
 * Due to the availability of a plethora of sync crates ([`lock_api`],
 * [`spinning_top`], [`std::sync`], etc..), and since there isn't a single
 * crate that exposes all the necessary stuffs (Mutex, RwLock, Once,
 * etc...), this library (for now) simply re-exports the necessary stuffs
 * from the different external crates, keeping a single place where maintain
 * the version of the used crates
 *
 * [`lock_api`]: https://docs.rs/lock_api/0.4.4/lock_api/
 * [`spinning_top`]: https://docs.rs/spinning_top/0.2.3/spinning_top/
 * [`std::sync`]: https://doc.rust-lang.org/stable/std/sync/
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
