/*! Debug utilities */

use core::fmt;

/**
 * Kibibyte multiplier
 */
pub const KIB: usize = 1024;

/**
 * Mebibyte Byte multiplier
 */
pub const MIB: usize = KIB * 1024;

/**
 * Gibibyte Byte multiplier
 */
pub const GIB: usize = MIB * 1024;

/**
 * Tebibyte multiplier
 */
pub const TIB: usize = GIB * 1024;

/**
 * Returns a `fmt::Display` implementation to print in a pretty way the
 * `size_value` given
 */
pub fn dbg_display_size(size_value: usize) -> impl fmt::Display {
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
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.m_decimals > 0 {
                write!(f, "{}.{}{}", self.m_value, self.m_decimals, self.m_multiplier)
            } else {
                write!(f, "{}{}", self.m_value, self.m_multiplier)
            }
        }
    }

    DebugSizeMul::new(size_value)
}
