/*! `Object`'s grants management */

use core::{
    marker::PhantomData,
    ops::{
        Deref,
        DerefMut
    }
};

use api_data::obj::grants::RawObjGrants;

use crate::obj::Object;

/**
 * High level type-safe `RawObjGrants` wrapper
 */
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
