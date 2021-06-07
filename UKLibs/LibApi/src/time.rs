/*! Time Management */

/** re-export `Duration` as libapi struct */
pub use core::time::Duration;
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
    }
};

use os::sysc::{
    codes::KernTimeInstFnId,
    fn_path::KernFnPath
};

use crate::caller::KernCaller;

/**
 * Precise timestamp unit used both in Kernel and userspace.
 *
 * Internally encapsulates a `Duration` object that is accessible through
 * `Instant::as_duration()`/`Instant::as_duration_mut()` or the
 * `Deref`/`DerefMut`
 */
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Instant(Duration);

impl Instant {
    /**
     * Constructs an `Instant` from the given raw parts
     */
    pub fn new(secs: u64, nanos: u32) -> Self {
        Self(Duration::new(secs, nanos))
    }

    /**
     * Constructs an updated `Instant`
     */
    pub fn now() -> Self {
        let mut value = Self::default();
        Self::default().kern_call_1(KernFnPath::TimeInst(KernTimeInstFnId::Now),
                                    &mut value as *mut _ as usize)
                       .map(|_| value)
                       .unwrap_or(value)
    }

    /**
     * Returns the reference to the underling `Duration` instance
     */
    pub fn as_duration(&self) -> &Duration {
        &self.0
    }

    /**
     * Returns the mutable reference to the underling `Duration` instance
     */
    pub fn as_duration_mut(&mut self) -> &mut Duration {
        &mut self.0
    }
}

impl From<Duration> for Instant {
    fn from(duration: Duration) -> Self {
        Self(duration)
    }
}

impl KernCaller for Instant {
    /* Nothing to implement */
}

impl Deref for Instant {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        self.as_duration()
    }
}

impl DerefMut for Instant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_duration_mut()
    }
}

impl Add for Instant {
    type Output = Instant;

    fn add(self, rhs: Instant) -> Instant {
        Self((*self) + (*rhs))
    }
}

impl AddAssign for Instant {
    fn add_assign(&mut self, rhs: Instant) {
        *self = *self + rhs;
    }
}

impl Sub for Instant {
    type Output = Instant;

    fn sub(self, rhs: Instant) -> Instant {
        Self((*self) - (*rhs))
    }
}

impl SubAssign for Instant {
    fn sub_assign(&mut self, rhs: Instant) {
        *self = *self - rhs;
    }
}

impl Mul<u32> for Instant {
    type Output = Instant;

    fn mul(self, rhs: u32) -> Instant {
        Self((*self) * rhs)
    }
}

impl Mul<Instant> for u32 {
    type Output = Instant;

    fn mul(self, rhs: Instant) -> Instant {
        Instant((*rhs) * self)
    }
}

impl MulAssign<u32> for Instant {
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl Div<u32> for Instant {
    type Output = Instant;

    fn div(self, rhs: u32) -> Instant {
        Self((*self) / rhs)
    }
}

impl DivAssign<u32> for Instant {
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

impl fmt::Display for Instant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
