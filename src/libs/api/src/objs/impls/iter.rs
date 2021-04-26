/*! # Iterator Reference
 *
 * Implements the reference to a kernel iterator
 */

use os::sysc::{codes::KrnIteratorFnId, fn_path::KernFnPath};

use crate::{
    bits::obj::{KrnIterDirection, ObjType, SeekMode},
    caller::{KernCaller, Result},
    objs::{ObjId, Object}
};

impl_obj_id_object! {
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
     * [`DirEntry`]: /api/objs/impls/struct.DirEntry.html
     * [`Task::find()`]: /api/tasks/trait.Task.html#method.find
     */
    pub struct KrnIterator {
        where TYPE = ObjType::Iterator;
    }
}

impl KrnIterator {
    /** # Sets the begin to end position
     *
     * According to the [`SeekMode`] given, it updates the begin to end
     * position
     *
     * [`SeekMode`]: /api/bits/obj/enum.SeekMode.html
     */
    pub fn set_begin_to_end_pos(&self, pos: SeekMode) -> Result<usize> {
        self.kern_call_1(KernFnPath::Iterator(KrnIteratorFnId::SetBeginToEndPos),
                         pos.into())
    }

    /** # Sets the end to begin position
     *
     * According to the [`SeekMode`] given, it updates the end to begin
     * position
     *
     * [`SeekMode`]: /api/bits/obj/enum.SeekMode.html
     */
    pub fn set_end_to_begin_pos(&self, pos: SeekMode) -> Result<usize> {
        self.kern_call_1(KernFnPath::Iterator(KrnIteratorFnId::SetEndToBeginPos),
                         pos.into())
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
     * [`KrnIterDirection`]: /api/bits/obj/enum.KrnIterDirection.html
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
