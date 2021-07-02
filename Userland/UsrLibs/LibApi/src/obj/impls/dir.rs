/*! Open Directory `Object` */

use api_data::{
    obj::{
        dir::DirEntry,
        modes::SeekMode,
        types::ObjType
    },
    sys::{
        codes::KernDirFnId,
        fn_path::KernFnPath,
        AsSysCallPtr
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

/**
 * Collection of named `Object`s.
 *
 * Consists essentially in a list `Object` of names and identifiers
 */
#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct Dir {
    m_obj_handle: ObjHandle
}

impl Dir {
    /**
     * Returns the next available `DirEntry` of this `Dir`
     */
    pub fn next_child(&self) -> Result<DirEntry> {
        let mut dir_entry = DirEntry::default();

        self.obj_handle()
            .kern_handle()
            .inst_kern_call_1(KernFnPath::Dir(KernDirFnId::NextChild),
                              dir_entry.as_syscall_ptr())
            .map(|_| dir_entry)
    }

    /**
     * Returns the current absolute position for reading
     */
    pub fn pos(&self) -> Result<usize> {
        self.set_pos(SeekMode::Relative(0))
    }

    /**
     * Sets the reading position cursor according to the given `SeekMode`
     */
    pub fn set_pos(&self, seek_mode: SeekMode) -> Result<usize> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_1(KernFnPath::Dir(KernDirFnId::SetPos),
                              seek_mode.as_syscall_ptr())
    }
}

impl Iterator for Dir {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_child().ok()
    }
}

impl From<ObjHandle> for Dir {
    fn from(obj_handle: ObjHandle) -> Self {
        Self { m_obj_handle: obj_handle }
    }
}

impl Object for Dir {
    const TYPE: ObjType = ObjType::Dir;

    fn obj_handle(&self) -> &ObjHandle {
        &self.m_obj_handle
    }

    fn obj_handle_mut(&mut self) -> &mut ObjHandle {
        &mut self.m_obj_handle
    }
}

impl UserCreatableObject for Dir {
    /* No methods to implement */
}
