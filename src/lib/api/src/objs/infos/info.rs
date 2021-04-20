/*! # Object Informations
 *
 * Implements the metadata descriptor common to all the implementor of
 * [`Object`]
 *
 * [`Object`]: /api/objs/trait.Object.html
 */

use core::str;

use bit_field::BitField;

use os::{limits::VFS_NAME_LEN_MAX, str_utils};

use crate::{
    bits::obj::{Grants, ObjType},
    caller::Result,
    ents::impls::{OSGroup, OSUser},
    objs::{ObjId, Object},
    time::Instant
};

/** # `Object` Informations
 *
 * Implements a simple metadata descriptor common to all the [`Object`]
 * implementors.
 *
 * This struct takes the place of the old-style Unix-like's [`stat struct`]
 * with a little improvement:
 * * There is no more need of tons of single system calls to update small
 *   piece of object's metadata, there is only one cumulative info update
 *   call that overwrites the previous one
 * * Using [RAII] the informations are updated when the struct goes out of
 *   scope or when called explicitly [`ObjInfo::update()`]
 *
 * [`Object`]: /api/objs/trait.Object.html
 * [`stat struct`]: https://man7.org/linux/man-pages/man2/stat.2.html
 * [RAII]: https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization
 * [`ObjInfo::update()`]: /api/objs/infos/struct.ObjInfo.html#method.update
 */
#[derive(Debug, Default)]
pub struct ObjInfo<T>
    where T: Object {
    m_obj: ObjId,
    m_type: ObjType,
    m_name: Option<ObjNameInfo>,
    m_size: usize,
    m_block_size: u32,
    m_os_user: OSUser,
    m_os_group: OSGroup,
    m_grants: Grants<T>,
    m_creat_inst: Instant,
    m_access_inst: Instant,
    m_data_modify_inst: Instant,
    m_info_modify_inst: Instant,
    m_to_update: u8
}

#[cfg(feature = "enable_kernel_methods")]
impl<T> ObjInfo<T> where T: Object {
    /** # Constructs an `ObjInfo`
     *
     * The returned instance is filled with the given data
     */
    pub fn new(obj_type: ObjType,
               name: Option<ObjNameInfo>,
               size: usize,
               block_size: u32,
               os_user: OSUser,
               os_group: OSGroup,
               grants: Grants<T>,
               creat_inst: Instant,
               access_inst: Instant,
               data_modify_inst: Instant,
               info_modify_inst: Instant)
               -> Self {
        Self { m_obj: ObjId::default(),
               m_type: obj_type,
               m_name: name,
               m_size: size,
               m_block_size: block_size,
               m_os_user: os_user,
               m_os_group: os_group,
               m_grants: grants,
               m_creat_inst: creat_inst,
               m_access_inst: access_inst,
               m_data_modify_inst: data_modify_inst,
               m_info_modify_inst: info_modify_inst,
               m_to_update: 0 }
    }
}

impl<T> ObjInfo<T> where T: Object {
    const UPDATE_NAME_BIT: usize = 0;
    const UPDATE_USER_BIT: usize = 1;
    const UPDATE_GROUP_BIT: usize = 2;
    const UPDATE_GRANTS_BIT: usize = 3;
    const UPDATE_CREAT_DATE_BIT: usize = 4;
    const UPDATE_ACCESS_DATE_BIT: usize = 5;
    const UPDATE_DATA_MODIFY_BIT: usize = 6;
    const UPDATE_INFO_MODIFY_BIT: usize = 7;

    /// Overwrites the stored [`ObjId`] cloning the given one
    ///
    /// [`ObjId`]: /api/objs/struct.ObjId.html
    pub(crate) fn set_obj(&mut self, obj: &ObjId) {
        self.m_obj = obj.clone()
    }

    /** Returns the concrete [`ObjType`] of the object
     *
     * [`ObjType`]: /api/bits/obj/enum.ObjType.html
     */
    pub fn obj_type(&self) -> ObjType {
        self.m_type
    }

    /** Returns whether the object have a name (it is represented into the
     * VFS tree)
     */
    pub fn is_named(&self) -> bool {
        self.m_name.is_some()
    }

