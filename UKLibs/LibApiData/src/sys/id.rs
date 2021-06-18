/*! Kernel function call identifier */

use core::{
    convert::TryFrom,
    fmt,
    ops::Range
};

use bits::fields::BitFields;

use crate::sys::fn_path::KernFnPath;

/**
 * Kernel call identifier wrapper
 *
 * Since the OS is compatible only with 64bit capable architectures a single
 * register can be quietly used to store more than one think, it is enough
 * play with bits.
 *
 * In particular the sys-id register stores the `KernFnPath`, which is
 * 32bit wide, and the `LibApi::caller::KernCaller::caller_handle_bits`,
 * which fills the remaining upper 32bits
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct SysCallId {
    m_raw_id: usize
}

impl SysCallId {
    const CALL_CLASS_BITS: Range<usize> = 0..16;
    const CALL_CODE_BITS: Range<usize> = 16..32;
    const CUSTOM_DATA_BITS: Range<usize> = 32..64;

    /**
     * Constructs a `SysCallId` composing the given raw parts
     */
    pub fn new(fn_path: KernFnPath, custom_data: u32) -> Self {
        let mut new_id = Self::default();
        new_id.m_raw_id.set_bits(Self::CALL_CLASS_BITS, fn_path.raw_fn_class() as usize);
        new_id.m_raw_id.set_bits(Self::CALL_CODE_BITS, fn_path.raw_fn_id() as usize);
        new_id.m_raw_id.set_bits(Self::CUSTOM_DATA_BITS, custom_data as usize);
        new_id
    }

    /**
     * Returns the `KernFnPath`
     */
    pub fn fn_path(&self) -> KernFnPath {
        KernFnPath::try_from((self.m_raw_id.bits_at(Self::CALL_CLASS_BITS),
                              self.m_raw_id.bits_at(Self::CALL_CODE_BITS)))
                   .expect("Malformed KernFnPath instance in SysCallId wrapper")
    }

    /**
     * Returns the custom data bits
     */
    pub fn custom_data(&self) -> u32 {
        self.m_raw_id.bits_at(Self::CUSTOM_DATA_BITS) as u32
    }
}

impl From<usize> for SysCallId {
    fn from(raw_id: usize) -> Self {
        Self { m_raw_id: raw_id }
    }
}

impl Into<usize> for SysCallId {
    fn into(self) -> usize {
        self.m_raw_id
    }
}

impl fmt::Display for SysCallId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SysCallId({}:{})", self.fn_path(), self.custom_data())
    }
}
