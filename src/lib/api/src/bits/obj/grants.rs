/*! # Grants Descriptor
 *
 * Implements the the descriptor of the permissions under the MeetiX
 * platform
 */

use core::marker::PhantomData;

use bit_field::BitField;

use crate::objs::{
    impls::{Dir, File, IpcChan, Link, MMap, OsRawMutex},
    Object
};

/** # Data Grants
 *
 * Represents the permissions bits for the [`Object`] implementations
 *
 * [`Object`]: /api/objs/trait.Object.html
 */
#[derive(Debug)]
pub struct Grants<T>
    where T: Object {
    m_bits: u32,
    _unused: PhantomData<T>
}

impl<T> Grants<T> where T: Object {
    /** `Open` permission bits
     */
    pub const OPENABLE_USER_BIT: usize = 0;
    pub const OPENABLE_GROUP_BIT: usize = 1;
    pub const OPENABLE_OTHER_BIT: usize = 2;

    /** Owner user `data` permission bits
     */
    pub const DATA_USER_READ_BIT: usize = 3;
    pub const DATA_USER_WRITE_BIT: usize = 4;
    pub const DATA_USER_EXEC_BIT: usize = 5;
    pub const DATA_USER_TRAVERS_BIT: usize = Self::DATA_USER_EXEC_BIT;

    /** Owner's group `read` data permission bits
     */
    pub const DATA_GROUP_READ_BIT: usize = 6;
    pub const DATA_GROUP_WRITE_BIT: usize = 7;
    pub const DATA_GROUP_EXEC_BIT: usize = 8;
    pub const DATA_GROUP_TRAVERS_BIT: usize = Self::DATA_GROUP_EXEC_BIT;

    /** Other users/groups `data` permission bits
     */
    pub const DATA_OTHER_READ_BIT: usize = 9;
    pub const DATA_OTHER_WRITE_BIT: usize = 10;
    pub const DATA_OTHER_EXEC_BIT: usize = 11;
    pub const DATA_OTHER_TRAVERS_BIT: usize = Self::DATA_OTHER_EXEC_BIT;

    /** Owner user `information` permissions bits
     */
    pub const INFO_USER_READ_BIT: usize = 12;
    pub const INFO_USER_WRITE_BIT: usize = 13;

    /** Owner's group `informations` permissions bits
     */
    pub const INFO_GROUP_READ_BIT: usize = 14;
    pub const INFO_GROUP_WRITE_BIT: usize = 15;

    /** Other users/groups `informations` permissions bits
     */
    pub const INFO_OTHER_READ_BIT: usize = 16;
    pub const INFO_OTHER_WRITE_BIT: usize = 17;

    /** `Visibility` permission bits
     */
    pub const VISIBLE_USER_BIT: usize = 18;
    pub const VISIBLE_GROUP_BIT: usize = 19;
    pub const VISIBLE_OTHER_BIT: usize = 20;

    /** # Constructs a new `Grants`
     *
     * The returned instance have no active bits
     */
    pub fn new() -> Self {
        Self { m_bits: 0,
               _unused: Default::default() }
    }

    /** # Sets the `OPENABLE_BIT`s
     *
     * The values given as arguments are used as bit values for,
     * respectively:
     * * [`OPENABLE_USER_BIT`]
     * * [`OPENABLE_GROUP_BIT`]
     * * [`OPENABLE_OTHER_BIT`]
     *
     * When the caller have `openable` permissions for an [`ObjId`] based
     * object means that it can call successfully [`Object::open()`]
     *
     * [`OPENABLE_USER_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.OPENABLE_USER_BIT
     * [`OPENABLE_GROUP_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * OPENABLE_GROUP_BIT [`OPENABLE_OTHER_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * OPENABLE_OTHER_BIT [`ObjId`]: /api/objs/struct.ObjId.html
     * [`Object::open()`]: /api/objs/trait.Object.html#method.open
     */
    pub fn set_openable(&mut self, user: bool, group: bool, other: bool) -> &mut Self {
        self.m_bits.set_bit(Self::OPENABLE_USER_BIT, user);
        self.m_bits.set_bit(Self::OPENABLE_GROUP_BIT, group);
        self.m_bits.set_bit(Self::OPENABLE_OTHER_BIT, other);
        self
    }

