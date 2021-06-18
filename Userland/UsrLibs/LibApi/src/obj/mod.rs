/*! Objects management */

use api_data::{
    obj::{
        modes::RecvMode,
        types::ObjType,
        uses::ObjUseBits
    },
    sys::{
        codes::KernObjectFnId,
        fn_path::KernFnPath,
        KernHandle
    },
    task::thread::RWatchThreadEntry
};
use bits::flags::BitFlags;

use crate::caller::{
    KernCaller,
    Result
};

pub type ObjUseFilters = BitFlags<usize, ObjUseBits>;

#[repr(transparent)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
pub struct ObjHandle {
    m_raw_handle: KernHandle
}

impl ObjHandle {
    fn send<T>(&self, recv_task: &T) -> Result<()> {
        self.inst_kern_call_1(KernFnPath::Object(KernObjectFnId::Send), 0).map(|_| ())
    }

    fn recv(&mut self, obj_type: ObjType, recv_mode: RecvMode) -> Result<()> {
        self.inst_kern_call_2(KernFnPath::Object(KernObjectFnId::Recv),
                              obj_type.into(),
                              recv_mode.into())
            .map(|obj_handle| {
                *self = Self::from(obj_handle);
                ()
            })
    }

    fn drop_name(&self) -> Result<()> {
        self.inst_kern_call_0(KernFnPath::Object(KernObjectFnId::DropName)).map(|_| ())
    }

    fn watch(&self,
             use_filter: ObjUseFilters,
             callback_fn: RWatchThreadEntry)
             -> Result<()> {
        Ok(())
    }
}

impl Clone for ObjHandle {
    /**
     * Increases the references count to the object referenced.
     *
     * The returned `ObjHandle` is a new user instance but reference the
     * same Kernel's object, so changes to any of the cloned instances
     * affect the same Kernel's object
     */
    fn clone(&self) -> Self {
        self.inst_kern_call_0(KernFnPath::Object(KernObjectFnId::AddRef))
            .map(|_| Self::from(self.m_raw_handle))
            .expect("Kernel failed to clone Object")
    }
}

impl Drop for ObjHandle {
    /**
     * Decreases by one the references count to the referenced Kernel's
     * object.
     *
     * The life of the objects varies by type:
     *
     * Permanent objects, like `File`s, `Dir`ectories, `Link`s and
     * `OsRawMutex`es, persists until they are explicitly destroyed with
     * `Object::drop_name()`.
     *
     * The other kind of objects, like `MMap`s and `IpcChan`nels, live
     * until there is a reference to them. When the references reaches the 0
     * they are definitely destroyed
     */
    fn drop(&mut self) {
        self.inst_kern_call_0(KernFnPath::Object(KernObjectFnId::Drop))
            .expect("Kernel failed to drop Object");
    }
}

impl From<KernHandle> for ObjHandle {
    fn from(raw_handle: KernHandle) -> Self {
        Self { m_raw_handle: raw_handle }
    }
}

impl From<usize> for ObjHandle {
    fn from(raw_handle: usize) -> Self {
        Self::from(raw_handle as KernHandle)
    }
}

impl KernCaller for ObjHandle {
    fn raw_handle(&self) -> KernHandle {
        self.m_raw_handle
    }
}

/**
 * Common interface implemented by all the `ObjHandle` based objects.
 *
 * It mainly exposes the private methods of the `ObjHandle` for safe calling
 * and provides convenient methods to easily perform works that normally
 * implies more than one call
 */
pub trait Object: From<ObjHandle> + Default + Clone + KernCaller {
    /**
     * The value of the `ObjType` that matches the implementation
     */
    const TYPE: ObjType;

    /**
     * Returns the immutable reference to the underling `ObjHandle` instance
     */
    fn obj_handle(&self) -> &ObjHandle;

    /**
     * Returns the mutable reference to the underling `ObjHandle` instance
     */
    fn obj_handle_mut(&mut self) -> &mut ObjHandle;
}
