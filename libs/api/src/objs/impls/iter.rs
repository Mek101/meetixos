/*! # Iterator Reference
 *
 * Implements the reference to a kernel iterator
 */

use os::sysc::{
    codes::KrnIteratorFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::obj::{
        KrnIterDirection,
        ObjType,
        SeekMode
    },
    caller::{
        KernCaller,
        Result
    },
    objs::{
        ObjId,
        Object
    }
};

/** # Kernel Iterator
 *
 * Represents a reference to a double linked kernel's iteration pool.
 *
 * The kernel iterator could iterate different object types, from object
 * handles to more complex objects (like the [`DirEntry`]).
 *
 * This type of kernel's object is not creatable by the userspace, but
 * only by the kernel as response of system calls which return more
 * than one result (like the [`Task::find()`]).
 *
 * As consequence of the fact written above the `KrnIterator` is not
 * either used directly, but always wrapped into another structure that
 * ensures type validity
 *
 * [`DirEntry`]: crate::objs::impls::DirEntry
 * [`Task::find()`]: crate::tasks::Task::find
 */
#[repr(transparent)]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct KrnIterator {
    m_handle: ObjId
}

impl KrnIterator {
    /** # Sets the begin to end position
     *
     * According to the [`SeekMode`] given, it updates the begin to end
     * position
     *
     * [`SeekMode`]: crate::bits::obj::modes::SeekMode
     */
    pub fn set_begin_to_end_pos(&self, pos: SeekMode) -> Result<usize> {
        self.kern_call_1(KernFnPath::Iterator(KrnIteratorFnId::SetBeginToEndPos),
                         pos.mode())
    }

    /** # Sets the end to begin position
     *
     * According to the [`SeekMode`] given, it updates the end to begin
     * position
     *
     * [`SeekMode`]: crate::bits::obj::modes::SeekMode
     */
    pub fn set_end_to_begin_pos(&self, pos: SeekMode) -> Result<usize> {
        self.kern_call_1(KernFnPath::Iterator(KrnIteratorFnId::SetEndToBeginPos),
                         pos.mode())
    }

    /** # Finds the next iteration element
     *
     * Returns the next element into the pool reading from the begin to the
     * end
     */
    pub(crate) fn find_next<T>(&self) -> Result<Option<T>>
        where T: Default {
        self.next_res(KrnIterDirection::BeginToEnd)
    }

    /** # Finds the next back iteration element
     *
     * Returns the next element into the pool reading from the end to the
     * begin
     */
    pub(crate) fn find_next_back<T>(&self) -> Result<Option<T>>
        where T: Default {
        self.next_res(KrnIterDirection::EndToBegin)
    }

    /** # Finds the next iteration element
     *
     * Returns the next element into the pool reading the next element
     * according to the given [`KrnIterDirection`]
     *
     * [`KrnIterDirection`]: crate::bits::obj::modes::KrnIterDirection
     */
    fn next_res<T>(&self, direction: KrnIterDirection) -> Result<Option<T>>
        where T: Default {
        let mut buf = T::default();
        self.kern_call_2(KernFnPath::Iterator(KrnIteratorFnId::NextValue),
                         direction.into(),
                         &mut buf as *mut _ as usize)
            .map(|res| {
                /* the kernel returns as result an unsigned integer that is non-zero
                 * for good result, 0 otherwise
                 */
                if res != 0 {
                    Some(buf)
                } else {
                    None
                }
            })
    }
}

impl Object for KrnIterator {
    /** The value of the [`ObjType`] that matches the implementation
     *
     * [`ObjType`]: crate::bits::obj::types::ObjType
     */
    const TYPE: ObjType = ObjType::KrnIterator;

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

impl From<ObjId> for KrnIterator {
    /** Performs the conversion
     */
    fn from(id: ObjId) -> Self {
        Self { m_handle: id }
    }
}

impl KernCaller for KrnIterator {
    /** Returns the upper 32bits of the 64bit identifier of a system call
     */
    fn caller_handle_bits(&self) -> u32 {
        self.obj_handle().caller_handle_bits()
    }
}
