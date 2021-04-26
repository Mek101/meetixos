/*! # Call Identifier
 *
 * Defines a simple structure that encapsulates the data to call a kernel
 * routine
 */

use core::{
    convert::TryFrom,
    fmt,
    fmt::{Display, Formatter},
    ops::Range
};

use bit_field::BitField;

use crate::sysc::fn_path::KernFnPath;

/** # System Call Identifier
 *
 * Defines a wrapper for the raw system call identifier.
 *
 * Due to the low quantity of system calls available and because the system
 * is compilable only for 64bit architectures waste more than 48 bits of the
 * identifier and use additional registers to store the [call class] and the
 * [custom data] (used to perform instance call) is really stupid.
 *
 * So, according to rules defined by internal constants, the 64bits of the
 * identifier are all used for the previously said data
 *
 * [call class]: /os/sysc/classes/index.html
 * [custom data]:
 * /api/caller/trait.KernCaller.html#method.caller_handle_bits
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct SysCallId(usize);

impl SysCallId {
    const CALL_CLASS_BITS: Range<usize> = 0..16;
    const CALL_CODE_BITS: Range<usize> = 16..32;
    const CUSTOM_DATA_BITS: Range<usize> = 32..64;

    /** # Constructs a `SysCallId`
     *
     * Composes the identifier using the single raw parts
     */
    pub fn new(fn_path: KernFnPath, custom_data: u32) -> Self {
        let mut inst = Self::default();
        inst.0.set_bits(Self::CALL_CLASS_BITS, fn_path.raw_fn_class().into());
        inst.0.set_bits(Self::CALL_CODE_BITS, fn_path.raw_fn_id().into());
        inst.0.set_bits(Self::CUSTOM_DATA_BITS, custom_data as usize);
        inst
    }

    /** Returns the [`KernFnPath`]
     *
     * [`KernFnPath`]: /os/sysc/fn_path/enum.KernFnPath.html
     */
    pub fn fn_path(&self) -> KernFnPath {
        KernFnPath::try_from((self.0.get_bits(Self::CALL_CLASS_BITS),
                              self.0.get_bits(Self::CALL_CODE_BITS))).unwrap()
    }

    /** Returns the [custom data] bits
     *
     * [custom data]:
     * /api/caller/trait.KernCaller.html#method.caller_handle_bits
     */
    pub fn custom_data(&self) -> u32 {
        self.0.get_bits(Self::CUSTOM_DATA_BITS) as u32
    }
}

impl From<usize> for SysCallId {
    /** Performs the conversion.
     */
    fn from(raw_id: usize) -> Self {
        SysCallId(raw_id)
    }
}

impl Into<usize> for SysCallId {
    /** Performs the conversion.
     */
    fn into(self) -> usize {
        self.0
    }
}

impl Display for SysCallId {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.fn_path(), self.custom_data())
    }
}
