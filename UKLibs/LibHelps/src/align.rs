/*! Value alignment utility functions */

/**
 * Returns the `value` down aligned to the nearest value multiple of
 * `alignment`
 */
pub const fn align_down(value: usize, alignment: usize) -> usize {
    value & !(alignment - 1)
}

/**
 * Returns the `value` up aligned to the nearest value multiple of
 * `alignment`
 */
pub const fn align_up(value: usize, alignment: usize) -> usize {
    let align_mask = alignment - 1;

    if value & align_mask != 0 {
        (value | align_mask) + 1
    } else {
        value
    }
}
