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
        global_desc_table::{
            GlobalDescTable,
            Segment,
            SegmentSelector
        },
        interrupts::{
            apic_manager::ApicManager,
            intr_desc_table::IntrDescTable,
            intr_handler::syscall_entry
        },
        ms_register::MsRegister,
        task_state_segment::TaskStateSegment
    },
    dbg_print::DbgLevel,
    dbg_println,
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
    m_global_desc_table: GlobalDescTable,
    m_task_state_segment: TaskStateSegment,
    m_intr_desc_table: IntrDescTable,
    m_double_fault_stack: [u8; C_DOUBLE_FAULT_STACK]
}

impl THwCpuCore for HwCpuCore {
    fn new(is_ap: bool) -> Self {
        Self { m_is_ap: is_ap,
               m_global_desc_table: GlobalDescTable::new(),
               m_task_state_segment: TaskStateSegment::new(),
               m_intr_desc_table: IntrDescTable::new_uninitialized(),
               m_double_fault_stack: [0; C_DOUBLE_FAULT_STACK] }
    }

    fn init(&'static mut self) {
        dbg_println!(DbgLevel::Trace, "Initializing Double fault stack...");

        /* set the double fault stack pointer into the TSS */
        self.m_task_state_segment.m_intr_stack_table[C_DOUBLE_FAULT_STACK_INDEX] = {
            /* obtain the <VirtAddr> of the static buffer */
            let double_fault_stack_virt_addr: VirtAddr =
                self.m_double_fault_stack.as_mut_ptr().into();

            /* return the bottom of the area */
            double_fault_stack_virt_addr.offset(C_DOUBLE_FAULT_STACK)
        };

        /* setup the GDT segments */
        dbg_println!(DbgLevel::Trace, "Initializing GDT...");
        self.m_global_desc_table.add_entry(Segment::kernel_code_segment());
        self.m_global_desc_table.add_entry(Segment::kernel_data_segment());
        self.m_global_desc_table.add_entry(Segment::user_code_segment());
        self.m_global_desc_table.add_entry(Segment::user_data_segment());
        self.m_global_desc_table.add_entry(Segment::user_syscall_segment());
        self.m_global_desc_table
            .add_entry(Segment::tss_segment(&self.m_task_state_segment));

        /* flush the GDT table constructed */
        self.m_global_desc_table.load();

        /* load the TSS */
        let tss_selector: SegmentSelector = SegmentSelector::C_INDEX_TSS.into();
        unsafe {
            asm!("ltr {0:x}",
                 in(reg) tss_selector.as_raw(),
                 options(nomem, nostack, preserves_flags));
        }
    }

    fn init_interrupts(&'static mut self) {
        if !self.m_is_ap {
            /* initialize the Advanced Programmable Interrupt Controller */
            dbg_println!(DbgLevel::Trace, "Initializing APIC Manager...");
            ApicManager::init_instance();

            /* initialize the Advanced Configuration and Power Interface */
            dbg_println!(DbgLevel::Trace, "Initializing ACPI Manager...");
            AcpiManager::init_instance();

            /* register the AP CPUs */
            dbg_println!(DbgLevel::Trace, "Discovering APs CPUs...");
            AcpiManager::instance().register_ap_cpus();
        }

        /* enable the LAPIC for this CPU */
        dbg_println!(DbgLevel::Trace, "Enabling LAPIC for this CPU");
        unsafe {
            ApicManager::instance().local_apic().enable();
        }

        /* setup system-calls */
        dbg_println!(DbgLevel::Debug, "Enabling FastSystemCall Feature...");
        unsafe {
            /* STAR MSR need the code segments to use for syscall */
            let syscall_selector: SegmentSelector =
                SegmentSelector::C_INDEX_USER_SYSC.into();
            let kern_code_selector: SegmentSelector =
                SegmentSelector::C_INDEX_KERN_CODE.into();

            let msr_value = ((syscall_selector.as_raw() - 16) << 48)
                            | (kern_code_selector.as_raw() << 32);

            MsRegister::new_star().write(msr_value as u64);
            MsRegister::new_lstar().write(syscall_entry as u64);
            MsRegister::new_fmask().write(0x200);
        }

        /* flush the interrupts descriptor table */
        dbg_println!(DbgLevel::Debug, "Initializing IDT...");
        self.m_intr_desc_table.init_and_flush();
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
