/*! Open Link `Object` */

use api_data::{
    object::types::ObjType,
    sys::{
        codes::KernLinkFnId,
        fn_path::KernFnPath
    }
};

use crate::{
    kern_handle::Result,
    object::{
        ObjHandle,
        TObject,
        TUserCreatableObject
    }
};

/**
 * Reference to another `Object`.
 *
 * When the link is traversed in a path is automatically dereferenced to the
 * bind `Object`.
 *
 * It's also possible to open a `Link` using the type of the bind `Object`
 * (i.e a `Link` with points to a `File` can be directly opened with
 * `File::open()`), the kernel automatically tries to dereference the `Link`
 * before returning
 */
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

impl Link /* Methods */ {
    /**
     * Dereferences the link and returns the `ObjHandle` of the bind
     * `Object`.
     *
     * The returned `ObjHandle` can be easily wrapped inside the his real
     * `Object` type with `into()` or `try_into()`
     */
    pub fn deref_link(&self) -> Result<ObjHandle> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::Link(KernLinkFnId::Deref))
            .map(|bind_raw_handle| ObjHandle::from_raw(bind_raw_handle))
    }

    /**
     * Binds a new named `Object` overwriting the previous bind.
     *
     * Bind of anonymous `Object`s causes the return with an error
     */
    pub fn bind_to<T>(&self, obj_to_bind: &T) -> Result<()>
        where T: TObject {
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

impl TObject for Link {
    const TYPE: ObjType = ObjType::Link;

    fn obj_handle(&self) -> &ObjHandle {
        &self.m_obj_handle
    }

    fn obj_handle_mut(&mut self) -> &mut ObjHandle {
        &mut self.m_obj_handle
    }
}

impl TUserCreatableObject for Link {
    /* No methods to implement */
}
