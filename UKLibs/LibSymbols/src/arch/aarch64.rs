/*! aarch64 code implementation */

use crate::stack_back_trace::HwTracerHelperBase;

/**
 * aarch64 `HwTracerHelperBase` implementation
 */
pub struct HwTracerHelper;

impl HwTracerHelperBase for HwTracerHelper {
    const PREV_FRAME_PTR_OFFSET: isize = 0;
    const PREV_RETURN_PTR_OFFSET: isize = 1;

    #[inline(always)]
    fn read_frame_ptr() -> usize {
        let ptr_value: usize;
        unsafe {
            asm!("mov {}, x29", out(reg) ptr_value);
        }
        ptr_value
    }

    #[inline(always)]
    fn read_return_ptr() -> usize {
        let ptr_value: usize;
        unsafe {
            asm!("mov {}, x30", out(reg) ptr_value);
        }
        ptr_value
    }
}
