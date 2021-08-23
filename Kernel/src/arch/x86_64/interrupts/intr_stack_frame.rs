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
#[repr(packed)]
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
    m_align_pad: usize
}

impl IntrStackFrame /* Getters */ {
    pub fn is_from_user_space(&self) -> bool {
        self.m_intr_num == 0 || { self.m_rflags }.bit_at(9)
    }
}

impl Debug for IntrStackFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "IntrStackFrame {{")?;
        writeln!(f, "    m_r15:        {:#018x}", { self.m_r15 })?;
        writeln!(f, "    m_r14:        {:#018x}", { self.m_r14 })?;
        writeln!(f, "    m_r13:        {:#018x}", { self.m_r13 })?;
        writeln!(f, "    m_r12:        {:#018x}", { self.m_r12 })?;
        writeln!(f, "    m_r11:        {:#018x}", { self.m_r11 })?;
        writeln!(f, "    m_r10:        {:#018x}", { self.m_r10 })?;
        writeln!(f, "    m_r9:         {:#018x}", { self.m_r9 })?;
        writeln!(f, "    m_r8:         {:#018x}", { self.m_r8 })?;
        writeln!(f, "    m_rbp:        {:#018x}", { self.m_rbp })?;
        writeln!(f, "    m_rsi:        {:#018x}", { self.m_rsi })?;
        writeln!(f, "    m_rdi:        {:#018x}", { self.m_rdi })?;
        writeln!(f, "    m_rdx:        {:#018x}", { self.m_rdx })?;
        writeln!(f, "    m_rcx:        {:#018x}", { self.m_rcx })?;
        writeln!(f, "    m_rbx:        {:#018x}", { self.m_rbx })?;
        writeln!(f, "    m_rax:        {:#018x}", { self.m_rax })?;
        writeln!(f, "    m_intr_num:   {}", { self.m_intr_num })?;
        writeln!(f, "    m_error_code: {}", { self.m_error_code })?;
        writeln!(f, "    m_rip:        {:#018x}", { self.m_rip })?;
        writeln!(f, "    m_cs:         {:x}", { self.m_cs })?;
        writeln!(f, "    m_rflags:     {:b}", { self.m_rflags })?;
        writeln!(f, "    m_usr_sp:     {:#018x}", { self.m_usr_sp })?;
        writeln!(f, "    m_usr_ss:     {:x}", { self.m_usr_ss })?;
        writeln!(f, "    m_align_pad:  {:x}", { self.m_align_pad })?;
        writeln!(f, "}}")
    }
}
