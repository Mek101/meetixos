/** # Manageable C-like enum
 *
 * Generates a C-like enumeration that is convertible to/from an integer
 * type determined in declaration and can be printable with custom messages.
 *
 * C-like enumeration means that each variant represent a number and can be
 * represented by a number.
 *
 * The convertions and the printability are implemented through the
 * following traits:
 * * [`TryFrom`] for the choosen type and `usize`
 * * [`Into`] for the choosen type and `usize`
 * * [`Display`]
 *
 * ```rust
 * #[macro_use]
 * extern crate macros;
 *
 * c_handy_enum! {
 *     /** Doc comments are printed into the documentation
 *      */
 *     pub enum MyErrorEnum: u16 {
 *         /** Each variant can be composed like:
 *          * `VariantName = int_value => "Associated string in Display"`
 *          *
 *          * The integer type must be compatible with the type given
 *          * after the enum name
 *          */
 *         FirstError = 0 => "Associated Error Message",
 *         SecondError = 1 => "The other associated message",
 *     }
 * }
 *
 * c_handy_enum! {
 *     /** Doc comments are printed into the documentation
 *      */
 *     pub enum AnotherEnum: u16 {
 *         /** Otherwise, if the associated variant's message is not
 *          * given the Display will print `EnumName::VariantName`
 *          */
 *         MyFirstVariant = 1025,
 *         MySecondVariant = 0xFFF,
 *     }
 * }
 * ```
 *
 * [`TryFrom`]: core::convert::TryFrom
 * [`Into`]: core::convert::Into
 * [`Display`]: core::fmt::Display
 */
#[macro_export]
macro_rules! c_handy_enum {
    (
        $(#[$Comments:meta])*
        pub enum $EnumName:ident : $ToFromType:ident {
            $(
                $(#[$Meta:meta])*
                $Variant:ident = $Index:tt => $String:tt,
            )*
        }
    ) => {
        c_handy_enum_common! {
            $(#[$Comments])*
            pub enum $EnumName : $ToFromType {
                $(
                    $(#[$Meta])*
                    $Variant = $Index,
                )*
            }
        }

        impl core::fmt::Display for $EnumName {
            #[doc = "Formats the value using the given formatter"]
            fn fmt(&self, f: &mut core::Formatter<'_>) -> core::fmt::Result {
                match self {
                    $(
                        $EnumName::$Variant => f.write_str($String),
                    )*
                }
            }
        }
    };
    (
        $(#[$Comments:meta])*
        pub enum $EnumName:ident : $ToFromType:ident {
            $(
                $(#[$Meta:meta])*
                $Variant:ident = $Index:tt,
            )*
        }
    ) => {
        c_handy_enum_common! {
            $(#[$Comments])*
            pub enum $EnumName : $ToFromType {
                $(
                    $(#[$Meta])*
                    $Variant = $Index,
                )*
            }
        }

        impl core::fmt::Display for $EnumName {
            #[doc = "Formats the value using the given formatter"]
            fn fmt(&self, f: &mut core::Formatter<'_>) -> core::fmt::Result {
                match self {
                    $(
                        $EnumName::$Variant => write!(f, "{}::{}", stringify!($EnumName), stringify!($Variant)),
                    )*
                }
            }
        }
    }
}

#[macro_export]
macro_rules! c_handy_enum_common {
    (
        $(#[$Comments:meta])*
        pub enum $EnumName:ident : $ToFromType:ident {
            $(
                $(#[$Meta:meta])*
                $Variant:ident = $Index:tt,
            )+
        }
    ) => {
        $(#[$Comments])*
        #[repr($ToFromType)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
        pub enum $EnumName {
            $(
                $(#[$Meta])*
                $Variant = $Index,
            )*
        }

        impl $EnumName {
            #[doc = "Number of variants for this enumeration"]
            pub const COUNT: usize = macros::count_reps!($($Variant,)*);

            #[doc = "Returns an iterator which starts from the first to the last variant"]
            pub fn iter_all() -> impl Iterator<Item = Self> {
                use core::convert::TryFrom;
                macros::paste! {
                    [<$EnumName Iter>] { m_current: $EnumName::try_from(0usize).ok() }
                }
            }

            #[doc = "Returns an iterator which starts from this variant"]
            pub fn iter_from_this(&self) -> impl Iterator<Item = Self> {
                macros::paste! {
                    [<$EnumName Iter>] { m_current: Some(self.clone()) }
                }
            }

            #[doc = "Returns an iterator which starts from the first to this variant"]
            pub fn iter_to_this(&self) -> impl Iterator<Item = Self> {
                use core::convert::TryFrom;
                macros::paste! {
                    [<$EnumName RangeIter>] {
                        m_current: $EnumName::try_from(0usize).ok(),
                        m_last: self.clone(),
                        m_inclusive: true
                    }
                }
            }

            #[doc = "Returns an iterator which starts from the first to the variant before this"]
            pub fn iter_until_this(&self) -> impl Iterator<Item = Self> {
                use core::convert::TryFrom;
                macros::paste! {
                    [<$EnumName RangeIter>] {
                        m_current: $EnumName::try_from(0usize).ok(),
                        m_last: self.clone(),
                        m_inclusive: false
                    }
                }
            }

            #[doc = "Returns the current variant value as integer type"]
            pub fn as_value(&self) -> $ToFromType {
                *self as $ToFromType
            }

            #[doc = "Returns the current variant value as `usize` type"]
            pub fn as_usize(&self) -> usize {
                *self as usize
            }
        }

        impl Default for $EnumName {
            #[doc = "Returns the \"default value\" for a type"]
            fn default() -> Self {
                use core::convert::TryFrom;
                Self::try_from(0usize).unwrap()
            }
        }

        impl core::convert::TryFrom<$ToFromType> for $EnumName {
            type Error = $ToFromType;

            #[doc = "Performs the conversion"]
            fn try_from(code: $ToFromType) -> Result<Self, Self::Error> {
                match code {
                    $($Index => Ok($EnumName::$Variant),)*
                    _ => Err(code),
                }
            }
        }

        impl core::convert::TryFrom<usize> for $EnumName {
            type Error = usize;

            #[doc = "Performs the conversion"]
            fn try_from(code: usize) -> Result<Self, Self::Error> {
                match code {
                    $($Index => Ok($EnumName::$Variant),)*
                    _ => Err(code),
                }
            }
        }

        impl core::convert::Into<$ToFromType> for $EnumName {
            #[doc = "Performs the conversion"]
            fn into(self) -> $ToFromType {
                self.as_value()
            }
        }

        impl core::convert::Into<usize> for $EnumName {
            #[doc = "Performs the conversion"]
            fn into(self) -> usize {
                self.as_usize()
            }
        }

        macros::paste! {
            struct [<$EnumName Iter>] {
                m_current: Option<$EnumName>
            }

            impl core::iter::Iterator for [<$EnumName Iter>] {
                #[doc = "The type of the elements being iterated over"]
                type Item = $EnumName;

                #[doc = "Advances the iterator and returns the next value"]
                fn next(&mut self) -> Option<Self::Item> {
                    use core::convert::TryFrom;

                    let current = self.m_current;
                    if let Some(current) = current {
                        self.m_current = $EnumName::try_from(current.as_usize() + 1).ok();
                    }
                    current
                }
            }

            struct [<$EnumName RangeIter>] {
                m_current: Option<$EnumName>,
                m_last: $EnumName,
                m_inclusive: bool
            }

            impl [<$EnumName RangeIter>] {
                fn can_next(&self, current: $EnumName) -> bool {
                    if self.m_inclusive {
                        current.as_usize() + 1 <= self.m_last.as_usize()
                    } else {
                        current.as_usize() + 1 < self.m_last.as_usize()
                    }
                }
            }

            impl core::iter::Iterator for [<$EnumName RangeIter>] {
                #[doc = "The type of the elements being iterated over"]
                type Item = $EnumName;

                #[doc = "Advances the iterator and returns the next value"]
                fn next(&mut self) -> Option<Self::Item> {
                    use core::convert::TryFrom;

                    let current = self.m_current;
                    if let Some(current) = current {
                        self.m_current = if self.can_next(current) {
                            $EnumName::try_from(current.as_usize() + 1).ok()
                        } else {
                            None
                        }
                    }

                    current
                }
            }
        }
    };
}