    /** # Sets the `DATA_READ_BIT`s
     *
     * The values given as arguments are used as bit values for,
     * respectively:
     * * [`DATA_USER_READ_BIT`]
     * * [`DATA_GROUP_READ_BIT`]
     * * [`DATA_OTHER_READ_BIT`]
     *
     * When the caller have `openable` and `data read` permissions for an
     * [`ObjId`] based objects means that it can apply successfully
     * [`ObjConfig::for_read()`]
     *
     * [`DATA_USER_READ_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * DATA_USER_READ_BIT [`DATA_GROUP_READ_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * DATA_GROUP_READ_BIT [`DATA_OTHER_READ_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * DATA_OTHER_READ_BIT [`ObjId`]: /api/objs/struct.ObjId.html
     * [`ObjConfig::for_read()`]:
     * /api/objs/struct.ObjConfig.html#method.for_read
     */
    pub fn set_data_readable(&mut self,
                             user: bool,
                             group: bool,
                             other: bool)
                             -> &mut Self {
        self.m_bits.set_bit(Self::DATA_USER_READ_BIT, user);
        self.m_bits.set_bit(Self::DATA_GROUP_READ_BIT, group);
        self.m_bits.set_bit(Self::DATA_OTHER_READ_BIT, other);
        self
    }

    /** # Sets the `DATA_WRITE_BIT`s
     *
     * The values given as arguments are used as bit values for,
     * respectively:
     * * [`DATA_USER_WRITE_BIT`]
     * * [`DATA_GROUP_WRITE_BIT`]
     * * [`DATA_OTHER_WRITE_BIT`]
     *
     * When the caller have `openable` and `data write` permissions for an
     * [`ObjId`] based objects means that it can apply successfully
     * [`ObjConfig::for_write()`]
     *
     * [`DATA_USER_WRITE_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * DATA_USER_WRITE_BIT [`DATA_GROUP_WRITE_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * DATA_GROUP_WRITE_BIT [`DATA_OTHER_WRITE_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * DATA_OTHER_WRITE_BIT [`ObjId`]: /api/objs/struct.ObjId.html
     * [`ObjConfig::for_write()`]:
     * /api/objs/struct.ObjConfig.html#method.for_write
     */
    pub fn set_data_writeable(&mut self,
                              user: bool,
                              group: bool,
                              other: bool)
                              -> &mut Self {
        self.m_bits.set_bit(Self::DATA_USER_WRITE_BIT, user);
        self.m_bits.set_bit(Self::DATA_GROUP_WRITE_BIT, group);
        self.m_bits.set_bit(Self::DATA_OTHER_WRITE_BIT, other);
        self
    }

    /** # Sets the `INFO_READ_BIT`s
     *
     * The values given as arguments are used as bit values for,
     * respectively:
     * * [`INFO_USER_READ_BIT`]
     * * [`INFO_GROUP_READ_BIT`]
     * * [`INFO_OTHER_READ_BIT`]
     *
     * When the caller have this permission can successfully call
     * [`Object::infos()`] and [`Object::watch()`]
     *
     * [`INFO_USER_READ_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * INFO_USER_READ_BIT [`INFO_GROUP_READ_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * INFO_GROUP_READ_BIT [`INFO_OTHER_READ_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * INFO_OTHER_READ_BIT [`Object::infos()`]:
     * /api/objs/trait.Object.html#method.infos [`Object::watch()`]:
     * /api/objs/trait.Object.html#method.watch
     */
    pub fn set_info_readable(&mut self,
                             user: bool,
                             group: bool,
                             other: bool)
                             -> &mut Self {
        self.m_bits.set_bit(Self::INFO_USER_READ_BIT, user);
        self.m_bits.set_bit(Self::INFO_GROUP_READ_BIT, group);
        self.m_bits.set_bit(Self::INFO_OTHER_READ_BIT, other);
        self
    }

