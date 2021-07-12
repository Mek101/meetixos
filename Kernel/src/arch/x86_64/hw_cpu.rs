/*! x86_64 CPU management implementation */

use crate::{
    addr::{
        virt_addr::VirtAddr,
        Address
    },
    arch::x86_64::{
        gdt::{
            GlobalDescTable,
            Segment,
            SegmentSelector
        },
        tss::TaskStateSegment
    },
    cpu::{
        CpuId,
        HwCpuBase
    }
};

const C_DOUBLE_FAULT_STACK: usize = 4096;
const C_DOUBLE_FAULT_STACK_INDEX: usize = 0;

/**
 * x86_64 `HwCpuBase` implementation
 */
pub struct HwCpu {
    m_id: CpuId,
    m_is_ap: bool,
    m_gdt: GlobalDescTable,
    m_tss: TaskStateSegment,
    m_double_fault_stack: [u8; C_DOUBLE_FAULT_STACK]
}

impl HwCpu /* Privates */ {
    /**
     * Reloads the code-segment register with the given kernel-code selector
     */
    fn reload_code_segment_register(&self, kern_code_segment_selector: SegmentSelector) {
        unsafe {
            asm!("push      {seg_selector}",
                 "lea       {tmp}, [1f + rip]",
                 "push      {tmp}",
                 "retfq",
                 "1:",
                 seg_selector = in(reg) kern_code_segment_selector.as_raw(),
                 tmp = lateout(reg) _,
                 options(preserves_flags));
        }
    }

    /**
     * Loads the `TaskStateSegment` from the given selector
     */
    fn load_tss_segment(&self, tss_segment_selector: SegmentSelector) {
        unsafe {
            asm!("ltr       {0:x}",
                 in(reg) tss_segment_selector.as_raw(),
                 options(nomem, nostack, preserves_flags));
        }
    }
}

impl HwCpuBase for HwCpu {
    fn new_bsp() -> Self {
        Self { m_id: 0,
               m_is_ap: false,
               m_gdt: GlobalDescTable::new(),
               m_tss: TaskStateSegment::new(),
               m_double_fault_stack: [0; C_DOUBLE_FAULT_STACK] }
    }

    fn new_ap() -> Self {
        Self { m_id: 0,
               m_is_ap: true,
               m_gdt: GlobalDescTable::new(),
               m_tss: TaskStateSegment::new(),
               m_double_fault_stack: [0; C_DOUBLE_FAULT_STACK] }
    }

    fn init(&'static mut self) {
        /* set the double fault stack pointer into the TSS */
        self.m_tss.m_full_intr_stack_table[C_DOUBLE_FAULT_STACK_INDEX]
            = VirtAddr::from(self.m_double_fault_stack.as_mut_ptr()).offset(C_DOUBLE_FAULT_STACK as isize);

        /* setup the GDT segments */
        let kern_code_segment_selector =
            self.m_gdt.add_entry(Segment::kernel_code_segment());
        self.m_gdt.add_entry(Segment::kernel_data_segment());
        self.m_gdt.add_entry(Segment::user_code_segment());
        self.m_gdt.add_entry(Segment::user_data_segment());
        let tss_segment_selector =
            self.m_gdt.add_entry(Segment::tss_segment(&self.m_tss));

        /* load the GDT, reload the code-segment register (CS) and load the TSS */
        self.m_gdt.load();
        self.reload_code_segment_register(kern_code_segment_selector);
        self.load_tss_segment(tss_segment_selector);
    }

    fn current_id() -> CpuId {
        0 /* TODO APIC */
    }

    fn id(&self) -> CpuId {
        self.m_id
    }
}
