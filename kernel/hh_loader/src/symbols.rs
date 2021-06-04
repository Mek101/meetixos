/*! stores the kernel symbols for the kernel core */

/* stores the kernel core symbols */
static KERNEL_SYMBOLS: &str = include_str!(env!("KERNEL_SYM"));

/**
 * Returns the immutable reference to the kernel core symbols
 */
pub fn kernel_symbols() -> &'static str {
    KERNEL_SYMBOLS
}
