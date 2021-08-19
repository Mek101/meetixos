/*! x86_64 interrupt stack frame */

use bits::bit_fields::TBitFields;

/**
 * x86_64 interrupt stack frame
 */
#[repr(C)]
#[derive(Debug)]
pub struct IntrStackFrame {
    /* general purpose registers */
    m_r15: usize,
    m_r14: usize,
    m_r13: usize,
    m_r12: usize,
    m_r11: usize,
    m_r10: usize,
    m_r9: usize,
    m_r8: usize,
    m_rbp: usize,
    m_rsi: usize,
    m_rdi: usize,
    m_rdx: usize,
    m_rcx: usize,
    m_rbx: usize,
    m_rax: usize,
    m_intr_num: usize,
    m_error_code: usize,

    /* fields automatically pushed by the CPU */
    m_rip: usize,
    m_cs: usize,
    m_rflags: usize,
    m_usr_sp: usize,
    m_usr_ss: usize,
    _unused_alignment: usize
}

impl IntrStackFrame /* Getters */ {
    pub fn is_from_user_space(&self) -> bool {
        self.m_intr_num == 0 || self.m_rflags.bit_at(9)
    }
}
