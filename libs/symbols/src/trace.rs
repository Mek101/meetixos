/*! Stack tracer from the current calling point */

use core::fmt;

use crate::{
    arch::HwTracerHelper,
    list::CodeSymbolsList
};

/**
 * `Display` implementation for stack backtrace.
 *
 * Executes the backtrace from the point where is constructed with
 * `StackBackTrace::new()`
 */
pub struct StackBackTrace<'a> {
    m_return_ptr: usize,
    m_frame_ptr: usize,
    m_symbols_list: &'a CodeSymbolsList,
    m_text_begin: usize,
    m_text_end: usize
}

impl<'a> StackBackTrace<'a> {
    /**
     * Constructs a `StackBackTrace` reading the stack pointer
     */
    #[inline(always)]
    pub fn new(symbols_list: &'a CodeSymbolsList,
               text_begin: usize,
               text_end: usize)
               -> Self {
        Self { m_return_ptr: HwTracerHelper::read_return_ptr(),
               m_frame_ptr: HwTracerHelper::read_frame_ptr(),
               m_symbols_list: symbols_list,
               m_text_begin: text_begin,
               m_text_end: text_end }
    }
}

impl<'a> fmt::Display for StackBackTrace<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current_ret_ptr = self.m_return_ptr;
        let mut current_frm_ptr = self.m_frame_ptr;

        /* be sure we are into the text */
        while current_ret_ptr >= self.m_text_begin
              && current_ret_ptr <= self.m_text_end
              && current_frm_ptr != 0
        {
            /* obtain the symbol for the current pointer and display it */
            if let Some(code_sym) = self.m_symbols_list.symbol_at(current_ret_ptr) {
                writeln!(f, "{:#018x} - {}", current_ret_ptr, code_sym)?;
            } else {
                writeln!(f, "{:#018x} - (???)", current_ret_ptr)?;
            }

            /* step to the previous frame pointer */
            current_frm_ptr = unsafe {
                *(current_frm_ptr as *const usize).offset(HwTracerHelper::PREV_FRAME_PTR_OFFSET)
            };
            current_ret_ptr = unsafe {
                *(current_frm_ptr as *const usize).offset(HwTracerHelper::PREV_RETURN_PTR_OFFSET)
            };
        }

        Ok(())
    }
}

/**
 * Interface on which `StackBackTrace` relies to obtain hardware dependent
 * information
 */
pub(crate) trait HwTracerHelperBase {
    /**
     * Offset to obtain the previous frame pointer
     */
    const PREV_FRAME_PTR_OFFSET: isize;

    /**
     * Offset to obtain the previous return address pointer
     */
    const PREV_RETURN_PTR_OFFSET: isize;

    /**
     * Returns the current frame pointer value
     */
    fn read_frame_ptr() -> usize;

    /**
     * Returns the current return address
     */
    fn read_return_ptr() -> usize;
}
