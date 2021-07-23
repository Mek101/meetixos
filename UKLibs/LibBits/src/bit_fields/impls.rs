/*! `BitField` implementations */

use core::ops::{
    Bound,
    Range,
    RangeBounds
};

use crate::bit_fields::{
    BitFindMode,
    TBitArray,
    TBitFields
};

macro_rules! impl_bit_fields_for_numeric {
    ($($Type:ty)*) => {$(
        impl TBitFields for $Type {
            const BIT_LEN: usize = Self::BITS as usize;

            #[inline]
            fn bit_at(&self, bit_index: usize) -> bool {
                assert!(bit_index < Self::BIT_LEN);

                (*self & (1 << bit_index)) != 0
            }

            #[inline]
            fn bits_at<T>(&self, bits_range: T) -> Self
                where T: RangeBounds<usize> {
                let bits_range = normalize_range(bits_range, Self::BIT_LEN);

                assert!(bits_range.start < Self::BIT_LEN);
                assert!(bits_range.end <= Self::BIT_LEN);
                assert!(bits_range.start < bits_range.end);

                /* shift away high bits */
                let bits = *self << (Self::BIT_LEN - bits_range.end)
                                 >> (Self::BIT_LEN - bits_range.end);

                /* shift away low bits */
                bits >> bits_range.start
            }

            #[inline]
            fn set_bit(&mut self, bit_index: usize, bit_value: bool) -> &mut Self {
                assert!(bit_index < Self::BIT_LEN);

                if bit_value {
                    *self |= 1 << bit_index;
                } else {
                    *self &= !(1 << bit_index);
                }
                self
            }

            #[inline]
            fn set_bits<T>(&mut self, bits_range: T, bits_values: Self) -> &mut Self
                where T: RangeBounds<usize> {
                let bits_range = normalize_range(bits_range, Self::BIT_LEN);

                assert!(bits_range.start < Self::BIT_LEN);
                assert!(bits_range.end <= Self::BIT_LEN);
                assert!(bits_range.start < bits_range.end);
                assert!(bits_values << (Self::BIT_LEN - (bits_range.end - bits_range.start))
                                    >> (Self::BIT_LEN - (bits_range.end - bits_range.start))
                        == bits_values,
                        "value does not fit into bit range");

                let bitmask = !(Self::MAX << (Self::BIT_LEN - bits_range.end)
                                          >> (Self::BIT_LEN - bits_range.end)
                                          >> bits_range.start
                                          << bits_range.start);

                /* set bits */
                *self = (*self & bitmask) | (bits_values << bits_range.start);
                self
            }

            #[inline]
            fn find_bit(&self, bit_value: bool, find_mode: BitFindMode) -> Option<usize> {
                if (!bit_value && *self == 0) || (bit_value && *self == Self::MAX) {
                    /* when searching a <bit_value = 0> or a <bit_value = 1> and <self>
                     * is respectively all 0 or all 1 immediately return the first or the
                     * last bit according to the <find_mode>
                     */
                    match find_mode {
                        BitFindMode::Regular => Some(0),
                        BitFindMode::Reverse => Some(Self::BIT_LEN - 1)
                    }
                } else {
                    match find_mode {
                        BitFindMode::Regular => {
                            for bit_index in 0..Self::BIT_LEN {
                                if self.bit_at(bit_index) == bit_value {
                                    return Some(bit_index);
                                }
                            }
                            None
                        },
                        BitFindMode::Reverse => {
                            for bit_index in Self::BIT_LEN - 1..=0 {
                                if self.bit_at(bit_index) == bit_value {
                                    return Some(bit_index);
                                }
                            }
                            None
                        }
                    }
                }
            }
        }
    )*};
}

impl_bit_fields_for_numeric! {
    u8 u16 u32 u64 usize u128 i8 i16 i32 i64 isize i128
}

impl<T> TBitArray<T> for [T] where T: TBitFields {
    #[inline]
    fn bit_len(&self) -> usize {
        self.len() * T::BIT_LEN
    }

