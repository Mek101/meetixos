/*! `Object` configuration */

use core::marker::PhantomData;

use api_data::{
    object::config::{
        ObjConfigBits,
        RawObjConfig
    },
    sys::{
        codes::KernObjConfigFnId,
        fn_path::KernFnPath,
        TAsSysCallPtr
    }
};

use crate::{
    config_mode::{
        CreatMode,
        OpenMode,
        TConfigMode
    },
    kern_handle::{
        KernHandle,
        Result
    },
    object::{
        grants::ObjGrants,
        MTAnonymousObject,
        MTExecutableDataObject,
        MTSizeableDataObject,
        ObjHandle,
        TObject,
        TUserCreatableObject
    },
    path::Path
};

/**
 * High level type-safe `Object` configuration
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct ObjConfig<'a, T, M>
    where T: TObject,
          M: TConfigMode {
    m_raw_config: RawObjConfig<'a>,
    _unused: PhantomData<(T, M)>
}

impl<'a, T> ObjConfig<'a, T, CreatMode>
    where T: TObject + TUserCreatableObject /* Constructors */
{
    /**
     * Constructs a `ObjConfig` for `Object` creation
     */
    pub(super) fn new() -> Self {
        Self { m_raw_config: RawObjConfig::new(T::TYPE, true),
               _unused: PhantomData }
    }
}

impl<'a, T> ObjConfig<'a, T, OpenMode> where T: TObject /* Constructors */ {
    /**
     * Constructs an empty `ObjConfig` for `Object` opening
     */
    pub(super) fn new() -> Self {
        Self { m_raw_config: RawObjConfig::new(T::TYPE, false),
               _unused: PhantomData }
    }
}

impl<'a, T, M> ObjConfig<'a, T, M>
    where T: TObject,
          M: TConfigMode /* Methods */
{
    /**
     * Dispatches the configuration to the kernel that opens (or creates if
     * `Object::creat()` was called) the `Object` referenced by `path`.
     *
     * The lifetime of the `Object` created with this method varies by type:
     *
     * # Permanent `Object`s
     * * `File`s
     * * `Dir`s
     * * `Link`s
     * They must be destroyed explicitly with `Object::drop_name()` and can
     * survive reboots if they are stored into permanent filesystems
     *
     * # Volatile `Object`s
     * * `MMap`s
     * * `IpcChan`s
     * * `OsRawMutex`s
     * When all the references to them are dropped they are destroyed
     *
     * `Device`s are special cases, because they are volatile `Object`s, but
     * can be destroyed only by the kernel at system shutdown
     */
    pub fn apply_for(&mut self, path: &'a Path) -> Result<T> {
        self.m_raw_config.set_path(path.as_raw_components());
        self.apply_builder_config()
    }
}

impl<'a, T> ObjConfig<'a, T, CreatMode>
    where T: TObject + TUserCreatableObject + MTAnonymousObject /* Methods */
{
    /**
     * Dispatches the configuration to the kernel which creates a new
     * anonymous `Object`.
     *
     * An anonymous `Object` is an object that have no name, so it cannot be
     * explicitly open by other `Task`s, but can be shared among other with
     * `Object::send()`.
     *
     * The lifetime of the `Object`s created with this method is the scope
     * which contains the handle, when the `Object` goes out of scope
     * (from all the tasks that owns it) it is definitely destroyed
     */
    pub fn apply_for_anon(&self) -> Result<T> {
        self.apply_builder_config()
    }
}

impl<'a, T> ObjConfig<'a, T, CreatMode>
    where T: TObject + TUserCreatableObject /* Setters */
{
    /**
     * Sets custom `ObjGrants` for the creation of the new `Object`
     */
    pub fn with_grants(&mut self, grants: ObjGrants<T>) -> &mut Self {
        *self.m_raw_config.grants_mut() = *grants;
        self
    }
}

impl<'a, T> ObjConfig<'a, T, OpenMode> where T: TObject /* Setters */ {
    /**
     * Ensures that the `Object` can be opened only by one `Task` a time
     */
    pub fn exclusive(&mut self) -> &mut Self {
        self.m_raw_config.flags_mut().set_enabled(ObjConfigBits::Exclusive);
        self
    }
}

impl<'a, T, M> ObjConfig<'a, T, M>
    where T: TObject + MTSizeableDataObject,
          M: TConfigMode /* Setters */
{
    /**
     * Truncates the data size to the specified amount
     */
    pub fn with_data_size(&mut self, data_size: usize) -> &mut Self {
        self.m_raw_config.set_data_size(data_size);
        self
    }
}

impl<'a, T, M> ObjConfig<'a, T, M>
    where T: TObject + MTExecutableDataObject,
          M: TConfigMode /* Setters */
{
    /**
    * Enables data executable operations

    */
    pub fn for_exec(&mut self) -> &mut Self {
        self.m_raw_config.flags_mut().set_enabled(ObjConfigBits::Exec);
        self
    }
}

impl<'a, T, M> ObjConfig<'a, T, M>
    where T: TObject,
          M: TConfigMode /* Setters */
{
    /**
     * Enables data read operations
     */
    pub fn for_read(&mut self) -> &mut Self {
        self.m_raw_config.flags_mut().set_enabled(ObjConfigBits::Read);
        self
    }

    /**
     * Enables data write operations
     */
    pub fn for_write(&mut self) -> &mut Self {
        self.m_raw_config.flags_mut().set_enabled(ObjConfigBits::Write);
        self
    }
}

impl<'a, T, M> ObjConfig<'a, T, M>
    where T: TObject,
          M: TConfigMode /* Privates */
{
    /**
     * Requests to the kernel to apply the given configuration
     */
    fn apply_builder_config(&self) -> Result<T> {
        KernHandle::kern_call_1(KernFnPath::ObjConfig(KernObjConfigFnId::ApplyConfig),
                                self.m_raw_config.as_syscall_ptr())
                   .map(|raw_obj_handle| T::from(ObjHandle::from_raw(raw_obj_handle)))
    }
}
