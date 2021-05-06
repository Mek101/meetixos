/*! Open kernel iterator `Object` */

use os::sysc::{
    codes::KrnIteratorFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::obj::{
        modes::{
            KrnIterDirection,
            SeekMode
        },
        types::ObjType
    },
    caller::{
        KernCaller,
        Result
    },
    objs::object::{
        ObjId,
        Object
    }
};

/**
 * Reference to a double linked kernel's iteration pool.
 *
 * The kernel iterator could iterate different object types, from object
 * handles to more complex objects (like the `DirEntry`).
 *
 * This type of kernel's object is not creatable by the userspace, but
 * only by the kernel as response of system calls which return more
 * than one result (like the `Task::find()`).
 *
 * As consequence of the fact written above the `KrnIterator` is not
 * either used directly, but always wrapped into another structure that
 * ensures type validity
 */
#[repr(transparent)]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct KrnIterator {
    m_handle: ObjId
}

impl KrnIterator {
    /**
     * According to the `SeekMode` given, it updates the begin to end
     * position
     */
    pub fn set_begin_to_end_pos(&self, pos: SeekMode) -> Result<usize> {
        self.kern_call_1(KernFnPath::Iterator(KrnIteratorFnId::SetBeginToEndPos),
                         pos.mode())
    }

    /**
     * According to the `SeekMode` given, it updates the end to begin
     * position
     *
     * [`SeekMode`]: crate::bits::obj::modes::SeekMode
     */
    pub fn set_end_to_begin_pos(&self, pos: SeekMode) -> Result<usize> {
        self.kern_call_1(KernFnPath::Iterator(KrnIteratorFnId::SetEndToBeginPos),
                         pos.mode())
    }

    /**
     * Returns the next element into the pool reading from the begin to the
     * end
     */
    pub(crate) fn find_next<T>(&self) -> Result<Option<T>>
        where T: Default {
        self.next_res(KrnIterDirection::BeginToEnd)
    }

    /**
     * Returns the next element into the pool reading from the end to the
     * begin
     */
    pub(crate) fn find_next_back<T>(&self) -> Result<Option<T>>
        where T: Default {
        self.next_res(KrnIterDirection::EndToBegin)
    }

    /**
     * Returns the next element into the pool reading the next element
     * according to the given `KrnIterDirection`
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
    const TYPE: ObjType = ObjType::KrnIterator;

    fn obj_handle(&self) -> &ObjId {
        &self.m_handle
    }

    fn obj_handle_mut(&mut self) -> &mut ObjId {
        &mut self.m_handle
    }
}

impl From<ObjId> for KrnIterator {
    fn from(id: ObjId) -> Self {
        Self { m_handle: id }
    }
}

impl KernCaller for KrnIterator {
    fn caller_handle_bits(&self) -> u32 {
        self.obj_handle().caller_handle_bits()
    }
}
