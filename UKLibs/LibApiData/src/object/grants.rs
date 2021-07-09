/*! `Object`'s grants management */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use bits::bit_flags::{
    BitFlags,
    BitFlagsValues
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
