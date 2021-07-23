/*! `Device` specific data structures */

use core::convert::TryFrom;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use bits::bit_fields::TBitFields;

/**
 * `Device` identifier
 */
#[derive(Debug)]
#[derive(Default)]
#[derive(Copy, Clone)]
pub struct DeviceId {
    m_device_type: DeviceIdType,
    m_device_class: DeviceIdClass,
    m_serial_value: u32
}

impl DeviceId /* Constructors */ {
    /**
     * Constructs a `DeviceId` from the given parameters
     */
    pub fn new(device_type: DeviceIdType,
               device_class: DeviceIdClass,
               serial_value: u32)
               -> Self {
        Self { m_device_type: device_type,
               m_device_class: device_class,
               m_serial_value: serial_value }
    }
}

impl DeviceId /* Getters */ {
    /**
     * Returns the `DeviceIdType`
     */
    pub fn device_type(&self) -> DeviceIdType {
        self.m_device_type
    }

    /**
     * Returns the `DeviceIdClass`
     */
    pub fn device_class(&self) -> DeviceIdClass {
        self.m_device_class
    }

    /**
     * Returns the serial registration value
     */
    pub fn serial_value(&self) -> u32 {
        self.m_serial_value
    }
}

impl TryFrom<usize> for DeviceId {
    type Error = ();

    fn try_from(raw_device_id: usize) -> Result<Self, ()> {
        let device_type =
            DeviceIdType::try_from(raw_device_id.bits_at(40..48) as u8).map_err(|_| ())?;
        let device_class =
            DeviceIdClass::try_from(raw_device_id.bits_at(32..40) as u8).map_err(|_| ())?;

        Ok(Self { m_device_type: device_type,
                  m_device_class: device_class,
                  m_serial_value: raw_device_id.bits_at(0..32) as u32 })
    }
}

impl Into<usize> for DeviceId {
    fn into(self) -> usize {
        (Into::<u8>::into(self.device_type()) as usize) << 40
        | (Into::<u8>::into(self.device_class()) as usize) << 32
        | (self.serial_value() as usize)
    }
}

/**
 * Lists the supported `Device` sub-types
 */
#[repr(u8)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(Hash)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum DeviceIdType {
    /**
     * Identifies a `Device` which reads & writes data in contiguous blocks
     * of the same size
     */
    Block,

    /**
     * Identifies a `Device` which read & writes data in random order and
     * with heterogeneous blocks
     */
    Character
}

impl DeviceIdType /* Getters */ {
    /**
     * Returns whether this is a `Block` device
     */
    pub fn is_block_device(&self) -> bool {
        matches!(*self, Self::Block)
    }

    /**
     * Returns whether this is a `Character` device
     */
    pub fn is_char_device(&self) -> bool {
        matches!(*self, Self::Character)
    }
}

impl Default for DeviceIdType {
    fn default() -> Self {
        Self::Block
    }
}

/**
 * Lists the supported `Device` classes
 */
#[repr(u8)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(Hash)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum DeviceIdClass {
    /**
     * `Device` which is able to store data into physical storage devices,
     * like a disk, an SSD or an SD-Card
     */
    Storage,

    /**
     * `Device` which manages in-memory storages, like temporary filesystem
     * device support or userland `MMap`
     */
    Memory,

    /**
     * `Device` which manages network communication via sockets
     */
    Network,

    /**
     * `Device` which manages inter-process communication network via
     * `IpcChan`
     */
    Ipc,

    /**
     * `Device` which manages a screen framebuffer
     */
    Framebuffer,

    /**
     * `Device` which produces random numbers
     */
    Random,

    /**
     * `Device` which manages I/O with serial port
     */
    Uart,

    /**
     * `Device` which manages terminal I/O    
     */
    Terminal
}

impl DeviceIdClass /* Getters */ {
    /**
     * Returns whether this is a `Storage` device
     */
    pub fn is_storage(&self) -> bool {
        matches!(*self, Self::Storage)
    }

    /**
     * Returns whether this is a `Memory` device
     */
    pub fn is_memory(&self) -> bool {
        matches!(*self, Self::Memory)
    }

    /**
     * Returns whether this is a `Network` device
     */
    pub fn is_network(&self) -> bool {
        matches!(*self, Self::Network)
    }

    /**
     * Returns whether this is a `Ipc` device
     */
    pub fn is_ipc(&self) -> bool {
        matches!(*self, Self::Ipc)
    }

    /**
     * Returns whether this is a `Framebuffer` device
     */
    pub fn is_framebuffer(&self) -> bool {
        matches!(*self, Self::Framebuffer)
    }

    /**
     * Returns whether this is a `Random` device
     */
    pub fn is_random(&self) -> bool {
        matches!(*self, Self::Random)
    }

    /**
     * Returns whether this is a `Uart` device
     */
    pub fn is_uart(&self) -> bool {
        matches!(*self, Self::Uart)
    }

    /**
     * Returns whether this is a `Terminal` device
     */
    pub fn is_terminal(&self) -> bool {
        matches!(*self, Self::Terminal)
    }
}

impl Default for DeviceIdClass {
    fn default() -> Self {
        Self::Storage
    }
}
