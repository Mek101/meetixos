/*! `Object`'s grants management */

use core::marker::PhantomData;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use bits::flags::{
    BitFlags,
    BitFlagsValues
};

use crate::objs::{
    impls::{
        dir::Dir,
        file::File,
        ipc_chan::IpcChan,
        link::Link,
        mmap::MMap,
        mutex::OsRawMutex
    },
    object::Object
};

/**
 * Permissions bits for the `Object` implementations
 */
#[derive(Debug)]
pub struct Grants<T>
    where T: Object {
    m_bits: BitFlags<u32, ObjGrantsBits>,
    _unused: PhantomData<T>
}

impl<T> Grants<T> where T: Object {
    /**
     * Constructs a zeroed `Grants`
     */
    pub fn new() -> Self {
        Self { m_bits: BitFlags::new_zero(),
               _unused: Default::default() }
    }

    /**
     * Enables the given `GrantsBit`.
     *
     * Allows, for the interested user/group, the actions associated with
     * the grant
     */
    pub fn set_enabled(&mut self, bit: ObjGrantsBits) -> &mut Self {
        self.set(bit, true)
    }

    /**
     * Disables the given `GrantsBit`.
     *
     * Disallow, for the interested user/group, the actions associated with
     * the grant
     */
    pub fn set_disabled(&mut self, bit: ObjGrantsBits) -> &mut Self {
        self.set(bit, false)
    }

    /**
     * Enables or disables the given `GrantsBit`.
     *
     * According to the `allow` value allows or disallow the actions
     * associated with the given `GrantsBit`
     */
    pub fn set(&mut self, _bit: ObjGrantsBits, _allow: bool) -> &mut Self {
        // self.m_bits.set_bit(bit.into(), allow);
        self
    }

    /**
     * Builds a new instance consuming the filled one given
     */
    pub fn build(self) -> Self {
        self
    }

    /**
     * Returns the raw permission bits
     */
    pub fn as_raw(&self) -> u32 {
        self.m_bits.raw_bits()
    }

    /**
     * Returns the raw permission bits as usize
     */
    pub fn as_raw_usize(&self) -> usize {
        self.as_raw() as usize
    }

    /**
     * Returns the bit value for the given `GrantsBits`
     */
    pub fn is(&self, _bit: ObjGrantsBits) -> bool {
        // self.m_bits.bit_at(bit.into())
        false
    }

    /**
     * Returns whether any of given `GrantsBit`s are active
     */
    pub fn is_any_of(&self, _bits: &[ObjGrantsBits]) -> bool {
        false
        // for bit in bits {
        //     if self.is(*bit) {
        //         return true;
        //     }
        // }
        // return false;
    }

    /**
     * Returns whether all of given `GrantsBit`s are active
     */
    pub fn is_all_of(&self, _bits: &[ObjGrantsBits]) -> bool {
        false
        // !self.is_any_of(bits)
    }
}

impl<T> Clone for Grants<T> where T: Object {
    fn clone(&self) -> Self {
        Self { m_bits: self.m_bits.clone(),
               _unused: Default::default() }
    }
}

impl<T> Copy for Grants<T> where T: Object {
    /* no methods to implement, just a marker */
}

impl<T> From<u32> for Grants<T> where T: Object {
    fn from(raw_bits: u32) -> Self {
        Self { m_bits: BitFlags::from_raw_truncate(raw_bits),
               _unused: Default::default() }
    }
}

