/*! x86_64 CPU management implementation */

use core::arch::x86_64::{
    CpuidResult,
    __cpuid
};

use bits::bit_fields::TBitFields;

use crate::{
    addr::{
        phys_addr::PhysAddr,
        virt_addr::VirtAddr,
        TAddress
    },
    arch::x86_64::{
        gdt::{
            GlobalDescTable,
            Segment,
            SegmentSelector
        },
        idt::IntrDescTable,
        local_apic::LocalApic,
        ms_register::MsRegister,
        pic::PicManager,
        tss::TaskStateSegment
    },
    cpu::{
        CpuId,
        THwCpu
    },
    vm::mem_manager::MemManager
};

const C_DOUBLE_FAULT_STACK: usize = 4096;
const C_DOUBLE_FAULT_STACK_INDEX: usize = 0;

static mut SM_APIC_BASE_VIRT_ADDR: Option<VirtAddr> = None;

/**
 * x86_64 `HwCpuBase` implementation
 */
pub struct HwCpu {
    m_id: CpuId,
    m_is_ap: bool,
    m_gdt: GlobalDescTable,
    m_tss: TaskStateSegment,
    m_idt: IntrDescTable,
    m_local_apic: LocalApic,
    m_double_fault_stack: [u8; C_DOUBLE_FAULT_STACK]
}

impl HwCpu /* Privates */ {
    /**
     * Reloads the code-segment register with the given kernel-code selector
     */
    fn reload_code_segment_register(&self, kern_code_segment_selector: SegmentSelector) {
        unsafe {
            asm!("push      {ssl}",
                 "lea       {tmp}, [1f + rip]",
                 "push      {tmp}",
                 "retfq",
                 "1:",
                 ssl = in(reg) kern_code_segment_selector.as_raw(),
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

    fn init_apic(&self) -> bool {
        /* fail if the APIC is not supported by the CPU */
        if !LocalApic::is_supported() {
            return false;
        }

        /* obtain from the APIC-MSR the APIC base value */
        let apic_msr = MsRegister::new(0x1b);
        let apic_base = unsafe { apic_msr.read() as usize };

        /* check for disabled APIC (bit 11 stores enable-bit) */
        if !apic_base.bit_at(11) {
            return false;
        }

        /* obtain the APIC physical address and convert it to virtual */
        let apic_base_phys_addr: PhysAddr = (apic_base & 0xffff_f000).into();
        let apic_base_virt_addr =
            MemManager::instance().layout_manager()
                                  .phys_addr_to_virt_addr(apic_base_phys_addr);
        crate::dbg_println!(crate::dbg_print::DbgLevel::Trace,
                            "apic_base: {}, apic_base_phys_addr: {}, \
                             apic_base_virt_addr: {}",
                            apic_base,
                            apic_base_phys_addr,
                            apic_base_virt_addr);
        unsafe {
            /* store the APIC base VirtAddr for use of the other cores */
            SM_APIC_BASE_VIRT_ADDR = Some(apic_base_virt_addr);

            /* store into the APIC-MSR the enable flag */
            apic_msr.write(apic_base as u64 | 0x800);
        }
        true
    }

    fn enable_local_apic(&mut self) {
    }

    fn cpu_frequency_info(&self) -> CpuidResult {
        unsafe { __cpuid(0x16) }
    }
}

impl THwCpu for HwCpu {
    fn new_bsp() -> Self {
        Self { m_id: 0,
               m_is_ap: false,
               m_gdt: GlobalDescTable::new(),
               m_tss: TaskStateSegment::new(),
               m_idt: IntrDescTable {},
               m_local_apic: LocalApic::new(),
               m_double_fault_stack: [0; C_DOUBLE_FAULT_STACK] }
    }

    fn new_ap() -> Self {
        Self { m_id: 0,
               m_is_ap: true,
               m_gdt: GlobalDescTable::new(),
               m_tss: TaskStateSegment::new(),
               m_idt: IntrDescTable {},
               m_local_apic: LocalApic::new(),
               m_double_fault_stack: [0; C_DOUBLE_FAULT_STACK] }
    }

    fn init(&'static mut self) {
        /* set the double fault stack pointer into the TSS */
        self.m_tss.m_full_intr_stack_table[C_DOUBLE_FAULT_STACK_INDEX]
            = VirtAddr::from(self.m_double_fault_stack.as_mut_ptr()).offset(C_DOUBLE_FAULT_STACK);

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

    fn init_interrupts(&'static mut self) {
        if !self.m_is_ap {
            PicManager::init_instance();

            /* initialize the APIC */
            self.init_apic();
        }
        self.enable_local_apic();
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
        0 /* TODO APIC */
    }

    fn id(&self) -> CpuId {
        self.m_id
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
