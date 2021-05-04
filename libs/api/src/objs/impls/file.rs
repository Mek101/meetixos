/*! # Open File Object
 *
 * Implements the open file object
 */

use core::ptr::NonNull;

use os::sysc::{
    codes::KernFileFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::obj::{
        ObjType,
        SeekMode,
        WithExecutableDataObject
    },
    caller::{
        KernCaller,
        Result
    },
    objs::{
        impls::{
            Dir,
            MMap
        },
        ObjId,
        Object,
        SizeableData,
        UserCreatable
    }
};

/** # Open File
 *
 * Represents a reference to an open file on the VFS.
 *
 * Exposes all the common operations that is expected to be present
 * for a file, like [`read()`], [`write()`], [`seek()`] and many
 * others.
 *
 * It is possible to map a `File` into a virtual memory region of
 * the caller process like the Unix's [`mmap()`] system call
 *
 * [`read()`]: crate::objs::impls::File::read
 * [`write()`]: crate::objs::impls::File::write
 * [`seek()`]: crate::objs::impls::File::set_pos
 * [`mmap()`]: https://man7.org/linux/man-pages/man2/mmap.2.html
 */
#[repr(transparent)]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct File {
    m_handle: ObjId
}

impl File {
    /** # Reads the next `buf.len()` data bytes
     *
     * Puts into the given `buf` at max `buf.len()` bytes read from the
     * current file's data cursor position and returns the number of read
     * bytes
     */
    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        self.kern_call_2(KernFnPath::File(KernFileFnId::ReadData),
                         buf.as_mut_ptr() as usize,
                         buf.len())
    }

    /** # Writes `buf.len()` bytes to the `File`'s data
     *
     * Puts into the File's data the content of the given `buf` and returns
     * the number of written bytes
     */
    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        self.kern_call_2(KernFnPath::File(KernFileFnId::WriteData),
                         buf.as_ptr() as usize,
                         buf.len())
    }

    /** # Copies this `File` to another `Dir`
     *
     * The use of this system call, instead of manual copy with
     * [`File::read()`] & [`File::write()`], involves many kernel
     * optimizations like a better usage of the VFS cache and, if
     * available, FS copy on write, which reduces time and space usages
     *
     * [`File::read()`]: crate::objs::impls::File::read
     * [`File::write()`]: crate::objs::impls::File::write
     */
    pub fn copy_to(&self, dest: &Dir) -> Result<Self> {
        self.kern_call_1(KernFnPath::File(KernFileFnId::Copy),
                         dest.obj_handle().as_raw_usize())
            .map(|copy_id| Self::from(ObjId::from(copy_id)))
    }

    /** # Moves this `File` to another `Dir`
     *
     * The use of this system call, instead of manual move with
     * [`File::read()`], [`File::write()`] and [`File::drop_name()`],
     * involves kernel optimizations, because it simply changes the
     * parent directory node of this file.
     *
     * [`File::read()`]: crate::objs::impls::File::read
     * [`File::write()`]: crate::objs::impls::File::write
     * [`File::drop_name()`]: crate::objs::Object::drop_name
     */
    pub fn move_to(&self, dest: &Dir) -> Result<()> {
        self.kern_call_1(KernFnPath::File(KernFileFnId::Move),
                         dest.obj_handle().as_raw_usize())
            .map(|_| ())
    }

    /** # Creates a `File` backed `MMap`
     *
     * The [`MMap`]'s data will be filled by the mapped file's content
     * `from` the given offset to `from + size`.
     *
     * If `mem` is not [`None`] the kernel tries to put the [`MMap`]'s data
     * at the given address, if not available the system call fails.
     *
     * To obtain the maximum portability leave this parameter [`None`], it
     * is used only for special cases (like the dynamic loader)
     *
     * If `sync` is `true` and the file was opened with
     * [`ObjConfig::for_write()`], writes to the [`MMap`]'s content changes
     * the `File`'s content; otherwise, if the `File` was not opened for
     * write the `sync` is simply ignored
     *
     * Note that write to a `File` with a [`MMap`] doesn't grow the file's
     * data
     *
     * TODO if two or more processes map in read/exec the same portion of a
     *      same file share the physical pages
     *
     * TODO if two or more processes map in rw && sync the same file portion
     *      share in kernel mode an `RwLock`
     *
     * TODO what if the underling File changes because of File.write()
     *      calls from other threads?
     *      It would be interesting to directly map the physical pages of
     *      the kernel's PageCache layer, but what if the mapped portion is
     *      not page aligned like the offsets of the kernel's PageCache?
     *      When mapping ELFs executable this should not be a problem
     *      because ELF offsets are always page aligned (but into the file
     *      content too?)
     *
     * [`MMap`]: crate::objs::impls::MMap
     * [`None`]: core::option::Option::None
     * [`Err`]: core::result::Result::Err
     * [`ObjConfig::for_write()`]: crate::objs::ObjConfig::for_write
     */
    pub fn map_to_memory(&self,
                         addr: Option<NonNull<u8>>,
                         from: u64,
                         size: u64,
                         sync: bool)
                         -> Result<MMap> {
        self.kern_call_4(KernFnPath::File(KernFileFnId::MapToMem),
                         addr.map(|nn_ptr| nn_ptr.as_ptr() as usize).unwrap_or(0),
                         from as usize,
                         size as usize,
                         sync as usize)
            .map(|obj_id| MMap::from(ObjId::from(obj_id)))
    }

    /** # Sets the read/write position
     *
     * According to the [`SeekMode`] given, it updates the read/write
     * position.
     *
     * [`SeekMode`]: crate::bits::obj::modes::SeekMode
     */
    pub fn set_pos(&self, mode: SeekMode) -> Result<u64> {
        self.kern_call_2(KernFnPath::File(KernFileFnId::SetPos),
                         mode.mode(),
                         mode.off().unwrap_or(0))
            .map(|off| off as u64)
    }

    /** Returns the current cursor position
     */
    pub fn current_pos(&self) -> u64 {
        self.set_pos(SeekMode::Absolute(0)).unwrap_or(0)
    }
}

impl Object for File {
    /** The value of the [`ObjType`] that matches the implementation
     *
     * [`ObjType`]: crate::bits::obj::types::ObjType
     */
    const TYPE: ObjType = ObjType::File;

    /** Returns the immutable reference to the underling [`ObjId`] instance
     *
     * [`ObjId`]: crate::objs::ObjId
     */
    fn obj_handle(&self) -> &ObjId {
        &self.m_handle
    }

    /** Returns the mutable reference to the underling [`ObjId`] instance
     *
     * [`ObjId`]: crate::objs::ObjId
     */
    fn obj_handle_mut(&mut self) -> &mut ObjId {
        &mut self.m_handle
    }
}

impl From<ObjId> for File {
    /** Performs the conversion
     */
    fn from(id: ObjId) -> Self {
        Self { m_handle: id }
    }
}

impl KernCaller for File {
    /** Returns the upper 32bits of the 64bit identifier of a system call
     */
    fn caller_handle_bits(&self) -> u32 {
        self.obj_handle().caller_handle_bits()
    }
}

impl WithExecutableDataObject for File {
    /* No methods to implement */
}

impl SizeableData for File {
    /* No methods to implement */
}

impl UserCreatable for File {
    /* No methods to implement */
}
