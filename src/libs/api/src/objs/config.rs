/*! # `Object` builder
 *
 * Implements the standard and unique way to open or create [`Object`]s
 *
 * [`Object`]: crate::objs::object::Object
 */

use core::marker::PhantomData;

use bit_field::BitField;

use os::{
    str_utils,
    sysc::{
        codes::KernObjConfigFnId,
        fn_path::KernFnPath
    }
};

use crate::{
    bits::obj::{
        Grants,
        ObjType
    },
    caller::{
        KernCaller,
        Result
    },
    config::{
        ConfigMode,
        CreatMode,
        FindMode
    },
    objs::{
        ObjId,
        Object,
        UserCreatable
    },
    path::Path
};

/** # Object Configuration
 *
 * Implements a functional standard interface to open or create [`Object`]
 * based objects.
 *
 * This object takes the place of the old style Unix's [`open()`] system
 * call providing a function-call-chain interface where each called method
 * enables a feature.
 *
 * i.e [`ObjConfig::for_read()`] enables data read operations (if
 * the caller have the permissions to do that) exactly as [`O_RDONLY`] for
 * Unix's [`open()`] do.
 *
 * The constructed configuration can be then applied with:
 * * [`ObjConfig::apply_for()`] - to open or create a named object (an
 *   object with a name into the VFS)
 * * [`ObjConfig::apply_for_anon()`](AN) - to create anonymous object that
 *   are local to the scope of the object and not represented into the VFS
 *
 * [`Object`]: crate::objs::object::Object
 * [`open()`]: https://man7.org/linux/man-pages/man2/open.2.html
 * [`ObjConfig::for_read()`]: crate::objs::config::ObjConfig::for_read
 * [`O_RDONLY`]: https://man7.org/linux/man-pages/man2/open.2.html
 * [`ObjConfig::apply_for()`]: crate::objs::config::ObjConfig::apply_for
 * [AN]: rate::objs::config::ObjConfig::apply_for_anon
 */
#[derive(Debug)]
pub struct ObjConfig<T, M>
    where T: Object,
          M: ConfigMode {
    m_flags: u8,
    m_size: usize,
    m_grants: Grants<T>,
    m_type: ObjType,
    m_path: Option<Path>,
    _unused: PhantomData<T>,
    _unused2: PhantomData<M>
}

impl<T> ObjConfig<T, CreatMode> where T: Object + UserCreatable {
    /** # Constructs a new `ObjConfig`
     *
     * The instance is initialized for creation
     */
    pub(crate) fn new() -> Self {
        Self { m_flags: 0.set_bit(Self::CFG_CREAT_BIT, true).clone(),
               m_size: 0,
               m_grants: Grants::default(),
               m_type: T::TYPE,
               m_path: None,
               _unused: Default::default(),
               _unused2: Default::default() }
    }

    /** # Customizes the `Object`'s `Grants`
     *
     * Sets custom [`Grants`] for the creation of the new object.
     *
     * The caller [`OSUser`] (or at least one of his joined [`OSGroup`]s)
     * must have write grants for the parent directory
     *
     * [`Grants`]: crate::bits::obj::grants::Grants
     * [`OSUser`]: crate::ents::impls::user::OSUser
     * [`OSGroup`]: crate::ents::impls::group::OSGroup
     */
    pub fn with_grants(&mut self, grants: Grants<T>) -> &mut Self {
        self.m_grants = grants;
        self
    }

    /** # Creates an anonymous object
     *
     * Dispatches the configuration to the kernel that creates a new
     * anonymous object.
     *
     * An anonymous object is an object that [`have no name`] but can be
     * shared among other tasks with [`Object::send()`].
     *
     * The life of the objects created with this method is the scope that
     * contains the handle, when the object goes out of scope (from all the
     * tasks that owns it) it is definitely destroyed
     *
     * [`have no name`]: crate::objs::infos::info::ObjInfo::is_named
     * [`Object::send()`]: crate::objs::object::Object::send
     * [`ObjConfig::exclusive()`]: crate::objs::config::ObjConfig::exclusive
     */
    pub fn apply_for_anon(&self) -> Result<T> {
        self.apply_builder_config()
    }
}

impl<T> ObjConfig<T, FindMode> where T: Object {
    /** # Constructs a new `ObjConfig`
     *
     * The returned instance is blank and zeroed
     */
    pub(crate) fn new() -> Self {
        Self { m_flags: 0,
               m_size: 0,
               m_grants: Grants::default(),
               m_type: T::TYPE,
               m_path: None,
               _unused: Default::default(),
               _unused2: Default::default() }
    }

