/*! `Object` information related data structures */

use api_data::obj::info::RawObjInfo;

use crate::obj::{
    ObjHandle,
    Object
};

#[derive(Debug)]
#[derive(Default)]
pub struct ObjInfo<T>
    where T: Object {
    m_raw_info: RawObjInfo,
    m_obj_handle: ObjHandle
}

impl<T> ObjInfo<T> where T: Object {
    pub(crate) fn new(raw_obj_info: RawObjInfo, obj_handle: ObjHandle) -> Self {
        Self { m_raw_info: raw_obj_info,
               m_obj_handle: obj_handle }
    }
}
