/*! stores the Kernel symbols for the Kernel core */

/* stores the Kernel core symbols */
static KERNEL_SYMBOLS: &str = include_str!(env!("KERNEL_SYM"));

/**
 * Returns the immutable reference to the Kernel core symbols
 */
pub fn kernel_symbols() -> &'static str {
    KERNEL_SYMBOLS
}