    /** Returns a reference to the [`ObjNameInfo`]
     *
     * [`ObjNameInfo`]: /api/objs/infos/struct.ObjNameInfo.html
     */
    pub fn name_infos(&self) -> Option<&ObjNameInfo> {
        self.m_name.as_ref()
    }

    /** Returns a mutable reference to the [`ObjNameInfo`]
     *
     * [`ObjNameInfo`]: /api/objs/infos/struct.ObjNameInfo.html
     */
    pub fn name_infos_mut(&mut self) -> Option<&mut ObjNameInfo> {
        self.m_to_update.set_bit(Self::UPDATE_NAME_BIT, true);
        self.m_name.as_mut()
    }

    /** Returns the occupied memory in bytes by the object's data
     */
    pub fn size(&self) -> usize {
        self.m_size
    }

    /** Returns the block allocation unit used
     */
    pub fn block_size(&self) -> u32 {
        self.m_block_size
    }

    /** Returns the [`OSUser`] that owns the object
     *
     * [`OSUser`]: /api/ents/impls/struct.OSUser.html
     */
    pub fn os_user(&self) -> OSUser {
        self.m_os_user
    }

    /** Updates the owner [`OSUser`] of the object
     *
     * [`OSUser`]: /api/ents/impls/struct.OSUser.html
     */
    pub fn set_os_user(&mut self, os_user: OSUser) {
        self.m_to_update.set_bit(Self::UPDATE_USER_BIT, true);
        self.m_os_user = os_user;
    }

    /** Returns the [`OSGroup`] that owns the object
     *
     * [`OSGroup`]: /api/ents/impls/struct.OSGroup.html
     */
    pub fn os_group(&self) -> OSGroup {
        self.m_os_group
    }

    /** Updates the owner [`OSGroup`] of the object
     *
     * [`OSGroup`]: /api/ents/impls/struct.OSGroup.html
     */
    pub fn set_os_group(&mut self, os_group: OSGroup) {
        self.m_to_update.set_bit(Self::UPDATE_GROUP_BIT, true);
        self.m_os_group = os_group
    }

    /** Returns the [`Grants`] descriptor related to the owner [`OSUser`] and
     * [`OSGroup`]
     *
     * [`Grants`]: /api/bits/obj/struct.Grants.html
     * [`OSUser`]: /api/ents/impls/struct.OSUser.html
     * [`OSGroup`]: /api/ents/impls/struct.OSGroup.html
     */
    pub fn grants(&self) -> Grants<T> {
        self.m_grants
    }

    /** Returns an updatable reference to the [`Grants`] instance
     *
     * [`Grants`]: /api/bits/obj/struct.Grants.html
     */
    pub fn set_grants(&mut self) -> &mut Grants<T> {
        self.m_to_update.set_bit(Self::UPDATE_GRANTS_BIT, true);
        &mut self.m_grants
    }

    /** Returns the object creation [`Instant`]
     *
     * [`Instant`]: /api/time/struct.Instant.html
     */
    pub fn creat_time(&self) -> Instant {
        self.m_creat_inst
    }

    /** Returns an updatable reference to the creation [`Instant`]
     *
     * [`Instant`]: /api/time/struct.Instant.html
     */
    pub fn set_creat_time(&mut self) -> &mut Instant {
        self.m_to_update.set_bit(Self::UPDATE_CREAT_DATE_BIT, true);
        &mut self.m_creat_inst
    }

    /** Returns the object last access [`Instant`]
     *
     * [`Instant`]: /api/time/struct.Instant.html
     */
    pub fn access_time(&self) -> Instant {
        self.m_access_inst
    }

    /** Returns an updatable reference to the last access [`Instant`]
     *
     * [`Instant`]: /api/time/struct.Instant.html
     */
    pub fn set_access_time(&mut self) -> &mut Instant {
        self.m_to_update.set_bit(Self::UPDATE_ACCESS_DATE_BIT, true);
        &mut self.m_access_inst
    }

    /** Returns the object last data modification [`Instant`]
     *
     * [`Instant`]: /api/time/struct.Instant.html
     */
    pub fn data_modify_time(&self) -> Instant {
        self.m_data_modify_inst
    }

