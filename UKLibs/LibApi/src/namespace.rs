use crate::{
    bits::obj::grants::Grants,
    caller::{
        KernCaller,
        Result
    },
    errors::error::Error,
    objs::{
        impls::dir::Dir,
        object::ObjId
    }
};

pub struct NameSpace(ObjId);

impl NameSpace {
    pub fn mount(&self,
                 // _fs_type: FilesystemType,
                 _mnt_point: Dir,
                 _grants: Grants<Dir>)
                 -> Result<Dir> {
        Err(Error::default())
    }

    pub fn unmount(&self, _mnt_point: Dir) -> Result<()> {
        Err(Error::default())
    }
}

impl From<ObjId> for NameSpace {
    fn from(obj_id: ObjId) -> Self {
        Self(obj_id)
    }
}

impl KernCaller for NameSpace {
    /* No methods to implement */
}

// c_handy_enum! {
//     pub enum FilesystemType : u16 {
//         FatX = 0,
//         Iso9660 = 1,
//         MxFs = 2,
//     }
// }
