/*! x86_64 CPU management implementation */

use core::{
    arch::x86_64::_rdtsc,
    ptr::read_volatile
};

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
        pit::PitManager,
        task::task_state_segment::TaskStateSegment
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

impl HwCpuCore /* Privates */ {
    /**
     * Stores into the TSS the `m_double_fault_stack`
     */
    fn init_double_fault_stack(&mut self) {
        self.m_task_state_segment.m_intr_stack_table[C_DOUBLE_FAULT_STACK_INDEX] = {
            /* obtain the <VirtAddr> of the static buffer */
            let double_fault_stack_virt_addr: VirtAddr =
                self.m_double_fault_stack.as_mut_ptr().into();

            /* return the bottom of the area */
            double_fault_stack_virt_addr.offset(C_DOUBLE_FAULT_STACK)
        };
    }

    /**
     * Constructs and loads the GDT table for this `HwCpuCore`
     */
    fn init_gdt(&mut self) {
        self.m_global_desc_table.add_entry(Segment::kernel_code_segment());
        self.m_global_desc_table.add_entry(Segment::kernel_data_segment());
        self.m_global_desc_table.add_entry(Segment::user_code_segment());
        self.m_global_desc_table.add_entry(Segment::user_data_segment());
        self.m_global_desc_table.add_entry(Segment::user_syscall_segment());
        self.m_global_desc_table
            .add_entry(Segment::tss_segment(&self.m_task_state_segment));

        /* flush the GDT table constructed */
        self.m_global_desc_table.load();
    }

    /**
     * Loads the TSS segment
     */
    fn load_tss(&mut self) {
        /* load the TSS */
        let tss_selector: SegmentSelector = SegmentSelector::C_INDEX_TSS.into();
        unsafe {
            asm!("ltr {0:x}",
            in(reg) tss_selector.as_raw(),
            options(nomem, nostack, preserves_flags));
        }
    }

    /**
     * Enables fast system call feature
     */
    fn enable_fast_system_call(&self) {
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
    }

    /**
     * Detects the cpu speed using APIC and PIT
     */
    fn detect_speed(&self, instruction_count: usize) -> (u64, u64) {
        let pit_manager = PitManager::instance();
        let local_apic = ApicManager::instance().local_apic();

        /* set the PIT counter to -1 */
        pit_manager.enable_periodic(u16::MAX);
        let pit_counter = pit_manager.read_counter() as u64;

        /* set the LAPIC counter to -1 */
        local_apic.write_timer_counter(u32::MAX);
        let lapic_counter = local_apic.read_timer_counter() as u64;

        /* execute the instructions requested */
        let before = unsafe { _rdtsc() };
        let mut i = instruction_count;
        while i > 0 {
            unsafe {
                /* needed or the compiler will inline this loop */
                let _ = read_volatile(&i as *const _);
            }
            i -= 1;
        }
        let after = unsafe { _rdtsc() };

        /* read the left counters */
        let left_pit_count = pit_manager.read_counter() as u64;
        let left_lapic_count = local_apic.read_timer_counter() as u64;

        if left_pit_count == pit_counter {
            /* we have to increase the instruction count */
            (0, 0)
        } else {
            let pic_time = PitManager::BASE_FREQUENCY / (pit_counter - left_pit_count);

            ((after - before) * pic_time,
             (lapic_counter - left_lapic_count) * pic_time * ApicManager::TIMER_DIVIDER)
        }
    }
}

impl THwCpuCore for HwCpuCore {
    fn new(is_ap: bool) -> Self {
        Self { m_is_ap: is_ap,
               m_global_desc_table: GlobalDescTable::new(),
               m_task_state_segment: TaskStateSegment::new(),
               m_intr_desc_table: IntrDescTable::new(),
               m_double_fault_stack: [0; C_DOUBLE_FAULT_STACK] }
    }

    fn init(&mut self) {
        /* set the double fault stack pointer into the TSS */
        dbg_println!(DbgLevel::Trace, "Initializing Double Fault Stack...");
        self.init_double_fault_stack();

        /* setup the GDT segments */
        dbg_println!(DbgLevel::Trace, "Initializing GDT...");
        self.init_gdt();

        /* load the TSS segment */
        dbg_println!(DbgLevel::Trace, "Loading TSS Segment...");
        self.load_tss();
    }

    fn init_interrupts(&self) {
        if !self.m_is_ap {
            /* initialize the Advanced Programmable Interrupt Controller */
            dbg_println!(DbgLevel::Trace, "Initializing APIC Manager...");
            ApicManager::init_instance();

            /* initialize the Advanced Configuration and Power Interface */
            dbg_println!(DbgLevel::Trace, "Initializing ACPI Manager...");
            AcpiManager::init_instance();

            /* register the AP CPUs and discover I/O APICs */
            dbg_println!(DbgLevel::Trace, "Discovering AP CPUs...");
            AcpiManager::instance().register_ap_cpus();
        }

        /* enable the LAPIC for this CPU */
        dbg_println!(DbgLevel::Trace, "Enabling LAPIC for this CPU");
        ApicManager::instance().local_apic().enable();

        /* setup system-calls */
        dbg_println!(DbgLevel::Trace, "Enabling Fast SystemCall Feature...");
        self.enable_fast_system_call();

        /* flush the interrupts descriptor table */
        dbg_println!(DbgLevel::Trace, "Flushing IDT...");
        self.m_intr_desc_table.flush();
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

    fn calculate_speed(&self) -> (u64, u64) {
        const C_MEASURE_COUNT: usize = 5;
        const C_REQUIRED_MATCHES: usize = 3;
        const C_INSTR_COUNT_FACTOR: usize = 0x10000;
        const C_TOLERANCE: u64 = 1000000;

        /* called once by the BSP CpuCore, so init the PitManager here */
        PitManager::init_instance();

        let mut best_matches = -1;
        let mut best_max_frequency = 0;
        let mut best_bus_frequency = 0;
        for i in 1..=C_MEASURE_COUNT {
            /* try to detect the CPU speed from the current factor */
            let mut found = true;
            let (cores_max_frequency, cores_bus_frequency) =
                self.detect_speed(C_INSTR_COUNT_FACTOR * i * 100);
            if cores_max_frequency == 0 {
                continue;
            }

            /* retry for the required amount of matches */
            let mut match_counter = 0;
            for _ in 0..C_REQUIRED_MATCHES {
                let (match_cores_max_frequency, _) =
                    self.detect_speed(C_INSTR_COUNT_FACTOR * i * 100);

                let frequency_diff_1 = cores_max_frequency - match_cores_max_frequency;
                let frequency_diff_2 = match_cores_max_frequency - cores_max_frequency;

                /* check for tolerance */
                if (frequency_diff_1 > 0 && frequency_diff_1 > C_TOLERANCE)
                   || (frequency_diff_2 > 0 && frequency_diff_2 > C_TOLERANCE)
                {
                    found = false;
                    break;
                }

                match_counter += 1;
            }

            if found {
                best_max_frequency = cores_max_frequency;
                best_bus_frequency = cores_bus_frequency;
                break;
            }

            if match_counter > best_matches {
                best_matches = match_counter;
                best_max_frequency = cores_max_frequency;
                best_bus_frequency = cores_bus_frequency;
            }
        }

        (best_max_frequency, best_bus_frequency)
    }

    fn this_id() -> CpuCoreId {
        ApicManager::instance().local_apic().cpu_id()
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
