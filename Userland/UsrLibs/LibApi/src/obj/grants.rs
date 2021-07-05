/*! `Object`'s grants management */

use core::{
    marker::PhantomData,
    ops::{
        Deref,
        DerefMut
    }
};

use api_data::obj::grants::{
    ObjGrantsBits,
    RawObjGrants
};

use crate::obj::{
    impls::{
        dir::Dir,
        file::File,
        ipc_chan::IpcChan,
        link::Link,
        mmap::MMap,
        mutex::OsRawMutex
    },
    Object
};

/**
 * High level type-safe `RawObjGrants` wrapper
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct ObjGrants<T>
    where T: Object {
    m_raw_grants: RawObjGrants,
    _unused: PhantomData<T>
}

impl<T> ObjGrants<T> where T: Object {
    /**
     * Constructs a zeroed `ObjGrants`
     */
    pub fn new() -> Self {
        Self { m_raw_grants: RawObjGrants::new_zero(),
               _unused: PhantomData }
    }
}

impl<T> From<RawObjGrants> for ObjGrants<T> where T: Object {
    fn from(raw_grants: RawObjGrants) -> Self {
        Self { m_raw_grants: raw_grants,
               _unused: PhantomData }
    }
}

impl<T> Deref for ObjGrants<T> where T: Object {
    type Target = RawObjGrants;

    fn deref(&self) -> &Self::Target {
        &self.m_raw_grants
    }
}

impl<T> DerefMut for ObjGrants<T> where T: Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.m_raw_grants
    }
}

impl Default for ObjGrants<Dir> {
    /**
     * Returns the default `ObjGrants` for a `Dir`
     */
    fn default() -> Self {
        let mut obj_grants = Self::new();
        obj_grants.set_enabled(ObjGrantsBits::UserCanOpenIt)
            .set_enabled(ObjGrantsBits::UserCanReadData)
            .set_enabled(ObjGrantsBits::UserCanWriteData)
            .set_enabled(ObjGrantsBits::UserCanExecTraversData)
            .set_enabled(ObjGrantsBits::UserCanReadInfo)
            .set_enabled(ObjGrantsBits::UserCanWriteInfo)
            .set_enabled(ObjGrantsBits::UserCanSeeIt)
            /* Owner's Group ObjGrants */
            .set_enabled(ObjGrantsBits::GroupCanOpenIt)
            .set_enabled(ObjGrantsBits::GroupCanReadData)
            .set_disabled(ObjGrantsBits::GroupCanWriteData)
            .set_enabled(ObjGrantsBits::GroupCanExecTraversData)
            .set_enabled(ObjGrantsBits::GroupCanReadInfo)
            .set_enabled(ObjGrantsBits::GroupCanWriteInfo)
            .set_enabled(ObjGrantsBits::GroupCanSeeIt)
            /* Other users/groups ObjGrants */
            .set_enabled(ObjGrantsBits::OtherCanOpenIt)
            .set_enabled(ObjGrantsBits::OtherCanReadData)
            .set_disabled(ObjGrantsBits::OtherCanWriteData)
            .set_enabled(ObjGrantsBits::OtherCanExecTraversData)
            .set_enabled(ObjGrantsBits::OtherCanReadInfo)
            .set_enabled(ObjGrantsBits::OtherCanWriteInfo)
            .set_enabled(ObjGrantsBits::OtherCanSeeIt);

        obj_grants
    }
}

