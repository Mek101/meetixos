/*! `OsEntity` configuration */

use core::marker::PhantomData;

use api_data::{
    entity::{
        config::{
            OsEntityConfigBits,
            RawOsEntityConfig
        },
        OsEntityId
    },
    sys::{
        codes::KernOsEntConfigFnId,
        fn_path::KernFnPath,
        AsSysCallPtr
    }
};

use crate::{
    config::{
        ConfigMode,
        CreatMode,
        OpenMode
    },
    entity::{
        OsEntity,
        OsEntityHandle
    },
    handle::{
        KernHandle,
        Result
    }
};

/**
 * High level type-safe `OsEntity` configuration
 */
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct OsEntityConfig<'a, T, M>
    where T: OsEntity,
          M: ConfigMode {
    m_raw_config: RawOsEntityConfig<'a>,
    _unused: PhantomData<(T, M)>
}

impl<'a, T> OsEntityConfig<'a, T, CreatMode> where T: OsEntity {
    /**
     * Constructs a `OsEntityConfig` for `OsEntity` creation
     */
    pub(super) fn new() -> Self {
        Self { m_raw_config: RawOsEntityConfig::new(T::TYPE, true),
               _unused: PhantomData }
    }

    /**
     * Dispatches the configuration to the kernel, which creates a new
     * `OsEntity`.
     */
    pub fn apply(&mut self, entity_name: &'a str) -> Result<T> {
        self.m_raw_config.set_name(entity_name);
        self.apply_config()
    }
}

impl<'a, T> OsEntityConfig<'a, T, OpenMode> where T: OsEntity {
    /**
     * Constructs a `OsEntityConfig` for `OsEntity` opening
     */
    pub(super) fn new() -> Self {
        Self { m_raw_config: RawOsEntityConfig::new(T::TYPE, false),
               _unused: PhantomData }
    }

    /**
     * Specifies the `OsEntity`'s name
     */
    pub fn with_name(&mut self, ent_name: &'a str) -> &mut Self {
        self.m_raw_config.set_name(ent_name);
        self
    }

    /**
     * Dispatches the configuration to the kernel, which tries to find the
     * requested `OsEntity`.
     *
     * For opening must be specified at least one of the id/name tuple
     */
    pub fn apply(&mut self) -> Result<T> {
        self.apply_config()
    }
}

impl<'a, T, M> OsEntityConfig<'a, T, M>
    where T: OsEntity,
          M: ConfigMode
{
    /**
     * Tells to the Kernel which unique identifier the `OsEntity` must
     * obtain in `CreatMode`.
     *
     * Or tells exactly which identifier the searched `OsEntity` have in
     * `FindMode`
     */
    pub fn with_id(&mut self, id: OsEntityId) -> &mut Self {
        self.m_raw_config.set_id(id);
        self
    }

    /**
     * Enables the admin filter
     */
    pub fn admin(&mut self) -> &mut Self {
        self.m_raw_config.flags_mut().set_enabled(OsEntityConfigBits::Admin);
        self
    }

    /**
     * Requests to the kernel to apply the given configuration
     */
    fn apply_config(&mut self) -> Result<T> {
        KernHandle::kern_call_1(KernFnPath::OsEntConfig(KernOsEntConfigFnId::ApplyConfig),
                                self.m_raw_config.as_syscall_ptr())
                   .map(|raw_entity_handle| T::from(OsEntityHandle::from_raw(raw_entity_handle)))
    }
}
