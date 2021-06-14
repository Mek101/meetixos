#[macro_export(local_inner_macros)]
macro_rules! bitflags {
    (
        $(#[$outer:meta])*
        pub struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )+
        }
    ) => {
        __bitflags! {
            $(#[$outer])*
            (pub) $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    $Flag = $value;
                )+
            }
        }
    };
    (
        $(#[$outer:meta])*
        struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )+
        }
    ) => {
        __bitflags! {
            $(#[$outer])*
            () $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    $Flag = $value;
                )+
            }
        }
    };
    (
        $(#[$outer:meta])*
        pub ($($vis:tt)+) struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )+
        }
    ) => {
        __bitflags! {
            $(#[$outer])*
            (pub ($($vis)+)) $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    $Flag = $value;
                )+
            }
        }
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __bitflags {
    (
        $(#[$outer:meta])*
        ($($vis:tt)*) $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $Flag:ident = $value:expr;
            )+
        }
    ) => {
        $(#[$outer])*
        #[derive(Copy, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
        $($vis)* struct $BitFlags {
            bits: $T,
        }

        __impl_bitflags! {
            $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    $Flag = $value;
                )+
            }
        }
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __fn_bitflags {
    (
        $(# $attr_args:tt)*
        const fn $($item:tt)*
    ) => {
        $(# $attr_args)*
        const fn $($item)*
    };
    (
        $(# $attr_args:tt)*
        pub const fn $($item:tt)*
    ) => {
        $(# $attr_args)*
        pub const fn $($item)*
    };
    (
        $(# $attr_args:tt)*
        pub const unsafe fn $($item:tt)*
    ) => {
        $(# $attr_args)*
        pub const unsafe fn $($item)*
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __impl_bitflags {
    (
        $BitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident = $value:expr;
            )+
        }
    ) => {
        impl core::fmt::Debug for $BitFlags {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                // This convoluted approach is to handle #[cfg]-based flag
                // omission correctly. For example it needs to support:
                //
                //    #[cfg(unix)] const A: Flag = /* ... */;
                //    #[cfg(windows)] const B: Flag = /* ... */;

                // Unconditionally define a check for every flag, even disabled
                // ones.
                #[allow(non_snake_case)]
                trait __BitFlags {
                    $(
                        #[inline]
                        fn $Flag(&self) -> bool { false }
                    )+
                }

                // Conditionally override the check for just those flags that
                // are not #[cfg]ed away.
                impl __BitFlags for $BitFlags {
                    $(
                        __impl_bitflags! {
                            #[allow(deprecated)]
                            #[inline]
                            $(? #[$attr $($args)*])*
                            fn $Flag(&self) -> bool {
                                if Self::$Flag.bits == 0 && self.bits != 0 {
                                    false
                                } else {
                                    self.bits & Self::$Flag.bits == Self::$Flag.bits
                                }
                            }
                        }
                    )+
                }

                let mut first = true;
                $(
                    if <$BitFlags as __BitFlags>::$Flag(self) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(__bitflags_stringify!($Flag))?;
                    }
                )+
                let extra_bits = self.bits & !$BitFlags::all().bits();
                if extra_bits != 0 {
                    if !first {
                        f.write_str(" | ")?;
                    }
                    first = false;
                    f.write_str("0x")?;
                    core::fmt::LowerHex::fmt(&extra_bits, f)?;
                }
                if first {
                    f.write_str("(empty)")?;
                }
                Ok(())
            }
        }
        impl core::fmt::Binary for $BitFlags {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::Binary::fmt(&self.bits, f)
            }
        }
        impl core::fmt::Octal for $BitFlags {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::Octal::fmt(&self.bits, f)
            }
        }
        impl core::fmt::LowerHex for $BitFlags {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::LowerHex::fmt(&self.bits, f)
            }
        }
        impl core::fmt::UpperHex for $BitFlags {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::UpperHex::fmt(&self.bits, f)
            }
        }

        #[allow(dead_code)]
        impl $BitFlags {
            $(
                $(#[$attr $($args)*])*
                pub const $Flag: $BitFlags = $BitFlags { bits: $value };
            )+

            __fn_bitflags! {
                /// Returns an empty set of flags
                #[inline]
                pub const fn empty() -> $BitFlags {
                    $BitFlags { bits: 0 }
                }
            }

            __fn_bitflags! {
                /// Returns the set containing all flags.
                #[inline]
                pub const fn all() -> $BitFlags {
                    // See `Debug::fmt` for why this approach is taken.
                    #[allow(non_snake_case)]
                    trait __BitFlags {
                        $(
                            const $Flag: $T = 0;
                        )+
                    }
                    impl __BitFlags for $BitFlags {
                        $(
                            __impl_bitflags! {
                                #[allow(deprecated)]
                                $(? #[$attr $($args)*])*
                                const $Flag: $T = Self::$Flag.bits;
                            }
                        )+
                    }
                    $BitFlags { bits: $(<$BitFlags as __BitFlags>::$Flag)|+ }
                }
            }

            __fn_bitflags! {
                /// Returns the raw value of the flags currently stored.
                #[inline]
                pub const fn bits(&self) -> $T {
                    self.bits
                }
            }

            /// Convert from underlying bit representation, unless that
            /// representation contains bits that do not correspond to a flag.
            #[inline]
            pub fn from_bits(bits: $T) -> core::option::Option<$BitFlags> {
                if (bits & !$BitFlags::all().bits()) == 0 {
                    core::option::Option::Some($BitFlags { bits })
                } else {
                    core::option::Option::None
                }
            }

            __fn_bitflags! {
                /// Convert from underlying bit representation, dropping any bits
                /// that do not correspond to flags.
                #[inline]
                pub const fn from_bits_truncate(bits: $T) -> $BitFlags {
                    $BitFlags { bits: bits & $BitFlags::all().bits }
                }
            }

            __fn_bitflags! {
                /// Convert from underlying bit representation, preserving all
                /// bits (even those not corresponding to a defined flag).
                #[inline]
                pub const unsafe fn from_bits_unchecked(bits: $T) -> $BitFlags {
                    $BitFlags { bits }
                }
            }

            __fn_bitflags! {
                /// Returns `true` if no flags are currently stored.
                #[inline]
                pub const fn is_empty(&self) -> bool {
                    self.bits() == $BitFlags::empty().bits()
                }
            }

            __fn_bitflags! {
                /// Returns `true` if all flags are currently set.
                #[inline]
                pub const fn is_all(&self) -> bool {
                    self.bits == $BitFlags::all().bits
                }
            }

            __fn_bitflags! {
                /// Returns `true` if there are flags common to both `self` and `other`.
                #[inline]
                pub const fn intersects(&self, other: $BitFlags) -> bool {
                    !$BitFlags{ bits: self.bits & other.bits}.is_empty()
                }
            }

            __fn_bitflags! {
                /// Returns `true` all of the flags in `other` are contained within `self`.
                #[inline]
                pub const fn contains(&self, other: $BitFlags) -> bool {
                    (self.bits & other.bits) == other.bits
                }
            }

            /// Inserts the specified flags in-place.
            #[inline]
            pub fn insert(&mut self, other: $BitFlags) {
                self.bits |= other.bits;
            }

            /// Removes the specified flags in-place.
            #[inline]
            pub fn remove(&mut self, other: $BitFlags) {
                self.bits &= !other.bits;
            }

            /// Toggles the specified flags in-place.
            #[inline]
            pub fn toggle(&mut self, other: $BitFlags) {
                self.bits ^= other.bits;
            }

            /// Inserts or removes the specified flags depending on the passed value.
            #[inline]
            pub fn set(&mut self, other: $BitFlags, value: bool) {
                if value {
                    self.insert(other);
                } else {
                    self.remove(other);
                }
            }
        }

        impl core::ops::BitOr for $BitFlags {
            type Output = $BitFlags;

            /// Returns the union of the two sets of flags.
            #[inline]
            fn bitor(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits | other.bits }
            }
        }

        impl core::ops::BitOrAssign for $BitFlags {

            /// Adds the set of flags.
            #[inline]
            fn bitor_assign(&mut self, other: $BitFlags) {
                self.bits |= other.bits;
            }
        }

        impl core::ops::BitXor for $BitFlags {
            type Output = $BitFlags;

            /// Returns the left flags, but with all the right flags toggled.
            #[inline]
            fn bitxor(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits ^ other.bits }
            }
        }

        impl core::ops::BitXorAssign for $BitFlags {

            /// Toggles the set of flags.
            #[inline]
            fn bitxor_assign(&mut self, other: $BitFlags) {
                self.bits ^= other.bits;
            }
        }

        impl core::ops::BitAnd for $BitFlags {
            type Output = $BitFlags;

            /// Returns the intersection between the two sets of flags.
            #[inline]
            fn bitand(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits & other.bits }
            }
        }

        impl core::ops::BitAndAssign for $BitFlags {

            /// Disables all flags disabled in the set.
            #[inline]
            fn bitand_assign(&mut self, other: $BitFlags) {
                self.bits &= other.bits;
            }
        }

        impl core::ops::Sub for $BitFlags {
            type Output = $BitFlags;

            /// Returns the set difference of the two sets of flags.
            #[inline]
            fn sub(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits & !other.bits }
            }
        }

        impl core::ops::SubAssign for $BitFlags {

            /// Disables all flags enabled in the set.
            #[inline]
            fn sub_assign(&mut self, other: $BitFlags) {
                self.bits &= !other.bits;
            }
        }

        impl core::ops::Not for $BitFlags {
            type Output = $BitFlags;

            /// Returns the complement of this set of flags.
            #[inline]
            fn not(self) -> $BitFlags {
                $BitFlags { bits: !self.bits } & $BitFlags::all()
            }
        }

        impl core::iter::Extend<$BitFlags> for $BitFlags {
            fn extend<T: core::iter::IntoIterator<Item=$BitFlags>>(&mut self, iterator: T) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }

        impl core::iter::FromIterator<$BitFlags> for $BitFlags {
            fn from_iter<T: core::iter::IntoIterator<Item=$BitFlags>>(iterator: T) -> $BitFlags {
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
    };

    // Every attribute that the user writes on a const is applied to the
    // corresponding const that we generate, but within the implementation of
    // Debug and all() we want to ignore everything but #[cfg] attributes. In
    // particular, including a #[deprecated] attribute on those items would fail
    // to compile.
    // https://github.com/bitflags/bitflags/issues/109
    //
    // Input:
    //
    //     ? #[cfg(feature = "advanced")]
    //     ? #[deprecated(note = "Use somthing else.")]
    //     ? #[doc = r"High quality documentation."]
    //     fn f() -> i32 { /* ... */ }
    //
    // Output:
    //
    //     #[cfg(feature = "advanced")]
    //     fn f() -> i32 { /* ... */ }
    (
        $(#[$filtered:meta])*
        ? #[cfg $($cfgargs:tt)*]
        $(? #[$rest:ident $($restargs:tt)*])*
        fn $($item:tt)*
    ) => {
        __impl_bitflags! {
            $(#[$filtered])*
            #[cfg $($cfgargs)*]
            $(? #[$rest $($restargs)*])*
            fn $($item)*
        }
    };
    (
        $(#[$filtered:meta])*
        // $next != `cfg`
        ? #[$next:ident $($nextargs:tt)*]
        $(? #[$rest:ident $($restargs:tt)*])*
        fn $($item:tt)*
    ) => {
        __impl_bitflags! {
            $(#[$filtered])*
            // $next filtered out
            $(? #[$rest $($restargs)*])*
            fn $($item)*
        }
    };
    (
        $(#[$filtered:meta])*
        fn $($item:tt)*
    ) => {
        $(#[$filtered])*
        fn $($item)*
    };

    // Every attribute that the user writes on a const is applied to the
    // corresponding const that we generate, but within the implementation of
    // Debug and all() we want to ignore everything but #[cfg] attributes. In
    // particular, including a #[deprecated] attribute on those items would fail
    // to compile.
    // https://github.com/bitflags/bitflags/issues/109
    //
    // const version
    //
    // Input:
    //
    //     ? #[cfg(feature = "advanced")]
    //     ? #[deprecated(note = "Use somthing else.")]
    //     ? #[doc = r"High quality documentation."]
    //     const f: i32 { /* ... */ }
    //
    // Output:
    //
    //     #[cfg(feature = "advanced")]
    //     const f: i32 { /* ... */ }
    (
        $(#[$filtered:meta])*
        ? #[cfg $($cfgargs:tt)*]
        $(? #[$rest:ident $($restargs:tt)*])*
        const $($item:tt)*
    ) => {
        __impl_bitflags! {
            $(#[$filtered])*
            #[cfg $($cfgargs)*]
            $(? #[$rest $($restargs)*])*
            const $($item)*
        }
    };
    (
        $(#[$filtered:meta])*
        // $next != `cfg`
        ? #[$next:ident $($nextargs:tt)*]
        $(? #[$rest:ident $($restargs:tt)*])*
        const $($item:tt)*
    ) => {
        __impl_bitflags! {
            $(#[$filtered])*
            // $next filtered out
            $(? #[$rest $($restargs)*])*
            const $($item)*
        }
    };
    (
        $(#[$filtered:meta])*
        const $($item:tt)*
    ) => {
        $(#[$filtered])*
        const $($item)*
    };
}

// Same as std::stringify but callable from __impl_bitflags, which needs to use
// local_inner_macros so can only directly call macros from this crate.
#[macro_export]
#[doc(hidden)]
macro_rules! __bitflags_stringify {
    ($s:ident) => {
        stringify!($s)
    };
}