impl Default for ObjGrants<File> {
    /**
     * Returns the default `ObjGrants` for a `File`
     */
    fn default() -> Self {
        let mut obj_grants = Self::new();
        obj_grants.set_enabled(ObjGrantsBits::UserCanOpenIt)
                  .set_enabled(ObjGrantsBits::UserCanReadData)
                  .set_enabled(ObjGrantsBits::UserCanWriteData)
                  .set_enabled(ObjGrantsBits::UserCanExecTraversData)
                  .set_enabled(ObjGrantsBits::UserCanReadInfo)
                  .set_enabled(ObjGrantsBits::UserCanWriteInfo)
                  .set_enabled(ObjGrantsBits::UserCanSeeIt)
                  .set_enabled(ObjGrantsBits::GroupCanOpenIt)
                  .set_enabled(ObjGrantsBits::GroupCanReadData)
                  .set_enabled(ObjGrantsBits::GroupCanWriteData)
                  .set_disabled(ObjGrantsBits::GroupCanExecTraversData)
                  .set_enabled(ObjGrantsBits::GroupCanReadInfo)
                  .set_disabled(ObjGrantsBits::GroupCanWriteInfo)
                  .set_enabled(ObjGrantsBits::GroupCanSeeIt)
                  .set_enabled(ObjGrantsBits::OtherCanOpenIt)
                  .set_enabled(ObjGrantsBits::OtherCanReadData)
                  .set_disabled(ObjGrantsBits::OtherCanWriteData)
                  .set_disabled(ObjGrantsBits::OtherCanExecTraversData)
                  .set_disabled(ObjGrantsBits::OtherCanReadInfo)
                  .set_disabled(ObjGrantsBits::OtherCanWriteInfo)
                  .set_enabled(ObjGrantsBits::OtherCanSeeIt);

        obj_grants
    }
}

impl Default for ObjGrants<IpcChan> {
    /**
     * Returns the default `ObjGrants` for a `IpcChan`
     */
    fn default() -> Self {
        let mut obj_grants = Self::new();
        obj_grants.set_enabled(ObjGrantsBits::UserCanOpenIt)
                  .set_enabled(ObjGrantsBits::UserCanReadData)
                  .set_enabled(ObjGrantsBits::UserCanWriteData)
                  .set_disabled(ObjGrantsBits::UserCanExecTraversData)
                  .set_enabled(ObjGrantsBits::UserCanReadInfo)
                  .set_disabled(ObjGrantsBits::UserCanWriteInfo)
                  .set_enabled(ObjGrantsBits::UserCanSeeIt)
                  .set_enabled(ObjGrantsBits::GroupCanOpenIt)
                  .set_enabled(ObjGrantsBits::GroupCanReadData)
                  .set_enabled(ObjGrantsBits::GroupCanWriteData)
                  .set_disabled(ObjGrantsBits::GroupCanExecTraversData)
                  .set_enabled(ObjGrantsBits::GroupCanReadInfo)
                  .set_disabled(ObjGrantsBits::GroupCanWriteInfo)
                  .set_enabled(ObjGrantsBits::GroupCanSeeIt)
                  .set_enabled(ObjGrantsBits::OtherCanOpenIt)
                  .set_enabled(ObjGrantsBits::OtherCanReadData)
                  .set_enabled(ObjGrantsBits::OtherCanWriteData)
                  .set_disabled(ObjGrantsBits::OtherCanExecTraversData)
                  .set_enabled(ObjGrantsBits::OtherCanReadInfo)
                  .set_disabled(ObjGrantsBits::OtherCanWriteInfo)
                  .set_enabled(ObjGrantsBits::OtherCanSeeIt);

        obj_grants
    }
}

impl Default for ObjGrants<Link> {
    /**
     * Returns the default `ObjGrants` for a `Link`
     */
    fn default() -> Self {
        let mut obj_grants = Self::new();
        obj_grants.set_enabled(ObjGrantsBits::UserCanOpenIt)
                  .set_enabled(ObjGrantsBits::UserCanReadData)
                  .set_enabled(ObjGrantsBits::UserCanWriteData)
                  .set_enabled(ObjGrantsBits::UserCanExecTraversData)
                  .set_enabled(ObjGrantsBits::UserCanReadInfo)
                  .set_enabled(ObjGrantsBits::UserCanWriteInfo)
                  .set_enabled(ObjGrantsBits::UserCanSeeIt)
                  .set_enabled(ObjGrantsBits::GroupCanOpenIt)
                  .set_enabled(ObjGrantsBits::GroupCanReadData)
                  .set_enabled(ObjGrantsBits::GroupCanWriteData)
                  .set_enabled(ObjGrantsBits::GroupCanExecTraversData)
                  .set_enabled(ObjGrantsBits::GroupCanReadInfo)
                  .set_disabled(ObjGrantsBits::GroupCanWriteInfo)
                  .set_enabled(ObjGrantsBits::GroupCanSeeIt)
                  .set_enabled(ObjGrantsBits::OtherCanOpenIt)
                  .set_enabled(ObjGrantsBits::OtherCanReadData)
                  .set_disabled(ObjGrantsBits::OtherCanWriteData)
                  .set_enabled(ObjGrantsBits::OtherCanExecTraversData)
                  .set_enabled(ObjGrantsBits::OtherCanReadInfo)
                  .set_disabled(ObjGrantsBits::OtherCanWriteInfo)
                  .set_enabled(ObjGrantsBits::OtherCanSeeIt);

        obj_grants
    }
}

