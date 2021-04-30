/*! # Library Utils
 *
 * Contains a small list of utility functions
 */

use core::{cmp::min, slice, str};

/** # Copies an `&str` slice to an [u8] buffer
 *
 * The copied characters are the minimum of the two lengths.
 */
pub fn copy_str_to_u8_buf(dst: &mut [u8], src: &str) {
    let min = min(dst.len(), src.len());
    dst[..min].copy_from_slice(&src.as_bytes()[..min]);
}

/** # Constructs an `&str` slice from an `[u8]` slice
 *
 * Wrapper for [`u8_ptr_to_str_slice()`]
 *
 * [`u8_ptr_to_str_slice()`]: crate::str_utils::u8_ptr_to_str_slice
 */
pub fn u8_slice_to_str_slice<'a>(slice: &[u8]) -> &'a str {
    u8_ptr_to_str_slice(slice.as_ptr(), slice.len())
}

/** # Constructs an `&str` slice from raw
 *
 * Safe wrapper for [`slice::from_raw_parts()`] &
 * [`str::from_utf8_unchecked()`]
 *
 * [`slice::from_raw_parts()`]: core::slice::from_raw_parts
 * [`str::from_utf8_unchecked()`]: core::str::from_utf8_unchecked
 */
pub fn u8_ptr_to_str_slice<'a>(ptr: *const u8, len: usize) -> &'a str {
    unsafe {
        let slice = slice::from_raw_parts(ptr, len);
        str::from_utf8_unchecked(slice)
    }
}
