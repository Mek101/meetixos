/*! # `Entity` Configuration
 *
 * Implements the standard and unique way to find existing [`OSEntity`]s or
 * create new one
 *
 * [`OSEntity`]: crate::ents::entity::OSEntity
 */

use core::marker::PhantomData;

use bit_field::BitField;

use os::{
    limits::ENTITY_NAME_LEN_MAX,
    str_utils,
    sysc::{
        codes::KernOSEntConfigFnId,
        fn_path::KernFnPath
    }
};

use crate::{
    bits::ent::OSEntityType,
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
    ents::{
        OSEntity,
        OSEntityId
    }
};

/** # `OSEntity` Configuration
 *
 * Implements a function standard interface to find existing [`OSEntity`] or
 * create new one.
 *
 * [`OSEntity`]: crate::ents::entity::OSEntity
 */
#[derive(Debug)]
pub struct OSEntConfig<T, M>
    where T: OSEntity,
          M: ConfigMode {
    m_flags: u8,
    m_id: Option<u16>,
    m_name: Option<[u8; ENTITY_NAME_LEN_MAX]>,
    m_type: OSEntityType,
    _unused: PhantomData<T>,
    _unused2: PhantomData<M>
}

impl<T> OSEntConfig<T, CreatMode> where T: OSEntity {
    /** # Constructs a new `OSEntConfig`
     *
     * The instance is initialized for creation
     */
    pub(crate) fn new() -> Self {
        Self { m_flags: 0.set_bit(Self::CFG_CREAT_BIT, true).clone(),
               m_id: None,
               m_name: None,
               m_type: OSEntityType::Unknown,
               _unused: Default::default(),
               _unused2: Default::default() }
    }

    /** # Enables admin flag
     *
     * Makes the resultant [`OSEntity`] an administrative account
     *
     * [`OSEntity`]: crate::ents::entity::OSEntity
     */
    pub fn make_admin(&mut self) -> &mut Self {
        self.m_flags.set_bit(Self::CFG_ADMIN_BIT, true);
        self
    }

    /** # Creates a new `OSEntity`
     *
     * Dispatches the configuration to the kernel and creates a new
     * [`OSEntity`] with the given name, that is not necessarily unique (but
     * the couple id and name must).
     *
     * If the kernel finds another [`OSEntity`] with the same name it
     * ensures that the allocated id is not the same of the existing
     * one.
     *
     * [`OSEntity`]: crate::ents::entity::OSEntity
     */
    pub fn apply(mut self, name: &str) -> Result<T> {
        let mut buf = [0; ENTITY_NAME_LEN_MAX];
        str_utils::copy_str_to_u8_buf(&mut buf, name);
        self.m_name = Some(buf);

        self.kern_call_1(KernFnPath::OSEntConfig(KernOSEntConfigFnId::CreateEntity),
                         &self as *const _ as usize)
            .map(|ent_id| T::from(OSEntityId::from(ent_id)))
    }
}

impl<T> OSEntConfig<T, FindMode> where T: OSEntity {
    /** # Constructs a new `OSEntConfig`
     *
     * The returned instance is blank and zeroed
     */
    pub(crate) fn new() -> Self {
        Self { m_flags: 0,
               m_id: None,
               m_name: None,
               m_type: OSEntityType::Unknown,
               _unused: Default::default(),
               _unused2: Default::default() }
    }

    /** # Specifies the name
     *
     * Enables the name filter to tell the kernel which name must be
     * selected
     */
    pub fn with_name(&mut self, name: &str) -> &mut Self {
        let mut buf = [0; ENTITY_NAME_LEN_MAX];
        str_utils::copy_str_to_u8_buf(&mut buf, name);

        self.m_name = Some(buf);
        self
    }

    /** # Specifies only admin `OSEntities`
     *
     * Enables the "only administrative" account filter
     */
    pub fn only_admin(&mut self) -> &mut Self {
        self.m_flags.set_bit(Self::CFG_ADMIN_BIT, true);
        self
    }

    /** # Searches for existing `OSEntities`
     *
     * Dispatches the configuration to the kernel to validate and initialize
     * an iteration pool on which the returned [`Iterator`] will fetch
     * the results.
     *
     * If the given configuration have no filters, the kernel initializes an
     * iteration pool with **ALL** the active entities of the `T` type
     * ([`OSUser`] or [`OSGroup`])
     *
     * [`Iterator`]: core::iter::Iterator
     * [`OSUser`]: crate::ents::impls::user::OSUser
     * [`OSGroup`]: crate::ents::impls::group::OSGroup
     */
    pub fn search(self) -> Result<impl Iterator<Item = T>> {
        self.kern_call_1(KernFnPath::OSEntConfig(KernOSEntConfigFnId::InitFind),
                         &self as *const _ as usize)
            .map(|iter_id| ConfigFinderIter::from(iter_id))
    }
}

impl<T, M> OSEntConfig<T, M>
    where T: OSEntity,
          M: ConfigMode
{
    const CFG_CREAT_BIT: usize = 0;
    const CFG_ADMIN_BIT: usize = 1;

    /** # Specifies an unique identifier
     *
     * Tells to the kernel which unique identifier the [`OSEntity`] must
     * obtain in [`CreatMode`] (the entire operation will fail if the id is
     * already assigned).
     *
     * Or tells exactly which identifier the searched OSEntity have in
     * [`FindMode`]
     *
     * [`OSEntity`]: crate::ents::entity::OSEntity
     * [`CreatMode`]: crate::config::CreatMode
     * [`FindMode`]: crate::config::FindMode
     */
    pub fn with_id(&mut self, id: u16) -> &mut Self {
        self.m_id = Some(id);
        self
    }
}

#[cfg(feature = "enable_kernel_methods")]
impl<T: OSEntity, M: ConfigMode> OSEntConfig<T, M> {
    /** Returns whether this configuration represents a creation request
     */
    pub fn is_creat(&self) -> bool {
        self.m_flags.get_bit(Self::CFG_CREAT_BIT)
    }

    /** Returns whether the admin filter/flag is enabled
     */
    pub fn is_admin(&self) -> bool {
        self.m_flags.get_bit(Self::CFG_ADMIN_BIT)
    }

    /** Returns the optional identifier given
     */
    pub fn id(&self) -> Option<u16> {
        self.m_id
    }

    /** Returns the optional name given
     */
    pub fn name(&self) -> Option<&[u8; ENTITY_NAME_LEN_MAX]> {
        self.m_name.as_ref()
    }

    /** Returns the [`OSEntityType`]
     *
     * [`OSEntityType`]: crate::bits::ent::types::OSEntityType
     */
    pub fn ent_type(&self) -> OSEntityType {
        self.m_type
    }
}

impl<T, M> KernCaller for OSEntConfig<T, M>
    where T: OSEntity,
          M: ConfigMode
{
    /* Nothing to implement */
}