impl Default for ObjGrants<MMap> {
    /**
     * Returns the default `ObjGrants` for a `MMap`
     */
    fn default() -> Self {
        let mut obj_grants = Self::new();
        obj_grants.set_enabled(ObjGrantsBits::UserCanOpenIt)
                  .set_enabled(ObjGrantsBits::UserCanReadData)
                  .set_enabled(ObjGrantsBits::UserCanWriteData)
                  .set_disabled(ObjGrantsBits::UserCanExecTraversData)
                  .set_enabled(ObjGrantsBits::UserCanReadInfo)
                  .set_enabled(ObjGrantsBits::UserCanWriteInfo)
                  .set_enabled(ObjGrantsBits::UserCanSeeIt)
                  .set_enabled(ObjGrantsBits::GroupCanOpenIt)
                  .set_enabled(ObjGrantsBits::GroupCanReadData)
                  .set_enabled(ObjGrantsBits::GroupCanWriteData)
                  .set_disabled(ObjGrantsBits::GroupCanExecTraversData)
                  .set_enabled(ObjGrantsBits::GroupCanReadInfo)
                  .set_enabled(ObjGrantsBits::GroupCanWriteInfo)
                  .set_enabled(ObjGrantsBits::GroupCanSeeIt)
                  .set_disabled(ObjGrantsBits::OtherCanOpenIt)
                  .set_enabled(ObjGrantsBits::OtherCanReadData)
                  .set_enabled(ObjGrantsBits::OtherCanWriteData)
                  .set_enabled(ObjGrantsBits::OtherCanExecTraversData)
                  .set_enabled(ObjGrantsBits::OtherCanReadInfo)
                  .set_disabled(ObjGrantsBits::OtherCanWriteInfo)
                  .set_enabled(ObjGrantsBits::OtherCanSeeIt);

        obj_grants
    }
}

impl Default for ObjGrants<OsRawMutex> {
    /**
     * Returns the default `ObjGrants` for a `OsRawMutex`
     */
    fn default() -> Self {
        let mut obj_grants = Self::new();
        obj_grants.set_enabled(ObjGrantsBits::UserCanOpenIt)
                  .set_enabled(ObjGrantsBits::UserCanReadData)
                  .set_enabled(ObjGrantsBits::UserCanWriteData)
                  .set_disabled(ObjGrantsBits::UserCanExecTraversData)
                  .set_enabled(ObjGrantsBits::UserCanReadInfo)
                  .set_enabled(ObjGrantsBits::UserCanWriteInfo)
                  .set_enabled(ObjGrantsBits::UserCanSeeIt)
                  .set_enabled(ObjGrantsBits::GroupCanOpenIt)
                  .set_enabled(ObjGrantsBits::GroupCanReadData)
                  .set_enabled(ObjGrantsBits::GroupCanWriteData)
                  .set_disabled(ObjGrantsBits::GroupCanExecTraversData)
                  .set_enabled(ObjGrantsBits::GroupCanReadInfo)
                  .set_disabled(ObjGrantsBits::GroupCanWriteInfo)
                  .set_enabled(ObjGrantsBits::GroupCanSeeIt)
                  .set_enabled(ObjGrantsBits::OtherCanOpenIt)
                  .set_enabled(ObjGrantsBits::OtherCanReadData)
                  .set_enabled(ObjGrantsBits::OtherCanWriteData)
                  .set_enabled(ObjGrantsBits::OtherCanExecTraversData)
                  .set_enabled(ObjGrantsBits::OtherCanReadInfo)
                  .set_disabled(ObjGrantsBits::OtherCanWriteInfo)
                  .set_enabled(ObjGrantsBits::OtherCanSeeIt);

        obj_grants
    }
}

impl<T> Default for ObjGrants<T> where T: Object {
    /**
     * Implemented to shut the warning of the compiler about overlapping
     * implementations of the `Default` trait
     */
    default fn default() -> Self {
        Self::new()
    }
}