    /** # Enables exclusive open
     *
     * Fails to open the object if it is already open by someone else.
     *
     * The other tasks that tries to open the same object after a successful
     * exclusive open by someone will fail
     */
    pub fn exclusive(&mut self) -> &mut Self {
        self.m_flags.set_bit(Self::CFG_EXCLUSIVE_BIT, true);
        self
    }
}

impl<T, M> ObjConfig<T, M>
    where T: Object + SizeableData,
          M: ConfigMode
{
    /** # Gives a size to the object's data
     *
     * Allows to give a size to the object's data, both if it already exists
     * or it must be created.
     *
     * Like the exec bit this configuration is meaningless for certain type
     * of objects (i.e [`Dir`]ectories, [`OsRawMutex`]s, [`Link`]s),
     * optional for others (i.e [`File`]s, [`IpcChan`]nels) but
     * mandatory for [`MMap`]s **when created**
     *
     * [`Dir`]: crate::objs::impls::dir::Dir
     * [`OsRawMutex`]: crate::objs::impls::mutex::OsRawMutex
     * [`Link`]: crate::objs::impls::link::Link
     * [`File`]: crate::objs::impls::file::File
     * [`IpcChan`]: crate::objs::impls::ipc_chan::IpcChan
     * [`MMap`]: crate::objs::impls::mmap::MMap
     */
    pub fn with_size(&mut self, size: usize) -> &mut Self {
        self.m_flags.set_bit(Self::CFG_SET_SIZE_BIT, true);
        self.m_size = size;
        self
    }
}

impl<T, M> ObjConfig<T, M>
    where T: Object,
          M: ConfigMode
{
    const CFG_CREAT_BIT: usize = 0;
    const CFG_READ_BIT: usize = 1;
    const CFG_WRITE_BIT: usize = 2;
    const CFG_EXEC_BIT: usize = 3;
    const CFG_SET_SIZE_BIT: usize = 4;
    const CFG_EXCLUSIVE_BIT: usize = 5;

    /** # Enables data read operations
     *
     * Data read operation can be performed (if the caller have the
     * permissions to do that)
     */
    pub fn for_read(&mut self) -> &mut Self {
        self.m_flags.set_bit(Self::CFG_READ_BIT, true);
        self
    }

    /** # Enables data write operations
     *
     * Data write operations can be performed (if the caller have the
     * permissions to do that)
     */
    pub fn for_write(&mut self) -> &mut Self {
        self.m_flags.set_bit(Self::CFG_WRITE_BIT, true);
        self
    }

    /** # Enables data executable operations
     *
     * As written [here] the exec bit have different meaning among the
     * different [`Object`] implementations.
     *
     * Only for the [`File`]s and [`MMap`]s enable this configuration bit
     * changes the behaviours (i.e [`File`]s can be run as executable via
     * [`TaskConfig<Proc>::run()`] and [`MMap`]'s pages are mapped without
     * [`NO_EXECUTE`] bit).
     *
     * Calling this method for the other object types only tell to the
     * kernel to ensure the caller user have the data execution
     * permissions for the object to open
     *
     * [here]: crate::bits::obj::grants::Grants::set_data_executable
     * [`Object`]: crate::objs::object::Object
     * [`File`]: crate::objs::impls::file::File
     * [`MMap`]: crate::objs::impls::mmap::MMap
     * [`TaskConfig<Proc>::run()`]: crate::tasks::config::TaskConfig::run-1
     * [`NO_EXECUTE`]: shared::mem::paging::table::PTFlags::NO_EXECUTE
     */
    pub fn for_exec(&mut self) -> &mut Self {
        self.m_flags.set_bit(Self::CFG_EXEC_BIT, true);
        self
    }

    /** # Opens/Creates a named object
     *
     * Dispatches the configuration to the kernel that opens (or creates if
     * [`ObjConfig::creat()`] was called) the object referenced by `path`.
     *
     * The life of the objects created with this method varies by type:
     * Permanent objects, like [`File`]s, [`Dir`]ectories, [`Link`]s and
     * [`OsRawMutex`]es, persists until they are explicitly destroyed with
     * [`Object::drop_name()`].
     *
     * The other kind of objects, like [`MMap`]s and [`IpcChan`]nels, live
     * until there is a reference to them. When the references reaches the 0
     * they are definitely destroyed
     *
     * [`ObjConfig::creat()`]: crate::objs::config::ObjConfig::creat
     * [`File`]: crate::objs::impls::file::File
     * [`Dir`]: crate::objs::impls::dir::Dir
     * [`Link`]: crate::objs::impls::link::Link
     * [`OsRawMutex`]: crate::objs::impls::mutex::OsRawMutex
     * [`Object::drop_name()`]: crate::objs::object::Object::drop_name
     * [`MMap`]: crate::objs::impls::mmap::MMap
     * [`IpcChan`]: crate::objs::impls::ipc_chan::IpcChan
     */
    pub fn apply_for<P>(&mut self, path: P) -> Result<T>
        where P: AsRef<[u8]> {
        self.m_path = Some(Path::from(str_utils::u8_slice_to_str_slice(path.as_ref())));
        self.apply_builder_config()
    }

    /** # Dispatches the configuration to the kernel
     *
     * Requests to the kernel to apply the given configuration
     */
    fn apply_builder_config(&self) -> Result<T> {
        self.kern_call_1(KernFnPath::ObjConfig(KernObjConfigFnId::ApplyConfig),
                         &self as *const _ as usize)
            .map(|obj_id| T::from(ObjId::from(obj_id)))
    }
}

