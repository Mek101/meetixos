/*! Open Device `Object` */

use core::ptr::NonNull;

use api_data::{
    object::{
        modes::SeekMode,
        types::ObjType
    },
    sys::{
        codes::KernDeviceFnId,
        fn_path::KernFnPath,
        AsSysCallPtr
    }
};

use crate::{
    kern_handle::Result,
    object::{
        impls::mmap::MMap,
        ObjHandle,
        Object
    }
};

/**
 * File-like device driver.
 *
 * Reading/writing with this kind of `Object` directly communicates (without
 * intermediate kernel buffering) with the underling kernel driver
 */
#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct Device {
    m_obj_handle: ObjHandle
}

impl Device {
    /**
     * Requests to the underling driver to fill `buf` for at max
     * `buf.len()` bytes reading them from the source of the driver (like
     * the disk, the network or the framebuffer).
     *
     * For block devices `buf.len()` must be a multiple of the block_size.
     *
     * Returns the filled sub-slice of the given buffer
     */
    pub fn read<'a>(&self, buf: &'a mut [u8]) -> Result<&'a [u8]> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_2(KernFnPath::Device(KernDeviceFnId::Read),
                              buf.as_mut_ptr() as usize,
                              buf.len())
            .map(move |read_bytes| &buf[..read_bytes])
    }

    /**
     * Request to the underling driver to accept the given `buf` and write
     * his content into the source of the driver (like the disk, the network
     * or the framebuffer).
     *
     * For block devices `buf.len()` must be a multiple of the block_size.
     *
     * Returns the unwritten sub-slice of the given buffer
     */
    pub fn write<'a>(&self, buf: &'a [u8]) -> Result<&'a [u8]> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_2(KernFnPath::Device(KernDeviceFnId::Read),
                              buf.as_ptr() as usize,
                              buf.len())
            .map(|written_bytes| &buf[written_bytes..])
    }

    /**  
     * Request to the underling driver to execute his `map_to_memory()`
     * implementation (which could be not available, i.e not supported).
     */
    pub fn map_to_memory(&self,
                         map_addr: Option<NonNull<()>>,
                         from_off: usize,
                         mmap_size: usize)
                         -> Result<MMap> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_3(KernFnPath::Device(KernDeviceFnId::MapToMem),
                              &map_addr as *const _ as usize,
                              from_off,
                              mmap_size)
            .map(|raw_obj_handle| MMap::from(ObjHandle::from_raw(raw_obj_handle)))
    }

    /**
     * TODO enum with values and data
     */
    pub fn io_setup(&self,
                    cmd_value: usize,
                    arg_ptr: Option<NonNull<()>>)
                    -> Result<usize> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_2(KernFnPath::Device(KernDeviceFnId::IOSetup),
                              cmd_value,
                              &arg_ptr as *const _ as usize)
    }

    /**
     * Updates the read/write position according to the `SeekMode` given
     *
     * The offsets are expected in block units, i.e for char devices the
     * block unit is 1, for block devices use the `block_size`
     */
    pub fn set_pos(&self, mode: SeekMode) -> Result<usize> {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_1(KernFnPath::Device(KernDeviceFnId::SetPos),
                              mode.as_syscall_ptr())
    }

    /**
     * Returns the current cursor position
     */
    pub fn pos(&self) -> Result<usize> {
        self.set_pos(SeekMode::Absolute(0))
    }

    /**
     * Returns whether this `Device` is a char device
     */
    pub fn is_char_device(&self) -> Result<bool> {
        self.obj_handle().info().map(|raw_obj_info| {
                                    raw_obj_info.device_id()
                                                .device_type()
                                                .is_char_device()
                                })
    }

    /**
     * Returns whether this `Device` is a block device
     */
    pub fn is_block_device(&self) -> Result<bool> {
        self.obj_handle().info().map(|raw_obj_info| {
                                    raw_obj_info.device_id()
                                                .device_type()
                                                .is_block_device()
                                })
    }
}

impl From<ObjHandle> for Device {
    fn from(obj_handle: ObjHandle) -> Self {
        Self { m_obj_handle: obj_handle }
    }
}

impl Object for Device {
    const TYPE: ObjType = ObjType::Device;

    fn obj_handle(&self) -> &ObjHandle {
        &self.m_obj_handle
    }

    fn obj_handle_mut(&mut self) -> &mut ObjHandle {
        &mut self.m_obj_handle
    }
}
