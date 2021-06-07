/*! Open Device `Object` */

use core::ptr::NonNull;

use os::sysc::{
    codes::KernDeviceFnId,
    fn_path::KernFnPath
};

use crate::{
    bits::obj::{
        modes::SeekMode,
        types::ObjType
    },
    caller::{
        KernCaller,
        Result
    },
    objs::{
        impls::mmap::MMap,
        object::{
            ObjId,
            Object
        }
    }
};

/**
 * Reference to an open device on the VFS.
 *
 * VFS representation of a driver into the Kernel space, it exposes the
 * system calls to communicate with the driver according to the expected
 * protocol.
 *
 * A `Device` could be a character device (like a terminal, a network device
 * or a graphic framebuffer) or a block device, like a disk.
 *
 * For character devices, `Device::read()`/`Device::write()` accepts
 * arbitrary sized buffers, but for block devices the buffers must be
 * multiple of the block_size
 */
#[repr(transparent)]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Device {
    m_handle: ObjId
}

impl Device {
    /**
     * Requests to the underling driver to fill `buf` for at max
     * `buf.len()` bytes reading them from the source of the driver (like
     * the disk, the network or the framebuffer).
     *
     * For block devices `buf.len()` must be a multiple of the block_size
     */
    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        self.kern_call_2(KernFnPath::Device(KernDeviceFnId::Read),
                         buf.as_mut_ptr() as usize,
                         buf.len())
    }

    /**
     * Request to the underling driver to accept the given `buf` and write
     * his content into the source of the driver (like the disk, the network
     * or the framebuffer).
     *
     * For block devices `buf.len()` must be a multiple of the block_size
     */
    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        self.kern_call_2(KernFnPath::Device(KernDeviceFnId::Write),
                         buf.as_ptr() as usize,
                         buf.len())
    }

    /**  
     * Request to the underling driver to execute his `map_to_memory()`
     * implementation (which could be not available, i.e not supported).
     */
    pub fn map_to_memory(&self,
                         addr: Option<NonNull<u8>>,
                         from: u64,
                         size: u64)
                         -> Result<MMap> {
        self.kern_call_3(KernFnPath::Device(KernDeviceFnId::MapToMem),
                         addr.map(|nn_ptr| nn_ptr.as_ptr() as usize).unwrap_or_default(),
                         from as usize,
                         size as usize)
            .map(|obj_id| MMap::from(ObjId::from(obj_id)))
    }

    /**
     * TODO enum with values and data
     */
    pub fn io_setup(&self,
                    cmd_value: usize,
                    arg_ptr: Option<NonNull<()>>)
                    -> Result<usize> {
        self.kern_call_2(KernFnPath::Device(KernDeviceFnId::IOSetup),
                         cmd_value,
                         arg_ptr.map(|nn_ptr| nn_ptr.as_ptr() as usize)
                                .unwrap_or_default())
    }

    /**
     * According to the `SeekMode` given, it updates the read/write
     * position.
     *
     * The offsets are expected in block units, i.e for char devices the
     * block unit is 1, for block devices use the block_size
     */
    pub fn set_pos(&self, mode: SeekMode) -> Result<u64> {
        self.kern_call_2(KernFnPath::Device(KernDeviceFnId::SetPos),
                         mode.mode(),
                         mode.off().unwrap_or(0))
            .map(|off| off as u64)
    }

    /**
     * Returns the current cursor position
     */
    pub fn pos(&self) -> u64 {
        self.set_pos(SeekMode::Absolute(0)).unwrap_or(0)
    }

    /**
     * Returns whether this `Device` is a char device
     */
    pub fn is_char_device(&self) -> Result<bool> {
        self.info().map(|obj_info| obj_info.mem_info().block_size() == 1)
    }

    /**
     * Returns whether this `Device` is a block device
     */
    pub fn is_block_device(&self) -> Result<bool> {
        self.info().map(|obj_info| obj_info.mem_info().block_size().is_power_of_two())
    }
}

impl Object for Device {
    const TYPE: ObjType = ObjType::Device;

    fn obj_handle(&self) -> &ObjId {
        &self.m_handle
    }

    fn obj_handle_mut(&mut self) -> &mut ObjId {
        &mut self.m_handle
    }
}

impl From<ObjId> for Device {
    fn from(id: ObjId) -> Self {
        Self { m_handle: id }
    }
}

impl KernCaller for Device {
    fn caller_handle_bits(&self) -> u32 {
        self.obj_handle().caller_handle_bits()
    }
}
