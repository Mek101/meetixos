/*! `Object` configuration */

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
        grants::Grants,
        types::ObjType
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
    objs::object::{
        ObjId,
        Object,
        UserCreatable
    },
    path::Path
};

/**
 * Common functional configuration interface to open or create `Object`
 * based objects
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
    /**
     * Constructs an empty `ObjConfig` for creation
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

    /**
     * Sets custom `Grants` for the creation of the new object.
     *
     * The caller `OSUser` (or at least one of his joined `OSGroup`s)
     * must have write grants for the parent directory
     */
    pub fn with_grants(&mut self, grants: Grants<T>) -> &mut Self {
        self.m_grants = grants;
        self
    }

    /**
     * Dispatches the configuration to the kernel that creates a new
     * anonymous object.
     *
     * An anonymous object is an object that have no name but can be
     * shared among other tasks with `Object::send()`.
     *
     * The life of the objects created with this method is the scope that
     * contains the handle, when the object goes out of scope (from all the
     * tasks that owns it) it is definitely destroyed
     */
    pub fn apply_for_anon(&self) -> Result<T> {
        self.apply_builder_config()
    }
}

impl<T> ObjConfig<T, FindMode> where T: Object {
    /**
     * Constructs an empty `ObjConfig` for opening
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

    /**
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
    /**
     * Give a size to the object's data, both if it already exists or it
     * must be created.
     *
     * Like the exec bit this configuration is meaningless for certain type
     * of objects (i.e `Dir`ectories, `OsRawMutex`s, `Link`s),
     * optional for others (i.e `File`s, `IpcChan`nels) but
     * mandatory for `MMap`s **when created**
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

    /**
     * Enables data read operations
     *
     * Data read operation can be performed (if the caller have the
     * permissions to do that)
     */
    pub fn for_read(&mut self) -> &mut Self {
        self.m_flags.set_bit(Self::CFG_READ_BIT, true);
        self
    }

    /**
     * Enables data write operations
     *
     * Data write operations can be performed (if the caller have the
     * permissions to do that)
     */
    pub fn for_write(&mut self) -> &mut Self {
        self.m_flags.set_bit(Self::CFG_WRITE_BIT, true);
        self
    }

    /**
     * Enables data executable operations
     *
     * The exec bit have different meaning among the different `Object`
     * implementations.
     *
     * Only for the `File`s and `MMap`s enable this configuration bit
     * changes the behaviours (i.e `File`s can be run as executable via
     * `TaskConfig<Proc>::run()` and `MMap`'s pages are mapped without
     * `PTFlags::NO_EXECUTE` bit).
     *
     * Calling this method for the other object types only tell to the
     * kernel to ensure the caller user have the data execution
     * permissions for the object to open
     */
    pub fn for_exec(&mut self) -> &mut Self {
        self.m_flags.set_bit(Self::CFG_EXEC_BIT, true);
        self
    }

    /**
     * Dispatches the configuration to the kernel that opens (or creates if
     * `Object::creat()` was called) the object referenced by `path`.
     *
     * The life of the objects created with this method varies by type:
     * Permanent objects, like `File`s, `Dir`ectories, `Link`s and
     * `OsRawMutex`es, persists until they are explicitly destroyed with
     * `Object::drop_name()`.
     *
     * The other kind of objects, like `MMap`s and `IpcChan`nels, live
     * until there is a reference to them. When the references reaches the 0
     * they are definitely destroyed
     */
    pub fn apply_for<P>(&mut self, path: P) -> Result<T>
        where P: AsRef<[u8]> {
        self.m_path = Some(Path::from(str_utils::u8_slice_to_str_slice(path.as_ref())));
        self.apply_builder_config()
    }

    /**
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
    /**
     * Returns whether `ObjConfig::for_read()` was called
     */
    pub fn is_read(&self) -> bool {
        self.m_flags.get_bit(Self::CFG_READ_BIT)
    }

    /**
     * Returns whether `ObjConfig::for_write()` was called
     */
    pub fn is_write(&self) -> bool {
        self.m_flags.get_bit(Self::CFG_WRITE_BIT)
    }

    /**
     * Returns whether `ObjConfig::for_exec()` was called
     */
    pub fn is_exec(&self) -> bool {
        self.m_flags.get_bit(Self::CFG_EXEC_BIT)
    }

    /**
     * Returns whether `ObjConfig::exclusive()` was called
     */
    pub fn is_exclusive(&self) -> bool {
        self.m_flags.get_bit(Self::CFG_EXCLUSIVE_BIT)
    }

    /**
     * Returns whether `ObjConfig::with_size()` was called
     */
    pub fn is_sized(&self) -> bool {
        self.m_flags.get_bit(Self::CFG_SET_SIZE_BIT)
    }

    /**
     * Returns whether `ObjConfig::creat()` was called
     */
    pub fn is_creat(&self) -> bool {
        self.m_flags.get_bit(Self::CFG_CREAT_BIT)
    }

    /**
     * Returns the `Grants` given with `ObjConfig::creat()`
     */
    pub fn grants(&self) -> Grants<T> {
        self.m_grants
    }

    /**
     * Returns the size in bytes given to `ObjConfig::with_size()`
     */
    pub fn size(&self) -> usize {
        self.m_size
    }

    /**
     * Returns the `ObjType` given via generics
     */
    pub fn obj_type(&self) -> ObjType {
        self.m_type
    }

    /**
     * Returns the `Path` given to `ObjConfig::apply_for()`
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

/**
 * Marker trait implemented for the objects that have meaning with concept
 * of resizable data, like `File`, `MMap` and `IpcChan`
 */
pub trait SizeableData {
    /* No methods, just a marker trait */
}
