/*! riscv code implementation */

use crate::trace::HwTracerHelperBase;

/**
 * riscv `HwTracerHelperBase` implementation
 */
pub struct HwTracerHelper;

impl HwTracerHelperBase for HwTracerHelper {
    const PREV_FRAME_PTR_OFFSET: isize = -2;
    const PREV_RETURN_PTR_OFFSET: isize = -1;

    #[inline(always)]
    fn read_frame_ptr() -> usize {
        let ptr_value: usize;
        unsafe {
            asm!("mov {}, s0", out(reg) ptr_value);
        }
        ptr_value
    }

    #[inline(always)]
    fn read_return_ptr() -> usize {
        let ptr_value: usize;
        unsafe {
            asm!("mov {}, ra", out(reg) ptr_value);
        }
        ptr_value
    }
}
