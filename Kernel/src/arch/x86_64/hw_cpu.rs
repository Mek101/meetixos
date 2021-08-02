/*! x86_64 CPU management implementation */

use core::arch::x86_64::__cpuid;

use bits::bit_fields::TBitFields;

use crate::{
    addr::{
        virt_addr::VirtAddr,
        TAddress
    },
    arch::x86_64::{
        acpi_manager::AcpiManager,
        apic_manager::ApicManager,
        gdt::{
            GlobalDescTable,
            Segment
        },
        idt::IntrDescTable,
        tss::TaskStateSegment
    },
    processor::{
        CpuCoreId,
        THwCpuCore
    }
};

const C_DOUBLE_FAULT_STACK: usize = 4096;
const C_DOUBLE_FAULT_STACK_INDEX: usize = 0;

/**
 * x86_64 `HwCpuBase` implementation
 */
pub struct HwCpuCore {
    m_is_ap: bool,
    m_gdt: GlobalDescTable,
    m_tss: TaskStateSegment,
    m_idt: IntrDescTable,
    m_double_fault_stack: [u8; C_DOUBLE_FAULT_STACK]
}

impl THwCpuCore for HwCpuCore {
    fn new(is_ap: bool) -> Self {
        Self { m_is_ap: is_ap,
               m_gdt: GlobalDescTable::new(),
               m_tss: TaskStateSegment::new(),
               m_idt: IntrDescTable {},
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
            ApicManager::init_instance();
            AcpiManager::init_instance();
        }

        /* enable the LAPIC for this CPU */
        unsafe {
            ApicManager::instance().local_apic().enable();
        }
    }

    fn do_halt(&self) {
        unsafe {
            asm!("cli; hlt");
        }
    }

    fn do_enable_interrupts(&self) {
        unsafe {
            asm!("sti", options(nomem, nostack));
        }
    }

    fn do_disable_interrupts(&self) {
        unsafe {
            asm!("cli", options(nomem, nostack));
        }
    }

    fn this_id() -> CpuCoreId {
        ApicManager::instance().local_apic().cpu_id()
    }

    fn base_frequency() -> u64 {
        unsafe { __cpuid(0x16) }.eax.bits_at(0..15) as u64
    }

    fn max_frequency() -> u64 {
        unsafe { __cpuid(0x16) }.ebx.bits_at(0..15) as u64
    }

    fn bus_frequency() -> u64 {
        unsafe { __cpuid(0x16) }.ecx.bits_at(0..15) as u64
    }

    fn id(&self) -> CpuCoreId {
        ApicManager::instance().local_apic().cpu_id()
    }

    fn are_interrupts_enabled(&self) -> bool {
        let rflags: u64;
        unsafe {
            asm!("pushfq; pop {}", out(reg) rflags, options(nomem, preserves_flags));
        }
        rflags.bit_at(9)
    }
}