    #[inline]
    fn bit_at(&self, bit_index: usize) -> bool {
        let slice_index = bit_index / T::BIT_LEN;
        let bit_index = bit_index % T::BIT_LEN;

        self[slice_index].bit_at(bit_index)
    }

    #[inline]
    fn bits_at<R>(&self, bits_range: R) -> T
        where R: RangeBounds<usize> {
        let bits_range = normalize_range(bits_range, self.bit_len());

        assert!(bits_range.len() <= T::BIT_LEN);

        let slice_start = bits_range.start / T::BIT_LEN;
        let slice_end = bits_range.end / T::BIT_LEN;
        let bit_start = bits_range.start % T::BIT_LEN;
        let bit_end = bits_range.end % T::BIT_LEN;

        assert!(slice_end - slice_start <= 1);

        if slice_start == slice_end {
            self[slice_start].bits_at(bit_start..bit_end)
        } else if bit_end == 0 {
            self[slice_start].bits_at(bit_start..T::BIT_LEN)
        } else {
            let mut value = self[slice_start].bits_at(bit_start..T::BIT_LEN);

            value.set_bits(T::BIT_LEN - bit_start..bits_range.len(),
                           self[slice_end].bits_at(0..bit_end));
            value
        }
    }

    #[inline]
    fn set_bit(&mut self, bit_index: usize, bit_value: bool) {
        let slice_index = bit_index / T::BIT_LEN;
        let bit_index = bit_index % T::BIT_LEN;

        self[slice_index].set_bit(bit_index, bit_value);
    }

    #[inline]
    fn set_bits<R>(&mut self, bits_range: R, bits_values: T)
        where R: RangeBounds<usize> {
        let bits_range = normalize_range(bits_range, self.bit_len());

        assert!(bits_range.len() <= T::BIT_LEN);

        let slice_start = bits_range.start / T::BIT_LEN;
        let slice_end = bits_range.end / T::BIT_LEN;
        let bit_start = bits_range.start % T::BIT_LEN;
        let bit_end = bits_range.end % T::BIT_LEN;

        assert!(slice_end - slice_start <= 1);

        if slice_start == slice_end {
            self[slice_start].set_bits(bit_start..bit_end, bits_values);
        } else if bit_end == 0 {
            self[slice_start].set_bits(bit_start..T::BIT_LEN, bits_values);
        } else {
            self[slice_start].set_bits(bit_start..T::BIT_LEN,
                                       bits_values.bits_at(0..T::BIT_LEN - bit_start));
            self[slice_end].set_bits(0..bit_end,
                                     bits_values.bits_at(T::BIT_LEN - bit_start
                                                         ..T::BIT_LEN));
        }
    }

    #[inline]
    fn find_bit(&self, bit_value: bool, find_mode: BitFindMode) -> Option<usize> {
        match find_mode {
            BitFindMode::Regular => {
                for (byte_index, value) in self.iter().enumerate() {
                    if let Some(bit_index) = value.find_bit(bit_value, find_mode) {
                        return Some(byte_index * T::BIT_LEN + bit_index);
                    }
                }
                None
            },
            BitFindMode::Reverse => {
                for (byte_index, value) in self.iter().rev().enumerate() {
                    if let Some(bit_index) = value.find_bit(bit_value, find_mode) {
                        return Some(byte_index * T::BIT_LEN + bit_index);
                    }
                }
                None
            }
        }
    }
}

/**
 * Normalizes the given generic `RangeBounds` to a `Range`
 */
fn normalize_range<T>(range: T, bits_len: usize) -> Range<usize>
    where T: RangeBounds<usize> {
    let start_bound = match range.start_bound() {
        Bound::Included(&start_value) => start_value,
        Bound::Excluded(&start_value) => start_value + 1,
        Bound::Unbounded => 0
    };
    let end_bound = match range.end_bound() {
        Bound::Included(&end_value) => end_value + 1,
        Bound::Excluded(&end_value) => end_value,
        Bound::Unbounded => bits_len
    };

    start_bound..end_bound
}
