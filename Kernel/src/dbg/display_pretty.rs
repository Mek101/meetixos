/*! fmt::Display support */

use core::fmt;

use crate::dbg::{
    C_GIB,
    C_KIB,
    C_MIB,
    C_TIB
};

impl DisplaySizePretty for usize {
    fn display_pretty(&self) -> DbgDisplayFmtSize {
        DbgDisplayFmtSize::new(*self)
    }
}

/**
 * Returns a `fmt::Display` implementation to print the value as size
 * multiplier
 */
pub trait DisplaySizePretty {
    /**
     * Returns a `fmt::Display` implementation to print the value as size
     * multiplier
     */
    fn display_pretty(&self) -> DbgDisplayFmtSize;
}

/**
 * `fmt::Display` implementation to print a size divided by a multiplier in
 * a pretty way
 */
pub struct DbgDisplayFmtSize {
    m_raw_value: f64,
    m_multiplier: &'static str
}

impl DbgDisplayFmtSize /* Constructors */ {
    /**
     * Constructs a `DbgDisplayFmtSize`
     */
    fn new(usize_value: usize) -> Self {
        const CF_TIB: f64 = C_TIB as f64;
        const CF_GIB: f64 = C_GIB as f64;
        const CF_MIB: f64 = C_MIB as f64;
        const CF_KIB: f64 = C_KIB as f64;

        let raw_floating_value = usize_value as f64;
        let (raw_value, str_multiplier) = if raw_floating_value >= CF_TIB {
            (raw_floating_value / CF_TIB, "TiB")
        } else if raw_floating_value >= CF_GIB {
            (raw_floating_value / CF_GIB, "GiB")
        } else if raw_floating_value >= CF_MIB {
            (raw_floating_value / CF_MIB, "MiB")
        } else if raw_floating_value >= CF_KIB {
            (raw_floating_value / CF_KIB, "KiB")
        } else {
            (raw_floating_value, "Bytes")
        };

        Self { m_raw_value: raw_value,
               m_multiplier: str_multiplier }
    }
}

impl fmt::Display for DbgDisplayFmtSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}{}", self.m_raw_value, self.m_multiplier)
    }
}
