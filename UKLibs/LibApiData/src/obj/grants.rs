/*! `Object`'s grants management */

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use bits::flags::{
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
 * * `XxxCanOpenIt` - The caller can effectively open the obj reference
 *   (which can be opened without read/write/exec features, for example if
 *   only information are requested)
 * * `XxxCanReadData` - The caller can call data-read related system calls
 *   (obviously the read feature must be enabled in the originating
 *   configuration)
 * * `XxxCanWriteData` - The caller can call data-write related system calls
 *   (obviously the write feature must be enabled in the originating
 *   configuration)
 * * `XxxCanExecTraversData` - The caller can execute/traverse the data of
 *   the `Object`. Traverse the data have meaning with objects which data is
 *   reference to other objects (i.e directories and links)
 * * `XxxCanReadInfo` - The caller can read the metadata information of the
 *   `Object`
 * * `XxxCanWriteInfo` - The caller can update the metadata information of
 *   the `Object`
 * * `XxxCanSeeIt` - The is listed into the parent directory iteration for
 *   the caller `OsUser`
 */
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
