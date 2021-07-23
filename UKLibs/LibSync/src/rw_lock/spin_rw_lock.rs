/*! RwLock spinning implementation */

use core::{
    hint::spin_loop,
    sync::atomic::{
        AtomicUsize,
        Ordering
    }
};

use crate::{
    guards::LockGuardSendable,
    rw_lock::{
        TBackRawRwLock,
        TConstCreatBackRawRwLock
    }
};

pub struct RawSpinRwLock {
    m_state: AtomicUsize
}

/**
 * Spin-loop `TBackRawRwLock` implementation
 */
impl RawSpinRwLock /* Constants */ {
    const C_READER: usize = 1 << 1;
    const C_WRITER: usize = 1;
}

impl TConstCreatBackRawRwLock for RawSpinRwLock {
    const CONST_CREAT: Self = Self { m_state: AtomicUsize::new(0) };
}

unsafe impl TBackRawRwLock for RawSpinRwLock {
    type LockGuardShareabilityMark = LockGuardSendable;

    #[inline]
    fn do_lock_shared(&self) {
        while !self.do_try_lock_shared() {
            spin_loop();
        }
    }

    #[inline]
    fn do_try_lock_shared(&self) -> bool {
        let state_value = self.m_state.fetch_add(Self::C_READER, Ordering::Acquire);
        if state_value & Self::C_WRITER != 0 {
            self.m_state.fetch_sub(Self::C_READER, Ordering::Release);
            false
        } else {
            true
        }
    }

    #[inline]
    unsafe fn do_unlock_shared(&self) {
        self.m_state.fetch_sub(Self::C_READER, Ordering::Release);
    }

    #[inline]
    fn do_lock_exclusive(&self) {
        while !self.do_try_lock_exclusive() {
            spin_loop();
        }
    }

    #[inline]
    fn do_try_lock_exclusive(&self) -> bool {
        if self.m_state
               .compare_exchange(0, Self::C_WRITER, Ordering::Acquire, Ordering::Relaxed)
               .is_ok()
        {
            true
        } else {
            false
        }
    }

    #[inline]
    unsafe fn do_unlock_exclusive(&self) {
        self.m_state.fetch_sub(Self::C_WRITER, Ordering::Release);
    }
}
