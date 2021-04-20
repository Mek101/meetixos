/*! # Time Management
 *
 * Implements the time management used both in userspace and in kernel to
 * represent the time
 */

/** re-export `Duration` as libapi's struct */
pub use core::time::Duration;
use core::{
    fmt,
    fmt::{Debug, Display, Formatter},
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign
    }
};

use os::sysc::{codes::KernTimeInstFnId, fn_path::KernFnPath};

use crate::caller::KernCaller;

/** # Instant Time Value
 *
 * Implements a precise timestamp unit used both in kernel and userspace.
 *
 * Internally encapsulates a [`Duration`] object that is accessible through
 * [`Instant::as_duration()`]/[`Instant::as_duration_mut()`] or the
 * [`Deref`]/[`DerefMut`]
 *
 * [`Duration`]: /api/time/struct.Duration.html
 * [`Instant::as_duration()`]:
 * /api/time/struct.Instant.html#method.as_duration [`Instant::
 * as_duration_mut()`]: /api/time/struct.Instant.html#method.as_duration_mut
 * [`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
 * [`DerefMut`]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
 */
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Instant(Duration);

impl Instant {
    /** # Constructs an `Instant`
     *
     * The constructed instance will use the given seconds in addition to
     * the given nano seconds
     */
    pub fn new(secs: u64, nanos: u32) -> Self {
        Self(Duration::new(secs, nanos))
    }

    /** # Constructs an updated `Instant`
     *
     * Fills an empty `Instant` with the current instant
     */
    pub fn now() -> Self {
        let mut value = Self::default();
        Self::default().kern_call_1(KernFnPath::TimeInst(KernTimeInstFnId::Now),
                                    &mut value as *mut _ as usize)
                       .map(|_| value)
                       .unwrap_or(value)
    }

    /** Returns the reference to the underling [`Duration`] instance
     *
     * [`Duration`]: /api/time/struct.Duration.html
     */
    pub fn as_duration(&self) -> &Duration {
        &self.0
    }

    /** Returns the mutable reference to the underling [`Duration`] instance
     *
     * [`Duration`]: /api/time/struct.Duration.html
     */
    pub fn as_duration_mut(&mut self) -> &mut Duration {
        &mut self.0
    }
}

impl From<Duration> for Instant {
    /** Performs the conversion.
     */
    fn from(duration: Duration) -> Self {
        Self(duration)
    }
}

impl KernCaller for Instant {
    // Nothing to implement
}

impl Deref for Instant {
    /** The resulting type after dereference.
     */
    type Target = Duration;

    /** Dereferences the value to the underling [`Duration`] instance
     *
     * [`Duration`]: /api/time/struct.Duration.html
     */
    fn deref(&self) -> &Self::Target {
        self.as_duration()
    }
}

impl DerefMut for Instant {
    /** Mutably dereferences the value to the underling [`Duration`] instance
     *
     * [`Duration`]: /api/time/struct.Duration.html
     */
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_duration_mut()
    }
}

impl Add for Instant {
    /** The resulting type after applying the `+` operator.
     */
    type Output = Instant;

    /** Dispatches the operation to the [`Duration`] implementation
     *
     * [`Duration`]: /api/time/struct.Duration.html
     */
    fn add(self, rhs: Instant) -> Instant {
        Self((*self) + (*rhs))
    }
}

impl AddAssign for Instant {
    /** Dispatches the operation to the [`Duration`] implementation
     *
     * [`Duration`]: /api/time/struct.Duration.html
     */
    fn add_assign(&mut self, rhs: Instant) {
        *self = *self + rhs;
    }
}

impl Sub for Instant {
    /** The resulting type after applying the `-` operator.
     */
    type Output = Instant;

    /** Dispatches the operation to the [`Duration`] implementation
     *
     * [`Duration`]: /api/time/struct.Duration.html
     */
    fn sub(self, rhs: Instant) -> Instant {
        Self((*self) - (*rhs))
    }
}

impl SubAssign for Instant {
    /** Dispatches the operation to the [`Duration`] implementation
     *
     * [`Duration`]: /api/time/struct.Duration.html
     */
    fn sub_assign(&mut self, rhs: Instant) {
        *self = *self - rhs;
    }
}

impl Mul<u32> for Instant {
    /** The resulting type after applying the `*` operator.
     */
    type Output = Instant;

    /** Dispatches the operation to the [`Duration`] implementation
     *
     * [`Duration`]: /api/time/struct.Duration.html
     */
    fn mul(self, rhs: u32) -> Instant {
        Self((*self) * rhs)
    }
}

impl Mul<Instant> for u32 {
    /** The resulting type after applying the `*` operator.
     */
    type Output = Instant;

    /** Dispatches the operation to the [`Duration`] implementation
     *
     * [`Duration`]: /api/time/struct.Duration.html
     */
    fn mul(self, rhs: Instant) -> Instant {
        Instant((*rhs) * self)
    }
}

impl MulAssign<u32> for Instant {
    /** Dispatches the operation to the [`Duration`] implementation
     *
     * [`Duration`]: /api/time/struct.Duration.html
     */
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl Div<u32> for Instant {
    /** The resulting type after applying the `/` operator.
     */
    type Output = Instant;

    /** Dispatches the operation to the [`Duration`] implementation
     *
     * [`Duration`]: /api/time/struct.Duration.html
     */
    fn div(self, rhs: u32) -> Instant {
        Self((*self) / rhs)
    }
}

impl DivAssign<u32> for Instant {
    /** Dispatches the operation to the [`Duration`] implementation
     *
     * [`Duration`]: /api/time/struct.Duration.html
     */
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

impl Display for Instant {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
