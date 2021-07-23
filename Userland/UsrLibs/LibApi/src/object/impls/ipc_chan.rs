/*! Open IPC channel `Object` */

use core::num::NonZeroUsize;

use api_data::{
    object::{
        modes::ObjRecvMode,
        types::ObjType
    },
    sys::{
        codes::KernIpcChanFnId,
        fn_path::KernFnPath
    }
};

use crate::{
    kern_handle::Result,
    object::{
        MTAnonymousObject,
        MTSizeableDataObject,
        ObjHandle,
        TObject,
        TUserCreatableObject
    }
};

/**
 * Inter-process-communication channel.
 *
 * Acts like a message queue of arbitrary sized messages which could be
 * exchanged with two or more counter-sides
 */
#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct IpcChan {
    m_obj_handle: ObjHandle
}

impl IpcChan /* Methods */ {
    /**
     * Appends the given `payload` to the message-queue of this `IpcChan`.
     *
     * `tx_id` is the transaction-id, a way to direct a particular message
     * to a receiver which provides to `recv_msg()` the same identifier.
     *
     * When sending messages the transaction-id can be re-used or given a
     * new-one, the kernel will create a new hole for the messages with the
     * same `tx_id`
     */
    pub fn send_msg<T>(&self, payload: &T, tx_id: Option<NonZeroUsize>) -> Result<usize>
        where T: AsRef<[u8]> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_3(KernFnPath::IpcChan(KernIpcChanFnId::Send),
                              payload.as_ref().as_ptr() as usize,
                              payload.as_ref().len(),
                              &tx_id as *const _ as usize)
    }

    /**
     * Pops the first available message according to the `ObjRecvMode`.
     *
     * The kernel fills `payload` with the payload sent with `send_msg()`
     * only if the two length are exactly the same.
     *
     * Can be received a message with a particular transaction-id
     */
    pub fn recv_msg<T>(&self,
                       recv_mode: ObjRecvMode,
                       payload: &mut T,
                       tx_id: Option<NonZeroUsize>)
                       -> Result<usize>
        where T: AsMut<[u8]> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_4(KernFnPath::IpcChan(KernIpcChanFnId::Recv),
                              recv_mode.into(),
                              payload.as_mut().as_mut_ptr() as usize,
                              payload.as_mut().len(),
                              &tx_id as *const _ as usize)
    }
}

impl From<ObjHandle> for IpcChan {
    fn from(obj_handle: ObjHandle) -> Self {
        Self { m_obj_handle: obj_handle }
    }
}

impl TObject for IpcChan {
    const TYPE: ObjType = ObjType::IpcChan;

    fn obj_handle(&self) -> &ObjHandle {
        &self.m_obj_handle
    }

    fn obj_handle_mut(&mut self) -> &mut ObjHandle {
        &mut self.m_obj_handle
    }
}

impl TUserCreatableObject for IpcChan {
    /* No methods to implement */
}

impl MTSizeableDataObject for IpcChan {
    /* No methods to implement */
}

impl MTAnonymousObject for IpcChan {
    /* No methods to implement */
}
