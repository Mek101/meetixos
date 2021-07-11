/*! Mutex data guard */

use core::{
    marker::PhantomData,
    ops::{
        Deref,
        DerefMut
    }
};

use crate::mutex::{
    BackRawMutex,
    Mutex
};

/**
 * RAII guard of the held data.
 *
 * Automatically unlocks the originating `Mutex` when `Drop::drop()` is
 * called.
 *
 * Implements `Deref` and `DerefMut` to allow access to the inner held data
 */
#[must_use = "if unused the Mutex will immediately unlock"]
pub struct MutexDataGuard<'a, R, T>
    where R: BackRawMutex,
          T: ?Sized {
    m_mutex: &'a Mutex<R, T>,
    m_security: PhantomData<(&'a mut T, R::LockGuardShareabilityMark)>
}

impl<'a, R, T> MutexDataGuard<'a, R, T>
    where R: BackRawMutex + 'a,
          T: ?Sized + 'a /* Constructors */
{
    /**
     * Constructs a `MutexDataGuard` which wraps the given `Mutex`
     */
    pub(super) const fn new(mutex: &'a Mutex<R, T>) -> Self {
        Self { m_mutex: mutex,
               m_security: PhantomData }
    }
}

impl<'a, R, T> MutexDataGuard<'a, R, T>
    where R: BackRawMutex + 'a,
          T: ?Sized + 'a /* Getters */
{
    /**
     * Returns the reference to the held `Mutex`
     */
    pub fn mutex(&self) -> &'a Mutex<R, T> {
        self.m_mutex
    }
}

impl<'a, R, T> Deref for MutexDataGuard<'a, R, T>
    where R: BackRawMutex + 'a,
          T: ?Sized + 'a
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { &*self.m_mutex.m_held_data.get() }
    }
}

impl<'a, R, T> DerefMut for MutexDataGuard<'a, R, T>
    where R: BackRawMutex + 'a,
          T: ?Sized + 'a
{
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.m_mutex.m_held_data.get() }
    }
}

impl<'a, R, T> Drop for MutexDataGuard<'a, R, T>
    where R: BackRawMutex + 'a,
          T: ?Sized + 'a
{
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.m_mutex.m_back_raw_mutex.do_unlock();
        }
    }
}

unsafe impl<'a, R, T> Sync for MutexDataGuard<'a, R, T>
    where R: BackRawMutex + Sync + 'a,
          T: ?Sized + Sync + 'a
{
    /* No methods, just a marker trait */
}
