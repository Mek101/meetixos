/*! Open IPC channel `Object` */

use core::num::NonZeroUsize;

use api_data::{
    obj::{
        modes::ObjRecvMode,
        types::ObjType
    },
    sys::{
        codes::KernIpcChanFnId,
        fn_path::KernFnPath
    }
};

use crate::{
    handle::Result,
    obj::{
        ObjHandle,
        Object,
        SizeableDataObject,
        UserCreatableObject
    }
};

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

impl IpcChan {
    pub fn send_msg<T>(&self, payload: &T, tx_id: Option<NonZeroUsize>) -> Result<usize>
        where T: AsRef<[u8]> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_3(KernFnPath::IpcChan(KernIpcChanFnId::Send),
                              payload.as_ref().as_ptr() as usize,
                              payload.as_ref().len(),
                              tx_id.map(|nz_tx_id| nz_tx_id.get()).unwrap_or_default())
    }

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
                              tx_id.map(|nz_tx_id| nz_tx_id.get()).unwrap_or_default())
    }
}

impl From<ObjHandle> for IpcChan {
    fn from(obj_handle: ObjHandle) -> Self {
        Self { m_obj_handle: obj_handle }
    }
}

impl Object for IpcChan {
    const TYPE: ObjType = ObjType::IpcChan;

    fn obj_handle(&self) -> &ObjHandle {
        &self.m_obj_handle
    }

    fn obj_handle_mut(&mut self) -> &mut ObjHandle {
        &mut self.m_obj_handle
    }
}

impl UserCreatableObject for IpcChan {
    /* No methods to implement */
}

impl SizeableDataObject for IpcChan {
    /* No methods to implement */
}
