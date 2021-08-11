/*! `Object`'s grants management */

use core::convert::TryFrom;

use bits::bit_flags::{
    BitFlags,
    TBitFlagsValues
};

/**
 * `Object` permission flags
 */
pub type RawObjGrants = BitFlags<usize, ObjGrantsBits>;

/**
 * Lists the valid `ObjGrants` bits.
 *
 * # `[User/Group/Other]CanOpenIt`
 * The caller can effectively open the `Object` reference (which can be
 * opened without read/write/exec features, for example if only information
 * are requested)
 *
 * # `[User/Group/Other]CanReadData`
 * The caller can execute data-read related system calls (obviously the read
 * feature must be enabled in the originating configuration)
 *
 * # `[User/Group/Other]CanWriteData`
 * The caller can execute data-write related system calls (obviously the
 * write feature must be enabled in the originating configuration)
 *
 * # `[User/Group/Other]CanExecTraversData`
 * The caller can execute/traverse the data of the `Object`. Traverse the
 * data have meaning with objects which data is reference to other objects
 * (i.e directories and links)
 *
 * # `[User/Group/Other]CanReadInfo`
 * The caller can read the metadata information of the `Object`
 *
 * # `[User/Group/Other]CanWriteInfo`
 * The caller can update the metadata information of the `Object`
 *
 * # `[User/Group/Other]CanSeeIt`
 * The `Object` is listed into the parent directory iteration for the caller
 * `OsUser`
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
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

impl Into<usize> for ObjGrantsBits {
    fn into(self) -> usize {
        self as usize
    }
}

impl TryFrom<usize> for ObjGrantsBits {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UserCanOpenIt),
            1 => Ok(Self::UserCanReadData),
            2 => Ok(Self::UserCanWriteData),
            3 => Ok(Self::UserCanExecTraversData),
            4 => Ok(Self::UserCanReadInfo),
            5 => Ok(Self::UserCanWriteInfo),
            6 => Ok(Self::UserCanSeeIt),
            7 => Ok(Self::GroupCanOpenIt),
            8 => Ok(Self::GroupCanReadData),
            9 => Ok(Self::GroupCanWriteData),
            10 => Ok(Self::GroupCanExecTraversData),
            11 => Ok(Self::GroupCanReadInfo),
            12 => Ok(Self::GroupCanWriteInfo),
            13 => Ok(Self::GroupCanSeeIt),
            14 => Ok(Self::OtherCanOpenIt),
            15 => Ok(Self::OtherCanReadData),
            16 => Ok(Self::OtherCanWriteData),
            17 => Ok(Self::OtherCanExecTraversData),
            18 => Ok(Self::OtherCanReadInfo),
            19 => Ok(Self::OtherCanWriteInfo),
            20 => Ok(Self::OtherCanSeeIt),
            _ => Err(())
        }
    }
}

impl TBitFlagsValues for ObjGrantsBits {
}
