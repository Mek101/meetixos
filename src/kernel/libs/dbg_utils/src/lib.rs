/*! # Debug Utils
 *
 * Little collection of functions and utilities useful to print & debug data
 * and structures both in kernel's core and hh_loader
 */

#![no_std]

use core::fmt;

/** Kibibyte multiplier
 */
pub const KIB: usize = 1024;

/** Mebibyte Byte multiplier
 */
pub const MIB: usize = KIB * 1024;

/** Gibibyte Byte multiplier
 */
pub const GIB: usize = MIB * 1024;

/** Tebibyte multiplier
 */
pub const TIB: usize = GIB * 1024;

/** Returns a [`fmt::Display`] implementation to print in a pretty way the
 * `size_value` given
 *
 * [`fmt::Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
 */
pub fn debug_size_multiplier(size_value: usize) -> impl fmt::Display {
    DebugSizeMul::new(size_value)
}

/** # Debug Size Multiplier
 *
 * Internal debug struct used by the [`debug_size_multiplier()`] to
 * implement [`fmt::Display`]
 *
 * [`debug_size_multiplier()`]: /kernel/debug/fn.debug_size_multiplier.html
 * [`fmt::Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
 */
struct DebugSizeMul {
    m_value: usize,
    m_decimals: usize,
    m_multiplier: &'static str
}

impl DebugSizeMul {
    pub fn new(value: usize) -> Self {
        let (value, decimals, str_multiplier) = if value >= TIB {
            (value / TIB, value % TIB, "TiB")
        } else if value >= GIB {
            (value / GIB, value % GIB, "GiB")
        } else if value >= MIB {
            (value / MIB, value % MIB, "MiB")
        } else if value >= KIB {
            (value / KIB, value % KIB, "KiB")
        } else {
            (value, 0, "Bytes")
        };

        Self { m_value: value,
               m_decimals: decimals,
               m_multiplier: str_multiplier }
    }
}

impl fmt::Display for DebugSizeMul {
    /** Formats the value using the given formatter
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.m_decimals > 0 {
            write!(f, "{}.{}{}", self.m_value, self.m_decimals, self.m_multiplier)
        } else {
            write!(f, "{}{}", self.m_value, self.m_multiplier)
        }
    }
}
