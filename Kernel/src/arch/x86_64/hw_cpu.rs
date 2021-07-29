/*! x86_64 CPU management implementation */

use core::arch::x86_64::{
    CpuidResult,
    __cpuid
};

use bits::bit_fields::TBitFields;

use crate::{
    addr::{
        virt_addr::VirtAddr,
        TAddress
    },
    arch::x86_64::{
        acpi_manager::AcpiManager,
        gdt::{
            GlobalDescTable,
            Segment
        },
        idt::IntrDescTable,
        local_apic::LocalApic,
        tss::TaskStateSegment
    },
    cpu::{
        CpuId,
        THwCpu
    }
};

const C_DOUBLE_FAULT_STACK: usize = 4096;
const C_DOUBLE_FAULT_STACK_INDEX: usize = 0;

/**
 * x86_64 `HwCpuBase` implementation
 */
pub struct HwCpu {
    m_is_ap: bool,
    m_gdt: GlobalDescTable,
    m_tss: TaskStateSegment,
    m_idt: IntrDescTable,
    m_local_apic: LocalApic,
    m_double_fault_stack: [u8; C_DOUBLE_FAULT_STACK]
}

impl HwCpu /* Privates */ {
    fn cpu_frequency_info(&self) -> CpuidResult {
        unsafe { __cpuid(0x16) }
    }
}

impl THwCpu for HwCpu {
    fn new_bsp() -> Self {
        Self { m_is_ap: false,
               m_gdt: GlobalDescTable::new(),
               m_tss: TaskStateSegment::new(),
               m_idt: IntrDescTable {},
               m_local_apic: LocalApic::new_uninitialized(),
               m_double_fault_stack: [0; C_DOUBLE_FAULT_STACK] }
    }

    fn new_ap() -> Self {
        Self { m_is_ap: true,
               m_gdt: GlobalDescTable::new(),
               m_tss: TaskStateSegment::new(),
               m_idt: IntrDescTable {},
               m_local_apic: LocalApic::new_uninitialized(),
               m_double_fault_stack: [0; C_DOUBLE_FAULT_STACK] }
    }

    fn init(&'static mut self) {
        /* set the double fault stack pointer into the TSS */
        self.m_tss.m_full_intr_stack_table[C_DOUBLE_FAULT_STACK_INDEX] = {
            /* obtain the <VirtAddr> of the static buffer */
            let double_fault_stack_virt_addr: VirtAddr =
                self.m_double_fault_stack.as_mut_ptr().into();

            /* return the bottom of the area */
            double_fault_stack_virt_addr.offset(C_DOUBLE_FAULT_STACK)
        };

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

        /* reload the code-segment register (CS) */
        unsafe {
            asm!("push      {css}",
                 "lea       {tmp}, [1f + rip]",
                 "push      {tmp}",
                 "retfq",
                 "1:",
                 css = in(reg) kern_code_segment_selector.as_raw(),
                 tmp = lateout(reg) _,
                 options(preserves_flags));
        }

        /* load the TSS */
        unsafe {
            asm!("ltr {0:x}",
                 in(reg) tss_segment_selector.as_raw(),
                 options(nomem, nostack, preserves_flags));
        }
    }

    fn init_interrupts(&'static mut self) {
        if !self.m_is_ap {
            /* initialize APIC and collect ACPI tables on BSP */
            if !LocalApic::init_apic() {
                panic!("This CPU doesn't support APIC");
            }
            AcpiManager::init_instance();
        }
        self.m_local_apic.init_and_enable();
    }

    fn do_halt(&self) {
        unsafe {
            asm!("cli; hlt");
        }
    }

    fn do_enable_interrupts(&self) {
        unsafe {
            asm!("sti", options(nostack));
        }
    }

    fn do_disable_interrupts(&self) {
        unsafe {
            asm!("cli", options(nostack));
        }
    }

    fn current_id() -> CpuId {
        LocalApic::this_core_id()
    }

    fn id(&self) -> CpuId {
        if self.m_local_apic.is_enabled() {
            self.m_local_apic.cpu_id()
        } else {
            0
        }
    }

    fn base_frequency(&self) -> u64 {
        self.cpu_frequency_info().eax.bits_at(0..15) as u64
    }

    fn max_frequency(&self) -> u64 {
        self.cpu_frequency_info().ebx.bits_at(0..15) as u64
    }

    fn bus_frequency(&self) -> u64 {
        self.cpu_frequency_info().ecx.bits_at(0..15) as u64
    }

    fn are_interrupts_enabled(&self) -> bool {
        let rflags: u64;
        unsafe {
            asm!("pushfq; pop {}", out(reg) rflags, options(nomem, preserves_flags));
        }
        rflags.bit_at(9)
    }
}
