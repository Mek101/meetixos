/*! Mutual exclusion gate */

use core::cell::UnsafeCell;

use crate::{
    guards::LockGuardShareability,
    mutex::guard::MutexDataGuard
};

pub mod guard;
pub mod spin;

/**
 * Mutual exclusive gate protector for a customizable data type.
 *
 * Relies on a `BackRawMutex` implementation to effectively  protect the
 * data against concurrent access
 */
pub struct Mutex<R, T>
    where R: BackRawMutex,
          T: ?Sized {
    m_back_mutex: R,
    m_held_data: UnsafeCell<T>
}

impl<R, T> Mutex<R, T> where R: ConstCreatBackRawMutex {
    /**
     * Constructs a `Mutex` wrapping the given `value` and a const-creatable
     * `BackRawMutex`
     */
    pub const fn const_new(value: T) -> Self {
        Self { m_back_mutex: R::CONST_CREAT,
               m_held_data: UnsafeCell::new(value) }
    }
}

impl<R, T> Mutex<R, T> where R: FallibleCreatBackRawMutex {
    /**
     * Constructs a `Mutex` wrapping the given `value` and a may-fail
     * `BackRawMutex`
     */
    pub fn new(value: T) -> Result<Self, R::CreatError> {
        Ok(Self { m_back_mutex: R::creat_raw()?,
                  m_held_data: UnsafeCell::new(value) })
    }
}

impl<R, T> Mutex<R, T> where R: BackRawMutex {
    /**
     * Constructs a `Mutex` from his fundamental components
     */
    pub const fn raw_new(back_mutex: R, value: T) -> Self {
        Self { m_back_mutex: back_mutex,
               m_held_data: UnsafeCell::new(value) }
    }

    /**
     * Returns the unwrapped inner data
     */
    #[inline]
    pub fn into_inner(self) -> T {
        self.m_held_data.into_inner()
    }
}

impl<R, T> Mutex<R, T>
    where R: BackRawMutex,
          T: ?Sized
{
    /**
     * Acquires the `Mutex`, blocking the current thread until it is able to
     * do so.
     *
     * Returns the `MutexDataGuard` RAII obj, which automatically unlocks
     * the `Mutex` when goes out of scope (calls `Drop::drop()`)
     */
    #[inline]
    pub fn lock(&self) -> MutexDataGuard<'_, R, T> {
        self.m_back_mutex.do_lock();

        MutexDataGuard::new(self)
    }

    /**
     * Tries to acquire the `Mutex`, if success returns the `MutexDataGuard`
     * RAII obj
     */
    #[inline]
    pub fn try_lock(&self) -> Option<MutexDataGuard<'_, R, T>> {
        if self.m_back_mutex.do_try_lock() {
            Some(MutexDataGuard::new(self))
        } else {
            None
        }
    }

    /**
     * Forces the `Mutex` unlock
     */
    #[inline]
    pub unsafe fn force_unlock(&self) {
        self.m_back_mutex.do_unlock()
    }

    /**
     * Returns whether this `Mutex` is already locked
     */
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.m_back_mutex.do_is_locked()
    }

    /**
     * Returns the reference to the inner `BackRawMutex`
     */
    #[inline]
    pub unsafe fn raw_mutex(&self) -> &R {
        &self.m_back_mutex
    }

    /**
     * Returns the mutable reference to the held data.
     *
     * Since this method acquires `self` as `&mut` no locking is needed
     */
    #[inline]
    pub fn data_mut(&mut self) -> &mut T {
        unsafe { &mut *self.m_held_data.get() }
    }

    /**
     * Returns the mutable pointer to the held data
     */
    #[inline]
    pub fn data_ptr(&self) -> *mut T {
        self.m_held_data.get()
    }
}

unsafe impl<R, T> Send for Mutex<R, T>
    where R: BackRawMutex + Send,
          T: ?Sized + Send
{
    /* No methods, just a marker trait */
}

unsafe impl<R, T> Sync for Mutex<R, T>
    where R: BackRawMutex + Sync,
          T: ?Sized + Send
{
    /* No methods, just a marker trait */
}

/**
 * Interface on which the `Mutex` relies to effectively perform
 * locking/unlocking operations over the held data
 */
pub unsafe trait BackRawMutex {
    /**
     * Customizable creation error type
     */
    type LockGuardShareabilityMark: LockGuardShareability;

    /**
     * Acquires this `Mutex`, blocking the current thread until it is able
     * to do so
     */
    fn do_lock(&self);

    /**
     * Tries to acquire this `Mutex` without blocking the current thread.
     *
     * Returns `true` when locked successfully, `false` otherwise
     */
    fn do_try_lock(&self) -> bool;

    /**
     * Unlocks this `Mutex`.
     *
     * Must be called after a successful call to `do_lock()`/`do_try_lock()`
     */
    unsafe fn do_unlock(&self);

    /**
     * Checks whether this mutex is already locked
     */
    #[inline]
    fn do_is_locked(&self) -> bool {
        if self.do_try_lock() {
            unsafe {
                self.do_unlock();
            }
            false
        } else {
            true
        }
    }
}

/**
 * Interface on which the `Mutex` relies to const-create the `BackRawMutex`
 */
pub trait ConstCreatBackRawMutex: BackRawMutex {
    /**
     * Creates a `BackRawMutex` using const pseudo-function
     */
    const CONST_CREAT: Self;
}

/**
 * Interface on which the `Mutex` relies to create the `BackRawMutex` with
 * failure
 */
pub trait FallibleCreatBackRawMutex: BackRawMutex {
    /**
     * Customizable creation error type
     */
    type CreatError;

    /**
     * Creates a new `BackRawMutex` implementation which may fail if, for
     * example, relies on services of the operating system
     */
    fn creat_raw() -> Result<Self, Self::CreatError>
        where Self: Sized;
}