    /** # Sets the `INFO_WRITE_BIT`s
     *
     * The values given as arguments are used as bit values for,
     * respectively:
     * * [`INFO_USER_WRITE_BIT`]
     * * [`INFO_GROUP_WRITE_BIT`]
     * * [`INFO_OTHER_WRITE_BIT`]
     *
     * When the caller have this permission can successfully call
     * [`ObjInfo::update()`] and [`Object::drop_name()`]
     *
     * [`INFO_USER_WRITE_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * INFO_USER_WRITE_BIT [`INFO_GROUP_WRITE_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * INFO_GROUP_WRITE_BIT [`INFO_OTHER_WRITE_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * INFO_OTHER_WRITE_BIT [`ObjInfo::update()`]:
     * /api/objs/infos/struct.ObjInfo.html#method.update
     * [`Object::drop_name()`]:
     * /api/objs/trait.Object.html#method.drop_name
     */
    pub fn set_info_writeable(&mut self,
                              user: bool,
                              group: bool,
                              other: bool)
                              -> &mut Self {
        self.m_bits.set_bit(Self::INFO_USER_WRITE_BIT, user);
        self.m_bits.set_bit(Self::INFO_GROUP_WRITE_BIT, group);
        self.m_bits.set_bit(Self::INFO_OTHER_WRITE_BIT, other);
        self
    }

    /** # Sets the `VISIBLE_BIT`s
     *
     * The values given as arguments are used as bit values for,
     * respectively:
     * * [`VISIBLE_USER_BIT`]
     * * [`VISIBLE_GROUP_BIT`]
     * * [`VISIBLE_OTHER_BIT`]
     *
     * These bits allows administrators to configure the filesystem point of
     * view for the OS users. A named object that have his visibility bit
     * disabled for the user means that it cannot be showed (but still
     * openable if have the `OPENABLE_BIT` enabled) when scanning the
     * parent directory
     *
     * [`VISIBLE_USER_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.VISIBLE_USER_BIT
     * [`VISIBLE_GROUP_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.VISIBLE_GROUP_BIT
     * [`VISIBLE_OTHER_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.VISIBLE_OTHER_BIT
     */
    pub fn set_visible(&mut self, user: bool, group: bool, other: bool) -> &mut Self {
        self.m_bits.set_bit(Self::VISIBLE_USER_BIT, user);
        self.m_bits.set_bit(Self::VISIBLE_GROUP_BIT, group);
        self.m_bits.set_bit(Self::VISIBLE_OTHER_BIT, other);
        self
    }

    /** # Builds the filled instance
     *
     * Builds a new instance consuming the filled one given
     */
    pub fn build(self) -> Self {
        self
    }

    /** Returns the bits of this `Grants` descriptor
     */
    pub fn bits(&self) -> u32 {
        self.m_bits
    }
}

