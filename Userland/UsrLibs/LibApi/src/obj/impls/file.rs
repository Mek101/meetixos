/*! Open File `Object` */

use core::ptr::NonNull;

use api_data::{
    obj::{
        modes::SeekMode,
        types::ObjType
    },
    sys::{
        codes::KernFileFnId,
        fn_path::KernFnPath
    }
};

use crate::{
    handle::Result,
    obj::{
        impls::{
            dir::Dir,
            mmap::MMap
        },
        AnonymousObject,
        ExecutableDataObject,
        ObjHandle,
        Object,
        SizeableDataObject,
        UserCreatableObject
    }
};

/**
 * File reference
 */
#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct File {
    m_obj_handle: ObjHandle
}

impl File {
    /**
     * Puts into `buf` at max `buf.len()` bytes reading from the current
     * cursor position.
     *
     * Returns the reference to the sub-slice filled by the kernel
     */
    pub fn read<'a>(&self, buf: &'a mut [u8]) -> Result<&'a [u8]> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_2(KernFnPath::File(KernFileFnId::ReadData),
                              buf.as_mut_ptr() as usize,
                              buf.len())
            .map(|read_bytes| &buf[..read_bytes])
    }

    /**
     * Writes into this `File` the `buf` content for at max `buf.len()`
     * bytes from the current cursor position.
     *
     * Returns the reference to the sub-slice which wasn't possible to write
     */
    pub fn write<'a>(&self, buf: &'a [u8]) -> Result<&'a [u8]> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_2(KernFnPath::File(KernFileFnId::WriteData),
                              buf.as_ptr() as usize,
                              buf.len())
            .map(|written_bytes| &buf[written_bytes..])
    }

    /**
     * Copies this `File` into another `Dir`.
     *
     * The use of this system-call is encouraged when copying big amount of
     * data, since the kernel optimizes this operation in background and,
     * when available, uses filesystem copy-on-write
     */
    pub fn copy_to(&self, dest_dir: &Dir) -> Result<Self> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_1(KernFnPath::File(KernFileFnId::Copy),
                              dest_dir.obj_handle().kern_handle().raw_handle() as usize)
            .map(|file_copy_raw_handle| {
                Self::from(ObjHandle::from_raw(file_copy_raw_handle))
            })
    }

    /**
     * Moves this `File` to another `Dir`.
     *
     * The use of this system-call is encouraged because the kernel
     * optimizes the operation when possible
     */
    pub fn move_to(&self, dest_dir: &Dir) -> Result<()> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_1(KernFnPath::File(KernFileFnId::Move),
                              dest_dir.obj_handle().kern_handle().raw_handle() as usize)
            .map(|_| ())
    }

    /**
     * Maps part of this `File`'s content into a `MMap`.
     *
     * It is possible to keep the modification to the `MMap`'s memory synced
     * with the on-disk `File` content, providing `keep_file_sync = true`
     */
    pub fn map_to_memory(&self,
                         map_addr: Option<NonNull<()>>,
                         from_off: usize,
                         mmap_size: usize,
                         keep_file_sync: bool)
                         -> Result<MMap> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_4(KernFnPath::File(KernFileFnId::Move),
                              map_addr.map(|nn_ptr| nn_ptr.as_ptr() as usize)
                                      .unwrap_or_default(),
                              from_off,
                              mmap_size,
                              keep_file_sync as usize)
            .map(|mmap_raw_handle| MMap::from(ObjHandle::from_raw(mmap_raw_handle)))
    }

    /**
     * According to the `SeekMode` given, it updates the read/write
     * position
     */
    pub fn set_pos(&self, mode: SeekMode) -> Result<usize> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_1(KernFnPath::File(KernFileFnId::SetPos),
                              mode.as_syscall_ptr())
    }

    /**
     * Returns the current cursor position
     */
    pub fn pos(&self) -> Result<usize> {
        self.set_pos(SeekMode::Absolute(0))
    }
}

impl From<ObjHandle> for File {
    fn from(obj_handle: ObjHandle) -> Self {
        Self { m_obj_handle: obj_handle }
    }
}

impl Object for File {
    const TYPE: ObjType = ObjType::File;

    fn obj_handle(&self) -> &ObjHandle {
        &self.m_obj_handle
    }

    fn obj_handle_mut(&mut self) -> &mut ObjHandle {
        &mut self.m_obj_handle
    }
}

impl UserCreatableObject for File {
    /* No methods to implement */
}

impl SizeableDataObject for File {
    /* No methods to implement */
}

impl ExecutableDataObject for File {
    /* No methods to implement */
}

impl AnonymousObject for File {
    /* No methods to implement */
}