impl Default for Grants<Dir> {
    /**
     * Returns the default `Grants` for a `Dir`
     */
    fn default() -> Self {
        // let grants = Self::new().m_bits
        //              | GrantsBit::UserCanOpenIt
        //              | GrantsBit::UserCanReadData
        //              | GrantsBit::UserCanWriteData;

        Self::new().set_enabled(ObjGrantsBits::UserCanOpenIt)
                   .set_enabled(ObjGrantsBits::UserCanReadData)
                   .set_enabled(ObjGrantsBits::UserCanWriteData)
                   .set_enabled(ObjGrantsBits::UserCanExecTraversData)
                   .set_enabled(ObjGrantsBits::UserCanReadInfo)
                   .set_enabled(ObjGrantsBits::UserCanWriteInfo)
                   .set_enabled(ObjGrantsBits::UserCanSeeIt)
            /* Owner's Group Grants */
                   .set_enabled(ObjGrantsBits::GroupCanOpenIt)
                   .set_enabled(ObjGrantsBits::GroupCanReadData)
                   .set_disabled(ObjGrantsBits::GroupCanWriteData)
                   .set_enabled(ObjGrantsBits::GroupCanExecTraversData)
                   .set_enabled(ObjGrantsBits::GroupCanReadInfo)
                   .set_enabled(ObjGrantsBits::GroupCanWriteInfo)
                   .set_enabled(ObjGrantsBits::GroupCanSeeIt)
            /* Other users/groups Grants */
                   .set_enabled(ObjGrantsBits::OtherCanOpenIt)
                   .set_enabled(ObjGrantsBits::OtherCanReadData)
                   .set_disabled(ObjGrantsBits::OtherCanWriteData)
                   .set_enabled(ObjGrantsBits::OtherCanExecTraversData)
                   .set_enabled(ObjGrantsBits::OtherCanReadInfo)
                   .set_enabled(ObjGrantsBits::OtherCanWriteInfo)
                   .set_enabled(ObjGrantsBits::OtherCanSeeIt)
                   .build()
    }
}

impl Default for Grants<File> {
    /**
     * Returns the default `Grants` for a `File`
     */
    fn default() -> Self {
        Self::new().set_enabled(ObjGrantsBits::UserCanOpenIt)
                   .set_enabled(ObjGrantsBits::UserCanReadData)
                   .set_enabled(ObjGrantsBits::UserCanWriteData)
                   .set_enabled(ObjGrantsBits::UserCanExecTraversData)
                   .set_enabled(ObjGrantsBits::UserCanReadInfo)
                   .set_enabled(ObjGrantsBits::UserCanWriteInfo)
                   .set_enabled(ObjGrantsBits::UserCanSeeIt)
            /* Owner's Group Grants */
                   .set_enabled(ObjGrantsBits::GroupCanOpenIt)
                   .set_enabled(ObjGrantsBits::GroupCanReadData)
                   .set_enabled(ObjGrantsBits::GroupCanWriteData)
                   .set_disabled(ObjGrantsBits::GroupCanExecTraversData)
                   .set_enabled(ObjGrantsBits::GroupCanReadInfo)
                   .set_disabled(ObjGrantsBits::GroupCanWriteInfo)
                   .set_enabled(ObjGrantsBits::GroupCanSeeIt)
            /* Other users/groups Grants */
                   .set_enabled(ObjGrantsBits::OtherCanOpenIt)
                   .set_enabled(ObjGrantsBits::OtherCanReadData)
                   .set_disabled(ObjGrantsBits::OtherCanWriteData)
                   .set_disabled(ObjGrantsBits::OtherCanExecTraversData)
                   .set_disabled(ObjGrantsBits::OtherCanReadInfo)
                   .set_disabled(ObjGrantsBits::OtherCanWriteInfo)
                   .set_enabled(ObjGrantsBits::OtherCanSeeIt)
                   .build()
    }
}

