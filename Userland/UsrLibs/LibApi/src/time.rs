/*! Time management */

use core::{
    fmt,
    fmt::Debug,
    ops::{
        Add,
        AddAssign,
        Deref,
        DerefMut,
        Div,
        DivAssign,
        Mul,
        MulAssign,
        Sub,
        SubAssign
    },
    time::Duration
};

use api_data::{
    sys::{
        codes::KernTimeInstFnId,
        fn_path::KernFnPath
    },
    time::RawInstant
};

use crate::handle::KernHandle;

/**
 * Precise timestamp unit.
 *
 * Internally encapsulates a `Duration` object that is accessible through
 * `Instant::as_duration()`/`Instant::as_duration_mut()` or the
 * `Deref`/`DerefMut`
 */
#[derive(Default)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(Hash)]
pub struct Instant {
    m_raw_instant: RawInstant
}

impl Instant {
    /**
     * Constructs an `Instant` from the given raw parts
     */
    pub fn new(secs: u64, nanos: u32) -> Self {
        Self { m_raw_instant: RawInstant::new(secs, nanos) }
    }

    /**
     * Constructs an updated `Instant`
     */
    pub fn now() -> Self {
        let mut raw_instant = RawInstant::default();
        KernHandle::kern_call_1(KernFnPath::TimeInst(KernTimeInstFnId::Now),
                                &mut raw_instant as *mut _ as usize)
                   .map(|_| Self { m_raw_instant: raw_instant })
                   .expect("Failed to obtain updated Instant")
    }

    /**
     * Returns the reference to the underling `Duration` instance
     */
    #[inline]
    pub fn as_duration(&self) -> &Duration {
        &self.m_raw_instant
    }

    /**
     * Returns the reference to the underling `RawInstant` instance
     */
    #[inline]
    pub fn as_raw_instant(&self) -> &RawInstant {
        &self.m_raw_instant
    }

    /**
     * Returns the mutable reference to the underling `Duration` instance
     */
    #[inline]
    pub fn as_duration_mut(&mut self) -> &mut Duration {
        &mut self.m_raw_instant
    }

    /**
     * Returns the mutable reference to the underling `RawInstant` instance
     */
    #[inline]
    pub fn as_raw_instant_mut(&mut self) -> &mut RawInstant {
        &mut self.m_raw_instant
    }
}

impl From<RawInstant> for Instant {
    /**
     * Implemented to perform `RawInstant::into() -> Instant`
     */
    fn from(raw_instant: RawInstant) -> Self {
        Self { m_raw_instant: raw_instant }
    }
}

impl Deref for Instant {
    type Target = Duration;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_duration()
    }
}

impl DerefMut for Instant {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_duration_mut()
    }
}

impl Add for Instant {
    type Output = Instant;

    #[inline]
    fn add(self, rhs: Instant) -> Instant {
        Self { m_raw_instant: self.as_duration().add(*rhs) }
    }
}

impl AddAssign for Instant {
    #[inline]
    fn add_assign(&mut self, rhs: Instant) {
        self.as_duration_mut().add_assign(*rhs)
    }
}

impl Sub for Instant {
    type Output = Instant;

    #[inline]
    fn sub(self, rhs: Instant) -> Instant {
        Self { m_raw_instant: self.as_duration().sub(*rhs) }
    }
}

impl SubAssign for Instant {
    #[inline]
    fn sub_assign(&mut self, rhs: Instant) {
        self.as_duration_mut().sub_assign(*rhs)
    }
}

impl Mul<u32> for Instant {
    type Output = Instant;

    #[inline]
    fn mul(self, rhs: u32) -> Instant {
        Self { m_raw_instant: self.as_duration().mul(rhs) }
    }
}

impl Mul<Instant> for u32 {
    type Output = Instant;

    #[inline]
    fn mul(self, rhs: Instant) -> Instant {
        Instant { m_raw_instant: rhs.as_duration().mul(self) }
    }
}

impl MulAssign<u32> for Instant {
    #[inline]
    fn mul_assign(&mut self, rhs: u32) {
        self.as_duration_mut().mul_assign(rhs)
    }
}

impl Div<u32> for Instant {
    type Output = Instant;

    #[inline]
    fn div(self, rhs: u32) -> Instant {
        Self { m_raw_instant: self.as_duration().div(rhs) }
    }
}

impl DivAssign<u32> for Instant {
    #[inline]
    fn div_assign(&mut self, rhs: u32) {
        self.as_duration_mut().div_assign(rhs)
    }
}

impl fmt::Display for Instant {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.m_raw_instant.fmt(f)
    }
}
