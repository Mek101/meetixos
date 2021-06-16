/*! Bit field support for numeric types */

use core::ops::RangeBounds;

pub mod impls;

/**
 * Lists the available modes for `BitFields::find_bit()`
 */
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BitFindMode {
    /**
     * Performs a regular from first to last search
     */
    Regular,

    /**
     * Performs a reverse from last to first search
     */
    Reverse
}

/**
 * Exposes methods to get/set specific bits or ranges of bits.
 *
 * Each method interprets index 0 as the least significant bit, while index
 * `.len() - 1` as the most significant bit
 */
pub trait BitFields: Sized {
    /**
     * Number of bits which composes `Self`
     *
     * NOTE: I know that a `BITS` property exists for any integer type, but
     *       is typed as `i32`, which forces to continuous casts, this is
     *       just a convenience
     */
    const BIT_LEN: usize;

    /**
     * Clears all the bits into this bit fields
     */
    fn clear_bits(&mut self);

    /**
     * Obtains the bit value at the index `bit_index`
     */
    fn bit_at(&self, bit_index: usize) -> bool;

    /**
     * Obtains the range of bit values at the given `bits_range`
     */
    fn bits_at<T>(&self, bits_range: T) -> Self
        where T: RangeBounds<usize>;

    /**
     * Sets the bit at the index `bit_index` with the given `bit_value`
     */
    fn set_bit(&mut self, bit_index: usize, bit_value: bool) -> &mut Self;

    /**
     * Sets the range of bit values at the given `bits_range` with the given
     * `bit_value`
     */
    fn set_bits<T>(&mut self, bits_range: T, bits_values: Self) -> &mut Self
        where T: RangeBounds<usize>;

    /**
     * Returns the index of the first/last bit (according to the given
     * `BitFindMode`) which have the given `bit_value`
     */
    fn find_bit(&self, bit_value: bool, find_mode: BitFindMode) -> Option<usize>;
}

/**
 * Extends the `BitField` trait to be used for array slices too
 */
pub trait BitArray<T>
    where T: BitFields {
    /**
     * Returns the bits in this bit array
     */
    fn bit_len(&self) -> usize;

    /**
     * Clears all the bits into this bit fields array
     */
    fn clear_bits(&mut self);

    /**
     * Obtains the bit value at the index `bit_index`
     */
    fn bit_at(&self, bit_index: usize) -> bool;

    /**
     * Obtains the range of bit values at the given `bits_range`
     */
    fn bits_at<R>(&self, bits_range: R) -> T
        where R: RangeBounds<usize>;

    /**
     * Sets the bit at the index `bit_index` with the given `bit_value`
     */
    fn set_bit(&mut self, bit_index: usize, bit_value: bool);

    /**
     * Sets the range of bit values at the given `bits_range` with the given
     * `bit_value`
     */
    fn set_bits<R>(&mut self, bits_range: R, bits_values: T)
        where R: RangeBounds<usize>;

    /**
     * Returns the index of the first/last bit (according to the given
     * `BitFindMode`) which have the given `bit_value`
     */
    fn find_bit(&self, bit_value: bool, find_mode: BitFindMode) -> Option<usize>;
}
