/*! Mutex spinning implementation */

use core::{
    hint,
    sync::atomic::{
        AtomicBool,
        Ordering
    }
};

use crate::{
    guards::LockGuardSendable,
    mutex::{
        BackRawMutex,
        ConstCreatBackRawMutex
    }
};

/**
 * `BackRawMutex` implementation which relies on a `AtomicBool` to ensure
 * locking
 */
pub struct RawSpinMutex {
    m_is_locked: AtomicBool
}

impl RawSpinMutex {
    /**
     * Tries to acquire the `RawSpinMutex` with possibility of failures,
     * even when the mutex is not locked.
     *
     * According to the documentation, `compare_exchange_weak()` produces
     * more efficient code
     */
    fn try_lock_weak(&self) -> bool {
        self.m_is_locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }
}

impl ConstCreatBackRawMutex for RawSpinMutex {
    const CONST_CREAT: Self = Self { m_is_locked: AtomicBool::new(false) };
}

unsafe impl BackRawMutex for RawSpinMutex {
    type LockGuardShareabilityMark = LockGuardSendable;

    fn do_lock(&self) {
        while !self.try_lock_weak() {
            while self.do_is_locked() {
                hint::spin_loop()
            }
        }
    }

    fn do_try_lock(&self) -> bool {
        self.m_is_locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    unsafe fn do_unlock(&self) {
        self.m_is_locked.store(false, Ordering::Release);
    }

    fn do_is_locked(&self) -> bool {
        self.m_is_locked.load(Ordering::Relaxed)
    }
}