impl Default for Grants<IpcChan> {
    /**
     * Returns the default `Grants` for a `IpcChan`
     */
    fn default() -> Self {
        Self::new().set_enabled(ObjGrantsBits::UserCanOpenIt)
                   .set_enabled(ObjGrantsBits::UserCanReadData)
                   .set_enabled(ObjGrantsBits::UserCanWriteData)
                   .set_disabled(ObjGrantsBits::UserCanExecTraversData)
                   .set_enabled(ObjGrantsBits::UserCanReadInfo)
                   .set_disabled(ObjGrantsBits::UserCanWriteInfo)
                   .set_enabled(ObjGrantsBits::UserCanSeeIt)
            /* Owner's Group Grants */
                   .set_enabled(ObjGrantsBits::GroupCanOpenIt)
                   .set_enabled(ObjGrantsBits::GroupCanReadData)
                   .set_enabled(ObjGrantsBits::GroupCanWriteData)
                   .set_disabled(ObjGrantsBits::GroupCanExecTraversData)
                   .set_enabled(ObjGrantsBits::GroupCanReadInfo)
                   .set_disabled(ObjGrantsBits::GroupCanWriteInfo)
                   .set_enabled(ObjGrantsBits::GroupCanSeeIt)
            /* Other users/groups Grants */
                   .set_enabled(ObjGrantsBits::OtherCanOpenIt)
                   .set_enabled(ObjGrantsBits::OtherCanReadData)
                   .set_enabled(ObjGrantsBits::OtherCanWriteData)
                   .set_disabled(ObjGrantsBits::OtherCanExecTraversData)
                   .set_enabled(ObjGrantsBits::OtherCanReadInfo)
                   .set_disabled(ObjGrantsBits::OtherCanWriteInfo)
                   .set_enabled(ObjGrantsBits::OtherCanSeeIt)
                   .build()
    }
}

impl Default for Grants<Link> {
    /**
     * Returns the default `Grants` for a `Link`
     */
    fn default() -> Self {
        Self::new().set_enabled(ObjGrantsBits::UserCanOpenIt)
                   .set_enabled(ObjGrantsBits::UserCanReadData)
                   .set_enabled(ObjGrantsBits::UserCanWriteData)
                   .set_enabled(ObjGrantsBits::UserCanExecTraversData)
                   .set_enabled(ObjGrantsBits::UserCanReadInfo)
                   .set_enabled(ObjGrantsBits::UserCanWriteInfo)
                   .set_enabled(ObjGrantsBits::UserCanSeeIt)
            /* Owner's Group Grants */
                   .set_enabled(ObjGrantsBits::GroupCanOpenIt)
                   .set_enabled(ObjGrantsBits::GroupCanReadData)
                   .set_enabled(ObjGrantsBits::GroupCanWriteData)
                   .set_enabled(ObjGrantsBits::GroupCanExecTraversData)
                   .set_enabled(ObjGrantsBits::GroupCanReadInfo)
                   .set_disabled(ObjGrantsBits::GroupCanWriteInfo)
                   .set_enabled(ObjGrantsBits::GroupCanSeeIt)
            /* Other users/groups Grants */
                   .set_enabled(ObjGrantsBits::OtherCanOpenIt)
                   .set_enabled(ObjGrantsBits::OtherCanReadData)
                   .set_disabled(ObjGrantsBits::OtherCanWriteData)
                   .set_enabled(ObjGrantsBits::OtherCanExecTraversData)
                   .set_enabled(ObjGrantsBits::OtherCanReadInfo)
                   .set_disabled(ObjGrantsBits::OtherCanWriteInfo)
                   .set_enabled(ObjGrantsBits::OtherCanSeeIt)
                   .build()
    }
}

