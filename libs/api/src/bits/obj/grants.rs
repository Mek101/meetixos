/*! # Grants Descriptor
 *
 * Implements the the descriptor of the permissions under the MeetiX
 * platform
 */

use core::marker::PhantomData;

use bit_field::BitField;

use crate::objs::{
    impls::{
        Dir,
        File,
        IpcChan,
        Link,
        MMap,
        OsRawMutex
    },
    Object
};

/** # Data Grants
 *
 * Represents the permissions bits for the [`Object`] implementations
 *
 * [`Object`]: crate::objs::Object
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
    pub const OPEN_USER: usize = 0;
    pub const OPEN_GROUP: usize = 1;
    pub const OPEN_OTHER: usize = 2;

    /** Owner user `data` permission bits
     */
    pub const DATA_USER_READ: usize = 3;
    pub const DATA_USER_WRITE: usize = 4;
    pub const DATA_USER_EXEC: usize = 5;
    pub const DATA_USER_TRAVERS: usize = Self::DATA_USER_EXEC;

    /** Owner's group `read` data permission bits
     */
    pub const DATA_GROUP_READ: usize = 6;
    pub const DATA_GROUP_WRITE: usize = 7;
    pub const DATA_GROUP_EXEC: usize = 8;
    pub const DATA_GROUP_TRAVERS: usize = Self::DATA_GROUP_EXEC;

    /** Other users/groups `data` permission bits
     */
    pub const DATA_OTHER_READ: usize = 9;
    pub const DATA_OTHER_WRITE: usize = 10;
    pub const DATA_OTHER_EXEC: usize = 11;
    pub const DATA_OTHER_TRAVERS: usize = Self::DATA_OTHER_EXEC;

    /** Owner user `information` permissions bits
     */
    pub const INFO_USER_READ: usize = 12;
    pub const INFO_USER_WRITE: usize = 13;

    /** Owner's group `informations` permissions bits
     */
    pub const INFO_GROUP_READ: usize = 14;
    pub const INFO_GROUP_WRITE: usize = 15;

    /** Other users/groups `informations` permissions bits
     */
    pub const INFO_OTHER_READ: usize = 16;
    pub const INFO_OTHER_WRITE: usize = 17;

    /** `Visibility` permission bits
     */
    pub const VISIBLE_USER: usize = 18;
    pub const VISIBLE_GROUP: usize = 19;
    pub const VISIBLE_OTHER: usize = 20;

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
     * * [`OPENABLE_USER`][OU]
     * * [`OPENABLE_GROUP`][OG]
     * * [`OPENABLE_OTHER`][OH]
     *
     * When the caller have `openable` permissions for an [`ObjId`] based
     * object means that it can call successfully [`Object::open()`]
     *
     * [OU]: crate::bits::obj::Grants::OPENABLE_USER
     * [OG]: crate::bits::obj::Grants::OPENABLE_GROUP
     * [OH]: crate::bits::obj::Grants::OPENABLE_OTHER
     * [`ObjId`]: crate::objs::ObjId
     * [`Object::open()`]: crate::objs::Object::open
     */
    pub fn set_openable(&mut self, user: bool, group: bool, other: bool) -> &mut Self {
        self.m_bits.set_bit(Self::OPEN_USER, user);
        self.m_bits.set_bit(Self::OPEN_GROUP, group);
        self.m_bits.set_bit(Self::OPEN_OTHER, other);
        self
    }

    /** # Sets the `DATA_READ_BIT`s
     *
     * The values given as arguments are used as bit values for,
     * respectively:
     * * [`DATA_USER_READ`][DU]
     * * [`DATA_GROUP_READ`][DG]
     * * [`DATA_OTHER_READ`][DO]
     *
     * When the caller have `openable` and `data read` permissions for an
     * [`ObjId`] based objects means that it can apply successfully
     * [`ObjConfig::for_read()`]
     *
     * [DU]: crate::bits::obj::Grants::DATA_USER_READ
     * [DG]: crate::bits::obj::Grants::DATA_GROUP_READ
     * [DO]: crate::bits::obj::Grants::DATA_OTHER_READ
     * [`ObjId`]: crate::objs::ObjId
     * [`ObjConfig::for_read()`]: crate::objs::ObjConfig::for_read
     */
    pub fn set_data_readable(&mut self,
                             user: bool,
                             group: bool,
                             other: bool)
                             -> &mut Self {
        self.m_bits.set_bit(Self::DATA_USER_READ, user);
        self.m_bits.set_bit(Self::DATA_GROUP_READ, group);
        self.m_bits.set_bit(Self::DATA_OTHER_READ, other);
        self
    }

    /** # Sets the `DATA_WRITE_BIT`s
     *
     * The values given as arguments are used as bit values for,
     * respectively:
     * * [`DATA_USER_WRITE`][DU]
     * * [`DATA_GROUP_WRITE`][DG]
     * * [`DATA_OTHER_WRITE`][DO]
     *
     * When the caller have `openable` and `data write` permissions for an
     * [`ObjId`] based objects means that it can apply successfully
     * [`ObjConfig::for_write()`]
     *
     * [DU]: crate::bits::obj::Grants::DATA_USER_WRITE
     * [DG]: crate::bits::obj::Grants::DATA_GROUP_WRITE
     * [DO]: crate::bits::obj::Grants::DATA_OTHER_WRITE
     * [`ObjId`]: crate::objs::ObjId
     * [`ObjConfig::for_write()`]: crate::objs::ObjConfig::for_write
     */
    pub fn set_data_writeable(&mut self,
                              user: bool,
                              group: bool,
                              other: bool)
                              -> &mut Self {
        self.m_bits.set_bit(Self::DATA_USER_WRITE, user);
        self.m_bits.set_bit(Self::DATA_GROUP_WRITE, group);
        self.m_bits.set_bit(Self::DATA_OTHER_WRITE, other);
        self
    }

    /** # Sets the `INFO_READ_BIT`s
     *
     * The values given as arguments are used as bit values for,
     * respectively:
     * * [`INFO_USER_READ`][IU]
     * * [`INFO_GROUP_READ`][IG]
     * * [`INFO_OTHER_READ`][IO]
     *
     * When the caller have this permission can successfully call
     * [`Object::infos()`] and [`Object::watch()`]
     *
     * [IU]: crate::bits::obj::Grants::INFO_USER_READ
     * [IG]: crate::bits::obj::Grants::INFO_GROUP_READ
     * [IO]: crate::bits::obj::Grants::INFO_OTHER_READ
     * [`Object::infos()`]: crate::objs::Object::infos
     * [`Object::watch()`]: crate::objs::Object::watch
     */
    pub fn set_info_readable(&mut self,
                             user: bool,
                             group: bool,
                             other: bool)
                             -> &mut Self {
        self.m_bits.set_bit(Self::INFO_USER_READ, user);
        self.m_bits.set_bit(Self::INFO_GROUP_READ, group);
        self.m_bits.set_bit(Self::INFO_OTHER_READ, other);
        self
    }

    /** # Sets the `INFO_WRITE_BIT`s
     *
     * The values given as arguments are used as bit values for,
     * respectively:
     * * [`INFO_USER_WRITE`][IU]
     * * [`INFO_GROUP_WRITE`][IG]
     * * [`INFO_OTHER_WRITE`][IO]
     *
     * When the caller have this permission can successfully call
     * [`ObjInfo::update()`] and [`Object::drop_name()`]
     *
     * [IU]: crate::bits::obj::Grants::INFO_USER_WRITE
     * [IG]: crate::bits::obj::Grants::INFO_GROUP_WRITE
     * [IO]: crate::bits::obj::Grants::INFO_OTHER_WRITE
     * [`ObjInfo::update()`]: api::objs::infos::ObjInfo::update
     * [`Object::drop_name()`]: crate::objs::Object::drop_name
     */
    pub fn set_info_writeable(&mut self,
                              user: bool,
                              group: bool,
                              other: bool)
                              -> &mut Self {
        self.m_bits.set_bit(Self::INFO_USER_WRITE, user);
        self.m_bits.set_bit(Self::INFO_GROUP_WRITE, group);
        self.m_bits.set_bit(Self::INFO_OTHER_WRITE, other);
        self
    }

    /** # Sets the `VISIBLE_BIT`s
     *
     * The values given as arguments are used as bit values for,
     * respectively:
     * * [`VISIBLE_USER`][VU]
     * * [`VISIBLE_GROUP`][VG]
     * * [`VISIBLE_OTHER`][VO]
     *
     * These bits allows administrators to configure the filesystem point of
     * view for the OS users. A named object that have his visibility bit
     * disabled for the user means that it cannot be showed (but still
     * openable if have the `OPENABLE_BIT` enabled) when scanning the
     * parent directory
     *
     * [VU]: crate::bits::obj::Grants::VISIBLE_USER
     * [VG]: crate::bits::obj::Grants::VISIBLE_GROUP
     * [VO]: crate::bits::obj::Grants::VISIBLE_OTHER
     */
    pub fn set_visible(&mut self, user: bool, group: bool, other: bool) -> &mut Self {
        self.m_bits.set_bit(Self::VISIBLE_USER, user);
        self.m_bits.set_bit(Self::VISIBLE_GROUP, group);
        self.m_bits.set_bit(Self::VISIBLE_OTHER, other);
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
    /** # Sets the `DATA_EXEC_BIT`s
     *
     * The values given as arguments are used as bit values for,
     * respectively:
     * * [`DATA_USER_EXEC`][DU]
     * * [`DATA_GROUP_EXEC`][DG]
     * * [`DATA_OTHER_EXEC`][DO]
     *
     * These permission bits enables behaviours that differs a bit based on
     * the type of the object they refers to:
     * * [`File`]s - The data content can be executed, so the file can be
     *   used as executable file for [`TaskConfig<Proc>::run()`]
     * * [`MMap`]s - The data content can be executed, this implies only
     *   that the [`MMap`] can be opened with [`ObjConfig::for_exec()`] and
     *   the kernel will maps the pages without [`PTFlags::NO_EXECUTE`][NE]
     *
     * [DU]: crate::bits::obj::Grants::DATA_USER_EXEC
     * [DG]: crate::bits::obj::Grants::DATA_GROUP_EXEC
     * [DO]: crate::bits::obj::Grants::DATA_OTHER_EXEC
     * [`File`]: crate::objs::impls::File
     * [`TaskConfig<Proc>::run()`]: crate::tasks::TaskConfig::run
     * [`MMap`]: crate::objs::impls::MMap
     * [`ObjConfig::for_exec()`]: crate::objs::ObjConfig::for_exec
     * [NE]: shared::mem::paging::table::PTFlags::NO_EXECUTE
     */
    pub fn set_data_executable(&mut self,
                               user: bool,
                               group: bool,
                               other: bool)
                               -> &mut Self {
        self.m_bits.set_bit(Self::DATA_USER_EXEC, user);
        self.m_bits.set_bit(Self::DATA_GROUP_EXEC, group);
        self.m_bits.set_bit(Self::DATA_OTHER_EXEC, other);
        self
    }
}

