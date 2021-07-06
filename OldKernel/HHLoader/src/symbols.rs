/*! stores the OldKernel symbols for the OldKernel core */

/* stores the OldKernel core symbols */
static KERNEL_SYMBOLS: &str = include_str!(env!("KERNEL_SYM"));

/**
 * Returns the immutable reference to the OldKernel core symbols
 */
pub fn kernel_symbols() -> &'static str {
    KERNEL_SYMBOLS
}
