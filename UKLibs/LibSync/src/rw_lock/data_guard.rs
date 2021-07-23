/*! RwLock data guard */

use core::{
    marker::PhantomData,
    ops::{
        Deref,
        DerefMut
    }
};

use crate::rw_lock::{
    RwLock,
    TBackRawRwLock
};

/**
 * RAII read-guard of the held data.
 *
 * Automatically unlocks the originating `RwLock` when `Drop::drop()` is
 * called.
 *
 * Implements `Deref` to allow read-access to the inner held data
 */
#[must_use = "if unused the RwLock will immediately unlock"]
pub struct RwLockDataReadGuard<'a, R, T>
    where R: TBackRawRwLock,
          T: ?Sized {
    m_rw_lock: &'a RwLock<R, T>,
    m_security: PhantomData<(&'a T, R::LockGuardShareabilityMark)>
}

impl<'a, R, T> RwLockDataReadGuard<'a, R, T>
    where R: TBackRawRwLock + 'a,
          T: ?Sized + 'a /* Constructors */
{
    /**
     * Constructs a `RwLockDataReadGuard` which wraps the given `RwLock`
     */
    pub(super) const fn new(rw_lock: &'a RwLock<R, T>) -> Self {
        Self { m_rw_lock: rw_lock,
               m_security: PhantomData }
    }
}

impl<'a, R, T> RwLockDataReadGuard<'a, R, T>
    where R: TBackRawRwLock + 'a,
          T: ?Sized + 'a /* Getters */
{
    /**
     * Returns the reference to the originating `RwLock`
     */
    pub fn rw_lock(&self) -> &'a RwLock<R, T> {
        self.m_rw_lock
    }
}

impl<'a, R, T> Deref for RwLockDataReadGuard<'a, R, T>
    where R: TBackRawRwLock + 'a,
          T: ?Sized + 'a
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { &*self.m_rw_lock.m_held_data.get() }
    }
}

impl<'a, R, T> Drop for RwLockDataReadGuard<'a, R, T>
    where R: TBackRawRwLock + 'a,
          T: ?Sized + 'a
{
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.m_rw_lock.m_back_raw_rw_lock.do_unlock_shared();
        }
    }
}

/**
 * RAII read-guard of the held data.
 *
 * Automatically unlocks the originating `RwLock` when `Drop::drop()` is
 * called.
 *
 * Implements `Deref` & `DerefMut` to allow write-access to the inner held
 * data
 */
#[must_use = "if unused the RwLock will immediately unlock"]
pub struct RwLockDataWriteGuard<'a, R, T>
    where R: TBackRawRwLock,
          T: ?Sized {
    m_rw_lock: &'a RwLock<R, T>,
    m_security: PhantomData<(&'a mut T, R::LockGuardShareabilityMark)>
}

impl<'a, R, T> RwLockDataWriteGuard<'a, R, T>
    where R: TBackRawRwLock + 'a,
          T: ?Sized + 'a /* Constructors */
{
    /**
     * Constructs a `RwLockDataWriteGuard` which wraps the given `RwLock`
     */
    pub(super) const fn new(rw_lock: &'a RwLock<R, T>) -> Self {
        Self { m_rw_lock: rw_lock,
               m_security: PhantomData }
    }
}

impl<'a, R, T> RwLockDataWriteGuard<'a, R, T>
    where R: TBackRawRwLock + 'a,
          T: ?Sized + 'a /* Getters */
{
    /**
     * Returns the reference to the originating `RwLock`
     */
    pub fn rw_lock(&self) -> &'a RwLock<R, T> {
        self.m_rw_lock
    }
}

impl<'a, R, T> Deref for RwLockDataWriteGuard<'a, R, T>
    where R: TBackRawRwLock + 'a,
          T: ?Sized + 'a
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { &*self.m_rw_lock.m_held_data.get() }
    }
}

impl<'a, R, T> DerefMut for RwLockDataWriteGuard<'a, R, T>
    where R: TBackRawRwLock + 'a,
          T: ?Sized + 'a
{
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.m_rw_lock.m_held_data.get() }
    }
}

impl<'a, R, T> Drop for RwLockDataWriteGuard<'a, R, T>
    where R: TBackRawRwLock + 'a,
          T: ?Sized + 'a
{
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.m_rw_lock.m_back_raw_rw_lock.do_unlock_exclusive();
        }
    }
}