impl<T> Grants<T> where T: Object + WithTraversableDataObject {
    /** # Sets the `DATA_TRAVERS_BIT`s
     *
     * The values given as arguments are used as bit values for,
     * respectively:
     * * [`DATA_USER_TRAVERSE`][TU]
     * * [`DATA_GROUP_TRAVERSE`][TG]
     * * [`DATA_OTHER_TRAVERSE`][TO]
     *
     * These permission bits enable/disable the following behaviours:
     * * [`Dir`]s & [`Link`]s - Their name can be traversed when they are
     *   not the last path component. This means when a path contains a dir
     *   name either a link name in case one of them have no
     *   `DATA_TRAVERS_BIT` enabled the path resolution fails
     *
     * [TU]: crate::bits::obj::Grants::DATA_USER_TRAVERS
     * [TG]: crate::bits::obj::Grants::DATA_GROUP_TRAVERS
     * [TO]: crate::bits::obj::Grants::DATA_OTHER_TRAVERS
     * [`Dir`]: crate::objs::impls::Dir
     * [`Link`]: crate::objs::impls::Link
     */
    pub fn set_data_traversable(&mut self,
                                user: bool,
                                group: bool,
                                other: bool)
                                -> &mut Self {
        self.m_bits.set_bit(Self::DATA_USER_TRAVERS, user);
        self.m_bits.set_bit(Self::DATA_GROUP_TRAVERS, group);
        self.m_bits.set_bit(Self::DATA_OTHER_TRAVERS, other);
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
     * [`Grants`]: crate::bits::obj::Grants
     * [`Dir`]: crate::objs::impls::Dir
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
     * [`Grants`]: crate::bits::obj::Grants
     * [`File`]: crate::objs::impls::File
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
     * [`Grants`]: crate::bits::obj::Grants
     * [`IpcChan`]: crate::objs::impls::IpcChan
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
     * [`Grants`]: crate::bits::obj::Grants
     * [`Link`]: crate::objs::impls::Link
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
     * [`Grants`]: crate::bits::obj::Grants
     * [`MMap`]: crate::objs::impls::MMap
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
     * [`Grants`]: crate::bits::obj::Grants
     * [`OsRawMutex`]: crate::objs::impls::OsRawMutex
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
 * [`File`]: crate::objs::impls::File
 * [`MMap`]: crate::objs::impls::MMap
 */
pub trait WithExecutableDataObject {
    /* No methods, just a marker trait */
}

/** # Traversable Data Marker
 *
 * Marker trait implemented for the objects that have meaning with concept
 * of traversable data, like [`Link`] and [`Dir`]
 *
 * [`Link`]: crate::objs::impls::Link
 * [`Dir`]: crate::objs::impls::Dir
 */
pub trait WithTraversableDataObject {
    /* No methods, just a marker trait */
}
