/*! Bit flags wrapper */

use core::{
    marker::PhantomData,
    ops::{
        BitAnd,
        BitAndAssign,
        BitOr,
        BitOrAssign,
        BitXor,
        BitXorAssign,
        Not
    }
};

use crate::fields::BitFields;

/**
 * Safe wrapper for a bit flags
 */
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BitFlags<B, T>
    where B: BitFields + Default + Copy,
          T: BitFlagsValues {
    m_bits: B,
    _unused: PhantomData<T>
}

impl<B, T> BitFlags<B, T>
    where B: BitFields + Default + Copy,
          T: BitFlagsValues
{
    /**
     * Constructs a zeroed `BitFlags`
     */
    pub fn new_zero() -> Self {
        Self { m_bits: B::default(),
               _unused: PhantomData }
    }

    /**
     * Constructs a `BitFlags` interpreting the raw value given.
     *
     * Fails if `raw_bits` contains invalid bits
     */
    pub fn from_raw(raw_bits: B) -> Option<Self> {
        for bit_index in 0..B::BIT_LEN {
            if !T::is_bit_significant(bit_index) {
                return None;
            }
        }
        Some(Self { m_bits: raw_bits,
                    _unused: PhantomData })
    }

    /**
     * Constructs a `BitFlags` interpreting the raw value given and
     * truncating out invalid bits
     */
    pub fn from_raw_truncate(mut raw_bits: B) -> Self {
        for bit_index in 0..B::BIT_LEN {
            if !T::is_bit_significant(bit_index) {
                raw_bits.set_bit(bit_index, false);
            }
        }

        Self { m_bits: raw_bits,
               _unused: PhantomData }
    }

    /**
     * Enables the bit corresponding to the given value
     */
    pub fn set_enabled(&mut self, value: T) {
        self.set(value, true);
    }

    /**
     * Disables the bit corresponding to the given value
     */
    pub fn set_disabled(&mut self, value: T) {
        self.set(value, false);
    }

    /**
     * Sets for the bit corresponding to `value` the `enable` value
     */
    pub fn set(&mut self, value: T, enable: bool) {
        let bit_index = value.into_raw_bit_index();
        assert!(bit_index < B::BIT_LEN);

        self.m_bits.set_bit(bit_index, enable);
    }

    /**
     * Returns whether the bit corresponding to `value` is enabled
     */
    pub fn is_enabled(&self, value: T) -> bool {
        let bit_index = value.into_raw_bit_index();
        assert!(bit_index < B::BIT_LEN);

        self.m_bits.bit_at(bit_index)
    }

    /**
     * Returns whether the bit corresponding to `value` is disabled
     */
    pub fn is_disabled(&self, value: T) -> bool {
        !self.is_enabled(value)
    }
}

impl<B, T> BitOr for BitFlags<B, T>
    where B: BitFields + Default + Copy + BitOr<Output = B>,
          T: BitFlagsValues
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { m_bits: self.m_bits | rhs.m_bits,
               _unused: PhantomData }
    }
}

impl<B, T> BitOrAssign for BitFlags<B, T>
    where B: BitFields + Default + Copy + BitOrAssign,
          T: BitFlagsValues
{
    fn bitor_assign(&mut self, rhs: Self) {
        self.m_bits |= rhs.m_bits
    }
}

impl<B, T> BitOr<T> for BitFlags<B, T>
    where B: BitFields + Default + Copy,
          T: BitFlagsValues
{
    type Output = Self;

    fn bitor(self, rhs: T) -> Self::Output {
        let mut self_clone = self.clone();

        self_clone.set_enabled(rhs);
        self_clone
    }
}

impl<B, T> BitOrAssign<T> for BitFlags<B, T>
    where B: BitFields + Default + Copy,
          T: BitFlagsValues
{
    fn bitor_assign(&mut self, rhs: T) {
        self.set_enabled(rhs);
    }
}

impl<B, T> BitAnd for BitFlags<B, T>
    where B: BitFields + Default + Copy + BitAnd<Output = B>,
          T: BitFlagsValues
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self { m_bits: self.m_bits & rhs.m_bits,
               _unused: PhantomData }
    }
}

impl<B, T> BitAndAssign for BitFlags<B, T>
    where B: BitFields + Default + Copy + BitAndAssign,
          T: BitFlagsValues
{
    fn bitand_assign(&mut self, rhs: Self) {
        self.m_bits &= rhs.m_bits
    }
}

impl<B, T> BitAnd<T> for BitFlags<B, T>
    where B: BitFields + Default + Copy,
          T: BitFlagsValues + Copy
{
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        let mut self_zero = Self::new_zero();

        if self.is_enabled(rhs) {
            self_zero.set_enabled(rhs);
        }

        self_zero
    }
}

impl<B, T> BitAndAssign<T> for BitFlags<B, T>
    where B: BitFields + Default + Copy,
          T: BitFlagsValues + Copy
{
    fn bitand_assign(&mut self, rhs: T) {
        self.set(rhs, self.is_enabled(rhs));
    }
}

impl<B, T> BitXor for BitFlags<B, T>
    where B: BitFields + Default + Copy + BitXor<Output = B>,
          T: BitFlagsValues + Copy
{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { m_bits: self.m_bits ^ rhs.m_bits,
               _unused: PhantomData }
    }
}

impl<B, T> BitXorAssign for BitFlags<B, T>
    where B: BitFields + Default + Copy + BitXorAssign,
          T: BitFlagsValues + Copy
{
    fn bitxor_assign(&mut self, rhs: Self) {
        self.m_bits ^= rhs.m_bits;
    }
}

impl<B, T> BitXor<T> for BitFlags<B, T>
    where B: BitFields + Default + Copy + BitXor<Output = B>,
          T: BitFlagsValues + Copy
{
    type Output = Self;

    fn bitxor(self, rhs: T) -> Self::Output {
        let mut rhs_bf = Self::new_zero();
        rhs_bf.set_enabled(rhs);

        Self { m_bits: self.m_bits ^ rhs_bf.m_bits,
               _unused: PhantomData }
    }
}

impl<B, T> BitXorAssign<T> for BitFlags<B, T>
    where B: BitFields + Default + Copy + BitXorAssign,
          T: BitFlagsValues + Copy
{
    fn bitxor_assign(&mut self, rhs: T) {
        let mut rhs_bf = Self::new_zero();
        rhs_bf.set_enabled(rhs);

        self.m_bits ^= rhs_bf.m_bits;
    }
}

impl<B, T> Not for BitFlags<B, T>
    where B: BitFields + Default + Copy,
          T: BitFlagsValues + Copy
{
    type Output = Self;

    fn not(self) -> Self::Output {
        Self { m_bits: !self.m_bits,
               _unused: PhantomData }
    }
}

/**
 * Interface which must be implemented by the enumeration values
 */
pub trait BitFlagsValues {
    /**
     * Returns the index of the bit which the variant represent
     */
    fn into_raw_bit_index(self) -> usize;

    /**
     * Returns whether the given `bit_index` represent a flag value
     */
    fn is_bit_significant(bit_index: usize) -> bool;
}