impl<T> Grants<T> where T: Object + WithExecutableDataObject {
    #[rustfmt::skip] /* skip the format of this comment */
    /** # Sets the `DATA_EXEC_BIT`s
     *
     * The values given as arguments are used as bit values for,
     * respectively:
     * * [`DATA_USER_EXEC_BIT`]
     * * [`DATA_GROUP_EXEC_BIT`]
     * * [`DATA_OTHER_EXEC_BIT`]
     *
     * These permission bits enables behaviours that differs a bit based on
     * the type of the object they refers to:
     * * [`File`]s - The data content can be executed, so the file can be used
     *   as executable file for [`TaskConfig<Proc>::run()`]
     * * [`MMap`]s - The data content can be executed, this implies only that
     *   the [`MMap`] can be opened with [`ObjConfig::for_exec()`] and the
     *   kernel will maps the pages without [`PTFlags::NO_EXECUTE`]
     *
     * [`DATA_USER_EXEC_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.DATA_USER_EXEC_BIT
     * [`DATA_GROUP_EXEC_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.DATA_GROUP_EXEC_BIT
     * [`DATA_OTHER_EXEC_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.DATA_OTHER_EXEC_BIT
     * [`File`]: /api/objs/impls/struct.File.html
     * [`TaskConfig<Proc>::run()`]:
     * /api/tasks/struct.TaskConfig.html#method.run-1
     * [`MMap`]: /api/objs/impls/struct.MMap.html
     * [`ObjConfig::for_exec()`]:
     * /api/objs/struct.ObjConfig.html#method.for_exec
     * [`PTFlags::NO_EXECUTE`]:
     * /hal/paging/struct.PTFlags.html#associatedconstant.PTFlags::NO_EXECUTE
     */
    pub fn set_data_executable(&mut self,
                               user: bool,
                               group: bool,
                               other: bool)
                               -> &mut Self {
        self.m_bits.set_bit(Self::DATA_USER_EXEC_BIT, user);
        self.m_bits.set_bit(Self::DATA_GROUP_EXEC_BIT, group);
        self.m_bits.set_bit(Self::DATA_OTHER_EXEC_BIT, other);
        self
    }
}

impl<T> Grants<T> where T: Object + WithTraversableDataObject {
    /** # Sets the `DATA_TRAVERS_BIT`s
     *
     * The values given as arguments are used as bit values for,
     * respectively:
     * * [`DATA_USER_TRAVERSE_BIT`]
     * * [`DATA_GROUP_TRAVERSE_BIT`]
     * * [`DATA_OTHER_TRAVERSE_BIT`]
     *
     * These permission bits enable/disable the following behaviours:
     * * [`Dir`]s & [`Link`]s - Their name can be traversed when they are
     *   not the last path component. This means when a path contains a dir
     *   name either a link name in case one of them have no
     *   `DATA_TRAVERS_BIT` enabled the path resolution fails
     *
     * [`DATA_USER_TRAVERSE_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * DATA_USER_TRAVERS_BIT
     *
     * [`DATA_GROUP_TRAVERSE_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * DATA_GROUP_TRAVERS_BIT
     *
     * [`DATA_OTHER_TRAVERSE_BIT`]:
     * /api/bits/obj/struct.Grants.html#associatedconstant.
     * DATA_OTHER_TRAVERS_BIT
     *
     * [`Dir`]: /api/objs/impls/struct.Dir.html
     * [`Link`]: /api/objs/impls/struct.Link.html
     */
    pub fn set_data_traversable(&mut self,
                                user: bool,
                                group: bool,
                                other: bool)
                                -> &mut Self {
        self.m_bits.set_bit(Self::DATA_USER_TRAVERS_BIT, user);
        self.m_bits.set_bit(Self::DATA_GROUP_TRAVERS_BIT, group);
        self.m_bits.set_bit(Self::DATA_OTHER_TRAVERS_BIT, other);
        self
    }
}

impl<T> Clone for Grants<T> where T: Object {
    /** Returns a copy of the value
     */
    fn clone(&self) -> Self {
        Self { m_bits: self.bits(),
               _unused: Default::default() }
    }
}

impl<T> Copy for Grants<T> where T: Object {
    /* no methods to implement, just a marker */
}

impl<T> From<u32> for Grants<T> where T: Object {
    /** Performs the conversion
     */
    fn from(code: u32) -> Self {
        Self { m_bits: code,
               _unused: Default::default() }
    }
}

impl Default for Grants<Dir> {
    /** Returns the default [`Grants`] for a [`Dir`]
     *
     * [`Grants`]: /api/bits/obj/struct.Grants.html
     * [`Dir`]: /api/objs/impls/struct.Dir.html
     */
    fn default() -> Self {
        Self::new().set_openable(true, true, true)
                   .set_data_readable(true, true, true)
                   .set_data_writeable(true, false, false)
                   .set_data_traversable(true, true, true)
                   .set_info_readable(true, true, true)
                   .set_info_writeable(true, false, false)
                   .set_visible(true, true, true)
                   .build()
    }
}