    /** Returns an updatable reference to the last data modification
     * [`Instant`]
     *
     * [`Instant`]: /api/time/struct.Instant.html
     */
    pub fn set_data_modify_time(&mut self) -> &mut Instant {
        self.m_to_update.set_bit(Self::UPDATE_DATA_MODIFY_BIT, true);
        &mut self.m_data_modify_inst
    }

    /** Returns the object last information modification [`Instant`]
     *
     * [`Instant`]: /api/time/struct.Instant.html
     */
    pub fn info_modify_time(&self) -> Instant {
        self.m_info_modify_inst
    }

    /** Returns an updatable reference to the last information modification
     * [`Instant`]
     *
     * [`Instant`]: /api/time/struct.Instant.html
     */
    pub fn set_info_modify_time(&mut self) -> &mut Instant {
        self.m_to_update.set_bit(Self::UPDATE_INFO_MODIFY_BIT, true);
        &mut self.m_info_modify_inst
    }

    /** # Returns all the `Instant` timestamps
     *
     * They are respectively:
     * 0. Creation [`Instant`]
     * 1. Last access [`Instant`]
     * 2. Last data modify [`Instant`]
     * 3. Last info modify [`Instant`]
     *
     * [`Instant`]: /api/time/struct.Instant.html
     */
    pub fn timestamps(&self) -> (Instant, Instant, Instant, Instant) {
        (self.m_creat_inst,
         self.m_access_inst,
         self.m_data_modify_inst,
         self.m_info_modify_inst)
    }

    /** # Commits the updates
     *
     * Commits the changed fields to the kernel.
     *
     * This operation can be performed only if the referenced object is
     * valid and the caller have information write grants.
     */
    pub fn update(&mut self) -> Result<()> {
        if self.m_to_update != 0 {
            self.m_obj.update_infos(self).map(|_| self.reset_update())
        } else {
            // the object is not modified, so return okay anyway
            Ok(())
        }
    }

    /** # Discard fields to update
     *
     * Sets to 0 the flags that keeps track of the modified field of the
     * instance
     */
    pub fn reset_update(&mut self) -> () {
        self.m_to_update = 0;
        ()
    }
}

impl<T> Drop for ObjInfo<T> where T: Object {
    /** Calls [`self.update()`] to update the modified stuffs
     *
     * [`self.update()`]: /api/objs/infos/struct.ObjInfo.html#method.update
     */
    fn drop(&mut self) {
        self.update().unwrap_or_default()
    }
}

/** # VFS Name Infos
 *
 * Contains various informations that are related to the VFS representation
 * of an [`Object`] based type
 *
 * [`Object`]: /api/objs/trait.Object.html
 */
#[derive(Debug, Copy, Clone)]
pub struct ObjNameInfo {
    m_name_id: u64,
    m_name_buf: [u8; VFS_NAME_LEN_MAX],
    m_name_len: usize,
    m_links: u32
}

#[cfg(feature = "enable_kernel_methods")]
impl ObjNameInfo {
    /** # Constructs an `ObjNameInfo`
     *
     * The returned instance contains the given data
     */
    pub fn new(name_id: u64, name: &str, links: u32) -> Self {
        let mut buf = [0; VFS_NAME_LEN_MAX];
        str_utils::copy_str_to_u8_buf(&mut buf, name);

        Self { m_name_id: name_id,
               m_name_buf: buf,
               m_name_len: name.len(),
               m_links: links }
    }
}

impl ObjNameInfo {
    /** Returns the name unique identifier
     */
    pub fn name_id(&self) -> u64 {
        self.m_name_id
    }

    /** Returns the name as encoded string slice
     */
    pub fn name(&self) -> &str {
        str_utils::u8_ptr_to_str_slice(self.m_name_buf.as_ptr(), self.m_name_len)
    }

    /** # Set a new name
     *
     * The new name will be applied to the object when [`ObjInfo::update()`]
     * will be called
     *
     * [`ObjInfo::update()`]:
     * /api/objs/infos/struct.ObjInfo.html#method.update
     */
    pub fn set_name(&mut self, new_name: &str) {
        str_utils::copy_str_to_u8_buf(&mut self.m_name_buf, new_name)
    }

    /** Returns the number of [`Link`]s that refers this object
     *
     * [`Link`]: /api/objs/impls/struct.Link.html
     */
    pub fn links_count(&self) -> u32 {
        self.m_links
    }
}