impl Default for Grants<MMap> {
    /**
     * Returns the default `Grants` for a `MMap`
     */
    fn default() -> Self {
        Self::new().set_enabled(ObjGrantsBits::UserCanOpenIt)
                   .set_enabled(ObjGrantsBits::UserCanReadData)
                   .set_enabled(ObjGrantsBits::UserCanWriteData)
                   .set_disabled(ObjGrantsBits::UserCanExecTraversData)
                   .set_enabled(ObjGrantsBits::UserCanReadInfo)
                   .set_enabled(ObjGrantsBits::UserCanWriteInfo)
                   .set_enabled(ObjGrantsBits::UserCanSeeIt)
            /* Owner's Group Grants */
                   .set_enabled(ObjGrantsBits::GroupCanOpenIt)
                   .set_enabled(ObjGrantsBits::GroupCanReadData)
                   .set_enabled(ObjGrantsBits::GroupCanWriteData)
                   .set_disabled(ObjGrantsBits::GroupCanExecTraversData)
                   .set_enabled(ObjGrantsBits::GroupCanReadInfo)
                   .set_enabled(ObjGrantsBits::GroupCanWriteInfo)
                   .set_enabled(ObjGrantsBits::GroupCanSeeIt)
            /* Other users/groups Grants */
                   .set_disabled(ObjGrantsBits::OtherCanOpenIt)
                   .set_enabled(ObjGrantsBits::OtherCanReadData)
                   .set_enabled(ObjGrantsBits::OtherCanWriteData)
                   .set_enabled(ObjGrantsBits::OtherCanExecTraversData)
                   .set_enabled(ObjGrantsBits::OtherCanReadInfo)
                   .set_disabled(ObjGrantsBits::OtherCanWriteInfo)
                   .set_enabled(ObjGrantsBits::OtherCanSeeIt)
                   .build()
    }
}

impl Default for Grants<OsRawMutex> {
    /**
     * Returns the default `Grants` for a `OsRawMutex`
     */
    fn default() -> Self {
        Self::new().set_enabled(ObjGrantsBits::UserCanOpenIt)
                   .set_enabled(ObjGrantsBits::UserCanReadData)
                   .set_enabled(ObjGrantsBits::UserCanWriteData)
                   .set_disabled(ObjGrantsBits::UserCanExecTraversData)
                   .set_enabled(ObjGrantsBits::UserCanReadInfo)
                   .set_enabled(ObjGrantsBits::UserCanWriteInfo)
                   .set_enabled(ObjGrantsBits::UserCanSeeIt)
            /* Owner's Group Grants */
                   .set_enabled(ObjGrantsBits::GroupCanOpenIt)
                   .set_enabled(ObjGrantsBits::GroupCanReadData)
                   .set_enabled(ObjGrantsBits::GroupCanWriteData)
                   .set_disabled(ObjGrantsBits::GroupCanExecTraversData)
                   .set_enabled(ObjGrantsBits::GroupCanReadInfo)
                   .set_disabled(ObjGrantsBits::GroupCanWriteInfo)
                   .set_enabled(ObjGrantsBits::GroupCanSeeIt)
            /* Other users/groups Grants */
                   .set_enabled(ObjGrantsBits::OtherCanOpenIt)
                   .set_enabled(ObjGrantsBits::OtherCanReadData)
                   .set_enabled(ObjGrantsBits::OtherCanWriteData)
                   .set_enabled(ObjGrantsBits::OtherCanExecTraversData)
                   .set_enabled(ObjGrantsBits::OtherCanReadInfo)
                   .set_disabled(ObjGrantsBits::OtherCanWriteInfo)
                   .set_enabled(ObjGrantsBits::OtherCanSeeIt)
                   .build()
    }
}

impl<T> Default for Grants<T> where T: Object {
    /**
     * Implemented to shut the warning of the compiler about overlapping
     * implementations of the `Default` trait
     */
    default fn default() -> Self {
        Self::new()
    }
}

#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum ObjGrantsBits {
    UserCanOpenIt,
    UserCanReadData,
    UserCanWriteData,
    UserCanExecTraversData,
    UserCanReadInfo,
    UserCanWriteInfo,
    UserCanSeeIt,

    GroupCanOpenIt,
    GroupCanReadData,
    GroupCanWriteData,
    GroupCanExecTraversData,
    GroupCanReadInfo,
    GroupCanWriteInfo,
    GroupCanSeeIt,

    OtherCanOpenIt,
    OtherCanReadData,
    OtherCanWriteData,
    OtherCanExecTraversData,
    OtherCanReadInfo,
    OtherCanWriteInfo,
    OtherCanSeeIt
}

impl BitFlagsValues for ObjGrantsBits {
}