#[cfg(feature = "enable_kernel_methods")]
impl<T, M> ObjConfig<T, M>
    where T: Object,
          M: ConfigMode
{
    /** Returns whether [`ObjConfig::for_read()`] was called
     *
     * [`ObjConfig::for_read()`]: crate::objs::config::ObjConfig::for_read
     */
    pub fn is_read(&self) -> bool {
        self.m_flags.get_bit(Self::CFG_READ_BIT)
    }

    /** Returns whether [`ObjConfig::for_write()`] was called
     *
     * [`ObjConfig::for_write()`]: crate::objs::config::ObjConfig::for_write
     */
    pub fn is_write(&self) -> bool {
        self.m_flags.get_bit(Self::CFG_WRITE_BIT)
    }

    /** Returns whether [`ObjConfig::for_exec()`] was called
     *
     * [`ObjConfig::for_exec()`]: crate::objs::config::ObjConfig::for_exec
     */
    pub fn is_exec(&self) -> bool {
        self.m_flags.get_bit(Self::CFG_EXEC_BIT)
    }

    /** Returns whether [`ObjConfig::exclusive()`] was called
     *
     * [`ObjConfig::exclusive()`]: crate::objs::config::ObjConfig::exclusive
     */
    pub fn is_exclusive(&self) -> bool {
        self.m_flags.get_bit(Self::CFG_EXCLUSIVE_BIT)
    }

    /** Returns whether [`ObjConfig::with_size()`] was called
     *
     * [`ObjConfig::with_size()`]: crate::objs::config::ObjConfig::with_size
     */
    pub fn is_sized(&self) -> bool {
        self.m_flags.get_bit(Self::CFG_SET_SIZE_BIT)
    }

    /** Returns whether [`ObjConfig::creat()`] was called
     *
     * [`ObjConfig::creat()`]: crate::objs::config::ObjConfig::creat
     */
    pub fn is_creat(&self) -> bool {
        self.m_flags.get_bit(Self::CFG_CREAT_BIT)
    }

    /** Returns the [`Grants`] given with [`ObjConfig::creat()`]
     *
     * [`Grants`]: crate::bits::obj::grants::Grants
     * [`ObjConfig::creat()`]: crate::objs::config::ObjConfig::creat
     */
    pub fn grants(&self) -> Grants<T> {
        self.m_grants
    }

    /** Returns the size in bytes given to [`ObjConfig::with_size()`]
     *
     * [`ObjConfig::with_size()`]: crate::objs::config::ObjConfig::with_size
     */
    pub fn size(&self) -> usize {
        self.m_size
    }

    /** Returns the [`ObjType`] given via generics
     *
     * [`ObjType`]: crate::bits::obj::types::ObjType
     */
    pub fn obj_type(&self) -> ObjType {
        self.m_type
    }

    /** Returns the [`Path`] given to [`ObjConfig::apply_for()`]
     *
     * [`Path`]: crate::path::Path
     * [`ObjConfig::apply_for()`]: crate::objs::config::ObjConfig::apply_for
     */
    pub fn path(&self) -> Option<&Path> {
        self.m_path.as_ref()
    }
}

impl<T, M> KernCaller for ObjConfig<T, M>
    where T: Object,
          M: ConfigMode
{
    /* Nothing to implement */
}

/** # Sizeable Data Marker
 *
 * Marker trait implemented for the objects that have meaning with concept
 * of resizable data, like [`File`], [`MMap`] and [`IpcChan`]
 *
 * [`File`]: crate::objs::impls::file::File
 * [`MMap`]: crate::objs::impls::mmap::MMap
 * [`IpcChan`]: crate::objs::impls::ipc_chan::IpcChan
 */
pub trait SizeableData {
    /* No methods, just a marker trait */
}
