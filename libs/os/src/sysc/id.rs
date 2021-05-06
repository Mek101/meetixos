/*! Kernel function call identifier */

use core::{
    convert::TryFrom,
    fmt,
    ops::Range
};

use bit_field::BitField;

use crate::sysc::fn_path::KernFnPath;

/**
 * Kernel call identifier wrapper
 *
 * Since the OS is compatible only with 64bit capable architectures a single
 * register can be quietly used to store more than one think, it is enough
 * play with bits.
 *
 * In particular the syscall-id register stores the `KernFnPath`, which is
 * 32bit wide, and the `api::caller::KernCaller::caller_handle_bits`, which
 * fills the remaining upper 32bits
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct SysCallId {
    m_raw: usize
}

impl SysCallId {
    const CALL_CLASS_BITS: Range<usize> = 0..16;
    const CALL_CODE_BITS: Range<usize> = 16..32;
    const CUSTOM_DATA_BITS: Range<usize> = 32..64;

    /**
     * Constructs a `SysCallId` composing the given raw parts
     */
    pub fn new(fn_path: KernFnPath, custom_data: u32) -> Self {
        let mut id = Self::default();
        id.m_raw.set_bits(Self::CALL_CLASS_BITS, fn_path.raw_fn_class() as usize);
        id.m_raw.set_bits(Self::CALL_CODE_BITS, fn_path.raw_fn_id() as usize);
        id.m_raw.set_bits(Self::CUSTOM_DATA_BITS, custom_data as usize);
        id
    }

    /**
     * Returns the `KernFnPath`
     */
    pub fn fn_path(&self) -> KernFnPath {
        KernFnPath::try_from((self.m_raw.get_bits(Self::CALL_CLASS_BITS),
                              self.m_raw.get_bits(Self::CALL_CODE_BITS))).unwrap()
    }

    /**
     * Returns the custom data bits
     */
    pub fn custom_data(&self) -> u32 {
        self.m_raw.get_bits(Self::CUSTOM_DATA_BITS) as u32
    }
}

impl From<usize> for SysCallId {
    fn from(raw_id: usize) -> Self {
        Self { m_raw: raw_id }
    }
}

impl Into<usize> for SysCallId {
    fn into(self) -> usize {
        self.m_raw
    }
}

impl fmt::Display for SysCallId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.fn_path(), self.custom_data())
    }
}
