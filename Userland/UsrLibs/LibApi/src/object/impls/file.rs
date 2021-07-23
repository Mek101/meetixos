/*! Open File `Object` */

use core::ptr::NonNull;

use api_data::{
    object::{
        modes::SeekMode,
        types::ObjType
    },
    sys::{
        codes::KernFileFnId,
        fn_path::KernFnPath,
        TAsSysCallPtr
    }
};

use crate::{
    kern_handle::Result,
    object::{
        impls::{
            dir::Dir,
            mmap::MMap
        },
        MTAnonymousObject,
        MTExecutableDataObject,
        MTSizeableDataObject,
        ObjHandle,
        TObject,
        TUserCreatableObject
    }
};

/**
 * Open file reference.
 *
 * Represent the higher abstraction over `Device`s and filesystems
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

impl File /* Methods */ {
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
            .map(move |read_bytes| &buf[..read_bytes])
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
     * Maps part of this `File`'s content into a `MMap` with the same
     * permissions.
     *
     * `from_off` & `mmap_size` must be 4KiB aligned.
     *
     * It is possible to keep the modification to the `MMap`'s memory synced
     * with the on-disk `File` content, providing `keep_file_sync = true`.
     *
     * When `keep_file_sync` is `true` the kernel directly maps the
     * page-cache pages, which means that concurrent accesses to the same
     * location with `File::write()` modifies the `MMap` memory too
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
                              &map_addr as *const _ as usize,
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

impl TObject for File {
    const TYPE: ObjType = ObjType::File;

    fn obj_handle(&self) -> &ObjHandle {
        &self.m_obj_handle
    }

    fn obj_handle_mut(&mut self) -> &mut ObjHandle {
        &mut self.m_obj_handle
    }
}

impl TUserCreatableObject for File {
    /* No methods to implement */
}

impl MTSizeableDataObject for File {
    /* No methods to implement */
}

impl MTExecutableDataObject for File {
    /* No methods to implement */
}

impl MTAnonymousObject for File {
    /* No methods to implement */
}
