/*! `OSEntity` configuration */

use core::marker::PhantomData;

use bits::fields::BitFields;
use helps::str::copy_str_to_u8_buf;
use os::{
    limits::OS_ENTITY_NAME_LEN_MAX,
    sysc::{
        codes::KernOsEntConfigFnId,
        fn_path::KernFnPath
    }
};

use crate::{
    bits::ent::types::OsEntityType,
    caller::{
        KernCaller,
        Result
    },
    config::{
        ConfigFinderIter,
        ConfigMode,
        CreatMode,
        FindMode
    },
    ents::entity::{
        OsEntity,
        OsEntityId
    }
};

/**
 * Common functional configuration interface to find or create `OSEntity`
 * based objects
 */
#[derive(Debug)]
pub struct OsEntConfig<T, M>
    where T: OsEntity,
          M: ConfigMode {
    m_flags: u8,
    m_id: Option<u16>,
    m_name: Option<[u8; OS_ENTITY_NAME_LEN_MAX]>,
    m_type: OsEntityType,
    _unused: PhantomData<T>,
    _unused2: PhantomData<M>
}

impl<T> OsEntConfig<T, CreatMode> where T: OsEntity {
    /**
     * Constructs an empty `OSEntConfig` for creation
     */
    pub(crate) fn new() -> Self {
        Self { m_flags: 0.set_bit(Self::CFG_CREAT_BIT, true).clone(),
               m_id: None,
               m_name: None,
               m_type: OsEntityType::Unknown,
               _unused: Default::default(),
               _unused2: Default::default() }
    }

    /**
     * Makes the resultant `OSEntity` an administrative account
     */
    pub fn make_admin(&mut self) -> &mut Self {
        self.m_flags.set_bit(Self::CFG_ADMIN_BIT, true);
        self
    }

    /**
     * Dispatches the configuration to the Kernel and creates a new
     * `OSEntity` with the given name, which couldn't be unique (but
     * the tuple `id, name` must be).
     *
     * If the Kernel finds another `OSEntity` with the same name it
     * ensures that the allocated id is not the same of the existing
     * one
     */
    pub fn apply(mut self, name: &str) -> Result<T> {
        let mut buf = [0; OS_ENTITY_NAME_LEN_MAX];
        copy_str_to_u8_buf(&mut buf, name);
        self.m_name = Some(buf);

        self.kern_call_1(KernFnPath::OsEntConfig(KernOsEntConfigFnId::CreateEntity),
                         &self as *const _ as usize)
            .map(|ent_id| T::from(OsEntityId::from(ent_id)))
    }
}

impl<T> OsEntConfig<T, FindMode> where T: OsEntity {
    /**
     * Constructs an empty `OSEntConfig` for finding
     */
    pub(crate) fn new() -> Self {
        Self { m_flags: 0,
               m_id: None,
               m_name: None,
               m_type: OsEntityType::Unknown,
               _unused: Default::default(),
               _unused2: Default::default() }
    }

    /**
     * Enables the name filter to tell the Kernel which name must be
     * selected
     */
    pub fn with_name(&mut self, name: &str) -> &mut Self {
        let mut buf = [0; OS_ENTITY_NAME_LEN_MAX];
        copy_str_to_u8_buf(&mut buf, name);

        self.m_name = Some(buf);
        self
    }

    /**
     * Enables the "only administrative" account filter
     */
    pub fn only_admin(&mut self) -> &mut Self {
        self.m_flags.set_bit(Self::CFG_ADMIN_BIT, true);
        self
    }

    /**
     * Dispatches the configuration to the Kernel to validate and initialize
     * an iteration pool on which the returned `Iterator` will fetch
     * the results.
     *
     * If the given configuration have no filters, the Kernel initializes an
     * iteration pool with **ALL** the active entities of the `T` type
     * (`OSUser` or `OSGroup`)
     */
    pub fn search(self) -> Result<impl Iterator<Item = T>> {
        self.kern_call_1(KernFnPath::OsEntConfig(KernOsEntConfigFnId::InitFind),
                         &self as *const _ as usize)
            .map(|iter_id| ConfigFinderIter::from(iter_id))
    }
}

impl<T, M> OsEntConfig<T, M>
    where T: OsEntity,
          M: ConfigMode
{
    const CFG_CREAT_BIT: usize = 0;
    const CFG_ADMIN_BIT: usize = 1;

    /**
     * Tells to the Kernel which unique identifier the `OSEntity` must
     * obtain in `CreatMode` (the entire operation will fail if the id is
     * already assigned).
     *
     * Or tells exactly which identifier the searched OSEntity have in
     * `FindMode`
     */
    pub fn with_id(&mut self, id: u16) -> &mut Self {
        self.m_id = Some(id);
        self
    }
}

#[cfg(feature = "enable_kernel_methods")]
impl<T: OsEntity, M: ConfigMode> OsEntConfig<T, M> {
    /**
     * Returns whether this configuration represents a creation request
     */
    pub fn is_creat(&self) -> bool {
        self.m_flags.bit_at(Self::CFG_CREAT_BIT)
    }

    /**
     * Returns whether the admin filter/flag is enabled
     */
    pub fn is_admin(&self) -> bool {
        self.m_flags.bit_at(Self::CFG_ADMIN_BIT)
    }

    /**
     * Returns the optional identifier given
     */
    pub fn id(&self) -> Option<u16> {
        self.m_id
    }

    /**
     * Returns the optional name given
     */
    pub fn name(&self) -> Option<&[u8; OS_ENTITY_NAME_LEN_MAX]> {
        self.m_name.as_ref()
    }

    /**
     * Returns the `OSEntityType`
     */
    pub fn ent_type(&self) -> OsEntityType {
        self.m_type
    }
}

impl<T, M> KernCaller for OsEntConfig<T, M>
    where T: OsEntity,
          M: ConfigMode
{
    /* Nothing to implement */
}
