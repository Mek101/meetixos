/*! Open Link `Object` */

use api_data::{
    obj::types::ObjType,
    sys::{
        codes::KernLinkFnId,
        fn_path::KernFnPath
    }
};

use crate::{
    handle::Result,
    obj::{
        ObjHandle,
        Object,
        UserCreatableObject
    }
};

#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct Link {
    m_obj_handle: ObjHandle
}

impl Link {
    pub fn deref_link(&self) -> Result<ObjHandle> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::Link(KernLinkFnId::Deref))
            .map(|bind_raw_handle| ObjHandle::from_raw(bind_raw_handle))
    }

    pub fn bind_to<T>(&self, obj_to_bind: &T) -> Result<()>
        where T: Object {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_1(KernFnPath::Link(KernLinkFnId::BindTo),
                              obj_to_bind.obj_handle().kern_handle().raw_handle()
                              as usize)
            .map(|_| ())
    }
}

impl From<ObjHandle> for Link {
    fn from(obj_handle: ObjHandle) -> Self {
        Self { m_obj_handle: obj_handle }
    }
}

impl Object for Link {
    const TYPE: ObjType = ObjType::Link;

    fn obj_handle(&self) -> &ObjHandle {
        &self.m_obj_handle
    }

    fn obj_handle_mut(&mut self) -> &mut ObjHandle {
        &mut self.m_obj_handle
    }
}

impl UserCreatableObject for Link {
    /* No methods to implement */
}
