/*! x86_64 interrupt stack frame */

use core::{
    fmt,
    fmt::Debug
};

use bits::bit_fields::TBitFields;

/**
 * x86_64 interrupt stack frame
 */
#[repr(C)]
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

impl Debug for IntrStackFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "IntrStackFrame {{ \n\tm_r15: {:#018x},\n\tm_r14: {:#018x}, \n\tm_r13: \
                {:#018x}, \n\tm_r12: {:#018x}, \n\tm_r11: {:#018x}, \n\tm_r10: \
                {:#018x}, \n\tm_r9: {:#018x}, \n\tm_r8: {:#018x}, \n\tm_rbp: {:#018x}, \
                \n\tm_rsi: {:#018x}, \n\tm_rdi: {:#018x}, \n\tm_rdx: {:#018x}, \
                \n\tm_rcx: {:#018x}, \n\tm_rbx: {:#018x}, \n\tm_rax: {:#018x}, \
                \n\tm_intr_num: {:#018x}, \n\tm_error_code: {}, \n\tm_rip: {:#018x}, \
                \n\tm_cs: {:#018x}, \n\tm_rflags: {:b}, \n\tm_usr_sp: {:#018x}, \
                \n\tm_usr_ss: {:#018x}, \n\t_unused_alignment: {:#018x} }}",
               self.m_r15,
               self.m_r14,
               self.m_r13,
               self.m_r12,
               self.m_r11,
               self.m_r10,
               self.m_r9,
               self.m_r8,
               self.m_rbp,
               self.m_rsi,
               self.m_rdi,
               self.m_rdx,
               self.m_rcx,
               self.m_rbx,
               self.m_rax,
               self.m_intr_num,
               self.m_error_code,
               self.m_rip,
               self.m_cs,
               self.m_rflags,
               self.m_usr_sp,
               self.m_usr_ss,
               self._unused_alignment)
    }
}