impl Default for Grants<File> {
    /** Returns the default [`Grants`] for a [`File`]
     *
     * [`Grants`]: /api/bits/obj/struct.Grants.html
     * [`File`]: /api/objs/impls/struct.File.html
     */
    fn default() -> Self {
        Self::new().set_openable(true, true, true)
                   .set_data_readable(true, true, true)
                   .set_data_writeable(true, true, false)
                   .set_data_executable(true, false, false)
                   .set_info_readable(true, true, false)
                   .set_info_writeable(true, false, false)
                   .set_visible(true, true, true)
                   .build()
    }
}

impl Default for Grants<IpcChan> {
    /** Returns the default [`Grants`] for a [`IpcChan`]
     *
     * [`Grants`]: /api/bits/obj/struct.Grants.html
     * [`IpcChan`]: /api/objs/impls/struct.IpcChan.html
     */
    fn default() -> Self {
        Self::new().set_openable(true, true, true)
                   .set_data_readable(true, true, true)
                   .set_data_writeable(true, true, true)
                   .set_info_readable(true, true, true)
                   .set_info_writeable(false, false, false)
                   .set_visible(true, true, true)
                   .build()
    }
}

impl Default for Grants<Link> {
    /** Returns the default [`Grants`] for a [`Link`]
     *
     * [`Grants`]: /api/bits/obj/struct.Grants.html
     * [`Link`]: /api/objs/impls/struct.Link.html
     */
    fn default() -> Self {
        Self::new().set_openable(true, true, true)
                   .set_data_readable(true, true, true)
                   .set_data_writeable(true, true, false)
                   .set_data_traversable(true, true, true)
                   .set_info_readable(true, true, true)
                   .set_info_writeable(true, false, false)
                   .set_visible(true, true, true)
                   .build()
    }
}

impl Default for Grants<MMap> {
    /** Returns the default [`Grants`] for a [`MMap`]
     *
     * [`Grants`]: /api/bits/obj/struct.Grants.html
     * [`MMap`]: /api/objs/impls/struct.MMap.html
     */
    fn default() -> Self {
        Self::new().set_openable(true, true, false)
                   .set_data_readable(true, true, true)
                   .set_data_writeable(true, true, true)
                   .set_data_executable(false, false, false)
                   .set_info_readable(true, true, false)
                   .set_info_writeable(true, true, false)
                   .set_visible(true, true, false)
                   .build()
    }
}

impl Default for Grants<OsRawMutex> {
    /** Returns the default [`Grants`] for a [`OsRawMutex`]
     *
     * [`Grants`]: /api/bits/obj/struct.Grants.html
     * [`OsRawMutex`]: /api/objs/impls/struct.OsRawMutex.html
     */
    fn default() -> Self {
        Self::new().set_openable(true, true, true)
                   .set_data_readable(true, true, true)
                   .set_data_writeable(true, true, true)
                   .set_info_readable(true, true, true)
                   .set_info_writeable(false, false, false)
                   .set_visible(true, true, true)
                   .build()
    }
}

impl<T> Default for Grants<T> where T: Object {
    /** Implemented to shut the warning of the compiler about overlapping
     * implementations of the `Default` trait
     */
    default fn default() -> Self {
        Self::new()
    }
}

/** # Executable Data Marker
 *
 * Marker trait implemented for the objects that have meaning with concept
 * of data execution as machine instructions, like [`File`] and [`MMap`]
 *
 * [`File`]: /api/objs/impls/struct.File.html
 * [`MMap`]: /api/objs/impls/struct.MMap.html
 */
pub trait WithExecutableDataObject {
    /* No methods, just a marker trait */
}

/** # Traversable Data Marker
 *
 * Marker trait implemented for the objects that have meaning with concept
 * of traversable data, like [`Link`] and [`Dir`]
 *
 * [`Link`]: /api/objs/impls/struct.Link.html
 * [`Dir`]: /api/objs/impls/struct.Dir.html
 */
pub trait WithTraversableDataObject {
    /* No methods, just a marker trait */
}
