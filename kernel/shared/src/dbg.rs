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
        m_value: f32,
        m_multiplier: &'static str
    }

    impl DebugSizeMul {
        pub fn new(value: usize) -> Self {
            const F_TIB: f32 = TIB as f32;
            const F_GIB: f32 = GIB as f32;
            const F_MIB: f32 = MIB as f32;
            const F_KIB: f32 = KIB as f32;

            let value = value as f32;
            let (value, str_multiplier) = if value >= F_TIB {
                (value / F_TIB, "TiB")
            } else if value >= F_GIB {
                (value / F_GIB, "GiB")
            } else if value >= F_MIB {
                (value / F_MIB, "MiB")
            } else if value >= F_KIB {
                (value / F_KIB, "KiB")
            } else {
                (value, "Bytes")
            };

            Self { m_value: value,
                   m_multiplier: str_multiplier }
        }
    }

    impl fmt::Display for DebugSizeMul {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}{}", self.m_value, self.m_multiplier)
        }
    }

    DebugSizeMul::new(size_value)
}
