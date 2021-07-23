/*! Read/write exclusion gate */

use core::cell::UnsafeCell;

use crate::{
    guards::MTLockGuardShareability,
    rw_lock::data_guard::{
        RwLockDataReadGuard,
        RwLockDataWriteGuard
    }
};

pub mod data_guard;
pub mod spin_rw_lock;

/**
 * Read-write lock.
 *
 * Relies on a `TBackRawRwLock` implementation to effectively  protect the
 * data against concurrent access
 */
pub struct RwLock<R, T>
    where R: TBackRawRwLock,
          T: ?Sized {
    m_back_raw_rw_lock: R,
    m_held_data: UnsafeCell<T>
}

impl<R, T> RwLock<R, T> where R: TConstCreatBackRawRwLock /* Constructors */ {
    /**
     * Constructs a `RwLock` wrapping the given `value` and a
     * const-creatable `TBackRawRwLock`
     */
    pub const fn const_new(value: T) -> Self {
        Self { m_back_raw_rw_lock: R::CONST_CREAT,
               m_held_data: UnsafeCell::new(value) }
    }
}

impl<R, T> RwLock<R, T> where R: TCreatMayFailBackRawRwLock /* Constructors */ {
    /**
     * Constructs a `RwLock` wrapping the given `value` and a may-fail
     * `TBackRawRwLock`
     */
    pub fn new(value: T) -> Result<Self, R::CreatError> {
        Ok(Self { m_back_raw_rw_lock: R::try_creat()?,
                  m_held_data: UnsafeCell::new(value) })
    }
}

impl<R, T> RwLock<R, T> where R: TBackRawRwLock /* Constructors */ {
    /**
     * Constructs a `RwLock` from his fundamental components
     */
    pub fn raw_new(back_rw_lock: R, value: T) -> Self {
        Self { m_back_raw_rw_lock: back_rw_lock,
               m_held_data: UnsafeCell::new(value) }
    }
}

impl<R, T> RwLock<R, T> where R: TBackRawRwLock /* Getters */ {
    /**
     * Returns the unwrapped inner data
     */
    #[inline]
    pub fn into_inner(self) -> T {
        self.m_held_data.into_inner()
    }
}

impl<R, T> RwLock<R, T>
    where R: TBackRawRwLock,
          T: ?Sized /* Methods */
{
    /**
     * Acquires the `RwLock` for reading operation. Multiple readers can
     * exist at the same time
     *
     * Returns the `RwLockDataReadGuard` RAII object, which automatically
     * unlocks the `RwLock` when goes out of scope (calls
     * `Drop::drop()`)
     */
    #[inline]
    pub fn read(&self) -> RwLockDataReadGuard<'_, R, T> {
        self.m_back_raw_rw_lock.do_lock_shared();

        RwLockDataReadGuard::new(self)
    }

    /**
     * Tries to acquire the `RwLock`, if success returns the
     * `RwLockDataReadGuard` RAII object
     */
    #[inline]
    pub fn try_read(&self) -> Option<RwLockDataReadGuard<'_, R, T>> {
        if self.m_back_raw_rw_lock.do_try_lock_shared() {
            Some(RwLockDataReadGuard::new(self))
        } else {
            None
        }
    }

    /**
     * Forces the `RwLock` read unlock
     */
    #[inline]
    pub unsafe fn force_read_unlock(&self) {
        self.m_back_raw_rw_lock.do_unlock_shared()
    }

    /**
     * Acquires the `RwLock` for writing operation. Only one writer can
     * exist at the same time
     *
     * Returns the `RwLockDataWriteGuard` RAII object, which automatically
     * unlocks the `RwLock` when goes out of scope (calls
     * `Drop::drop()`)
     */
    #[inline]
    pub fn write(&self) -> RwLockDataWriteGuard<'_, R, T> {
        self.m_back_raw_rw_lock.do_lock_exclusive();

        RwLockDataWriteGuard::new(self)
    }

    /**
     * Tries to acquire the `RwLock`, if success returns the
     * `RwLockDataWriteGuard` RAII object
     */
    #[inline]
    pub fn try_write(&self) -> Option<RwLockDataWriteGuard<'_, R, T>> {
        if self.m_back_raw_rw_lock.do_try_lock_exclusive() {
            Some(RwLockDataWriteGuard::new(self))
        } else {
            None
        }
    }

    /**
     * Forces the `RwLock` read unlock
     */
    #[inline]
    pub unsafe fn force_write_unlock(&self) {
        self.m_back_raw_rw_lock.do_unlock_exclusive()
    }
}

impl<R, T> RwLock<R, T>
    where R: TBackRawRwLock,
          T: ?Sized /* Getters */
{
    /**
     * Returns whether this `RwLock` is already locked
     */
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.m_back_raw_rw_lock.do_is_locked()
    }

    /**
     * Returns the reference to the inner `TBackRawRwLock`
     */
    #[inline]
    pub unsafe fn raw_rw_lock(&self) -> &R {
        &self.m_back_raw_rw_lock
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
    pub unsafe fn data_ptr(&self) -> *mut T {
        self.m_held_data.get()
    }
}

unsafe impl<R, T> Send for RwLock<R, T>
    where R: TBackRawRwLock + Send,
          T: ?Sized + Send
{
    /* No methods, just a marker trait */
}
unsafe impl<R, T> Sync for RwLock<R, T>
    where R: TBackRawRwLock + Sync,
          T: ?Sized + Send + Sync
{
    /* No methods, just a marker trait */
}

/**
 * Interface on which the `RwLock` relies to effectively perform
 * locking/unlocking operations over the held data
 */
pub unsafe trait TBackRawRwLock {
    /**
     * Thread-safe shareability marker
     */
    type LockGuardShareabilityMark: MTLockGuardShareability;

    /**
     * Acquires a shared read-access to the held data
     */
    fn do_lock_shared(&self);

    /**
     * Tries to acquire a shared read-access to the held data
     */
    fn do_try_lock_shared(&self) -> bool;

    /**
     * Unlocks a shared read-access
     */
    unsafe fn do_unlock_shared(&self);

    /**
     * Acquires a exclusive write-access to the held data
     */
    fn do_lock_exclusive(&self);

    /**
     * Tries to acquire a exclusive write-access to the held data
     */
    fn do_try_lock_exclusive(&self) -> bool;

    /**
     * Unlocks a exclusive write-access
     */
    unsafe fn do_unlock_exclusive(&self);

    /**
     * Checks whether this rw-lock is already locked
     */
    #[inline]
    fn do_is_locked(&self) -> bool {
        if self.do_try_lock_exclusive() {
            unsafe {
                self.do_unlock_exclusive();
            }
            true
        } else {
            false
        }
    }
}

/**
 * Interface on which the `RwLock` relies to const-create the
 * `TBackRawRwLock`
 */
pub trait TConstCreatBackRawRwLock: TBackRawRwLock {
    /**
     * Creates a `TBackRawRwLock` using const pseudo-function
     */
    const CONST_CREAT: Self;
}

/**
 * Interface on which the `RwLock` relies to create the `TBackRawRwLock`
 * with failure
 */
pub trait TCreatMayFailBackRawRwLock: TBackRawRwLock {
    /**
     * Customizable creation error type
     */
    type CreatError;

    /**
     * Creates a new `TBackRawRwLock` implementation which may fail if, for
     * example, relies on services of the operating system
     */
    fn try_creat() -> Result<Self, Self::CreatError>
        where Self: Sized;
}
