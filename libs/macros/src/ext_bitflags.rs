/**
 * Extends `bitflags` macro adding, for each flag, an
 * `is_<lower_flags_name>(&)` method that is simply a
 * `self.contains(Self::FLAG_NAME)`
 */
#[macro_export]
macro_rules! ext_bitflags {
    (
        $(#[$Comments:meta])*
        pub struct $StructName:ident: $BitsType:ty {
            $(
                $(#[$Inner:ident $($Args:tt)*])*
                const $Flag:ident = $Value:expr;
            )+
        }
    ) => {
        macros::bitflags! {
            pub struct $StructName : $BitsType {
                $(
                    $(#[$Inner $($Args)*])*
                    const $Flag = $Value;
                )+
            }
        }

        impl $StructName {
            macros::paste! {
                $(
                    pub fn [<is_ $Flag:lower>](&self) -> bool {
                        self.contains(Self::$Flag)
                    }
                )+
            }
        }
    };
    (
        $(#[$Comments:meta])*
        struct $StructName:ident: $BitsType:ty {
            $(
                $(#[$Inner:ident $($args:tt)*])*
                const $Flag:ident = $Value:expr;
            )+
        }
    ) => {
        macros::bitflags! {
            struct $StructName : $BitsType {
                $(
                    $(#[$inner $($args)*])*
                    const $Flag = $value;
                )+
            }
        }

        impl $StructName {
            macros::paste! {
                $(
                    pub fn [<is_ $Flag:lower>](&self) -> bool {
                        self.contains(Self::$Flag)
                    }
                )+
            }
        }
    }
}
