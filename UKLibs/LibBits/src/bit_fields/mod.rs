/*! Bit field support for numeric types */

use core::{
    marker::PhantomData,
    ops::RangeBounds
};

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

    /**
     * Returns an `Iterator` to iterate the bit-values
     */
    fn iter_bits(&self) -> BitFieldsIterator<'_, Self> {
        BitFieldsIterator::new(self)
    }
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

    /**
     * Returns an `Iterator` to iterate the bit-values
     */
    fn iter_bits(&self) -> BitArrayIterator<'_, T, Self>
        where Self: Sized {
        BitArrayIterator::new(self)
    }
}

/**
 * `BitFields` iterator
 */
pub struct BitFieldsIterator<'a, T>
    where T: BitFields {
    m_index: usize,
    m_bit_fields: &'a T
}

impl<'a, T> BitFieldsIterator<'a, T> where T: BitFields {
    /**
     * Constructs a `BitFieldsIterator` with the given `BitFields`
     */
    fn new(bit_fields: &'a T) -> Self {
        Self { m_index: 0,
               m_bit_fields: bit_fields }
    }
}

impl<'a, T> Iterator for BitFieldsIterator<'a, T> where T: BitFields {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.m_index < T::BIT_LEN {
            let bit_value = self.m_bit_fields.bit_at(self.m_index);
            self.m_index += 1;
            Some(bit_value)
        } else {
            None
        }
    }
}

/**
 * `BitArray` iterator
 */
pub struct BitArrayIterator<'a, B, T>
    where B: BitFields,
          T: BitArray<B> {
    m_index: usize,
    m_bit_array: &'a T,
    _unused: PhantomData<B>
}

impl<'a, B, T> BitArrayIterator<'a, B, T>
    where B: BitFields,
          T: BitArray<B>
{
    /**
     * Constructs a `BitArrayIterator` with the given `BitArray`
     */
    fn new(bit_array: &'a T) -> Self {
        Self { m_index: 0,
               m_bit_array: bit_array,
               _unused: PhantomData }
    }
}

impl<'a, B, T> Iterator for BitArrayIterator<'a, B, T>
    where B: BitFields,
          T: BitArray<B>
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.m_index < self.m_bit_array.bit_len() {
            let bit_value = self.m_bit_array.bit_at(self.m_index);
            self.m_index += 1;
            Some(bit_value)
        } else {
            None
        }
    }
}
