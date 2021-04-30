/** # Manageable Rust-like Enum
 *
 * Generates the boilerplate code that is necessary to have a (limited)
 * Rust-like enumeration that could contain arbitrary data in his variants,
 * can be converted to/from an integer type and can be printed with custom
 * messages.
 *
 * The conversions and the printability are implemented through the
 * following traits and methods:
 * * [`Display`]
 * * [`Into`] for the choosen type and `usize`
 * * `from()` & `from_usize()`
 *
 * ```rust
 * #[macro_use]
 * extern crate macros;
 *
 * rust_handy_enum! {
 *     pub enum WonderfulEnum: u32 {
 *         /** Each variant is composed as follow:
 *          * `VariantName = value`,
 *          * Or
 *          * `VariantName(integer_type) = value`,
 *          *
 *          * The value must be compatible with the type
 *          * given after the enum name
 *          */
 *         WonderfulVariant1 = 0,
 *         WonderfulVariant2(u16) = 1,
 *     }
 * }
 *
 * rust_handy_enum! {
 *     /** Doc comments are printed into the documentation
 *      */
 *     pub enum AnotherDataEnum: u16 {
 *         DataVariant(u64) = 0 => "The printed variant value",
 *         AnotherData(usize) = 1 => "tell me your story",
 *     }
 * }
 * ```
 *
 * [`Display`]: core::fmt::Display
 * [`Into`]: core::convert::Into
 */
#[macro_export]
macro_rules! rust_handy_enum {
    {
        $(#[$Comments:meta])*
        pub enum $EnumName:ident : $ToFromType:ident {
            $(
                $(#[$Meta:meta])*
                $Variant:ident $( ( $VariantValue:ident : $VariantType:ty ) )? = $Index:expr,
            )*
        }
    } => {
        $(#[$Comments])*
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum $EnumName {
            $( $(#[$Meta])*
            $Variant $( ( $VariantType ) )?,)*
        }

        impl core::convert::TryFrom<($ToFromType, usize)> for $EnumName {
            type Error = $ToFromType;

            #[doc = "Performs the conversion"]
            fn try_from(data: ($ToFromType, usize)) -> Result<Self, Self::Error> {
                use core::convert::TryFrom;
                match data.0 {
                    $(
                        $Index => {
                            Ok($EnumName::$Variant $(
                                    ( <$VariantType>::try_from(data.1)
                                                     .unwrap() )
                                )?)
                        },
                    )*
                    _ => Err(data.0)
                }
            }
        }

        impl core::convert::TryFrom<(usize, usize)> for $EnumName {
            type Error = usize;

            #[doc = "Performs the conversion"]
            fn try_from(data: (usize, usize)) -> Result<Self, Self::Error> {
                use core::convert::TryFrom;
                match data.0 {
                    $(
                        $Index => {
                            Ok($EnumName::$Variant $(
                                    ( <$VariantType>::try_from(data.1)
                                                     .unwrap() )
                                )?)
                        },
                    )*
                    _ => Err(data.0)
                }
            }
        }

        impl core::convert::Into<usize> for $EnumName {
            #[doc = "Performs the conversion"]
            fn into(self) -> usize {
                macros::paste! {
                    match self {
                        $(
                            $EnumName::$Variant $(
                                ([<_ $VariantValue>])
                            )? => $Index,
                        )*
                    }
                }
            }
        }

        impl core::convert::Into<$ToFromType> for $EnumName {
            #[doc = "Performs the conversion"]
            fn into(self) -> $ToFromType {
                macros::paste! {
                    match self {
                        $(
                            $EnumName::$Variant $(
                                ([<_ $VariantValue>])
                            )? => $Index,
                        )*
                    }
                }
            }
        }
    };
}
