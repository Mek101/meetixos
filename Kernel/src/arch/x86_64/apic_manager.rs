/*! Local Advanced Programmable Interrupt Controller */

use core::{
    arch::x86_64::__cpuid,
    ptr::{
        read_volatile,
        write_volatile
    }
};

use bits::bit_fields::TBitFields;

use crate::{
    addr::{
        phys_addr::PhysAddr,
        virt_addr::VirtAddr
    },
    arch::x86_64::ms_register::MsRegister,
    processor::CpuCoreId,
    vm::{
        mem_manager::MemManager,
        Page4KiB
    }
};

/* <None> until <ApicManager::init_instance()> is called */
static mut SM_APIC_MANAGER: Option<ApicManager> = None;

/**
 * Advanced Programmable Interrupt Controller manager
 */
pub struct ApicManager {
    m_base_virt_addr: VirtAddr
}

impl ApicManager /* Constructor */ {
    /**
     * Initializes the global `SM_APIC_MANAGER` instance
     */
    pub fn init_instance() {
        /* fail if the APIC is not supported by the CPU */
        if !Self::is_supported() {
            panic!("This CPU doesn't support APIC");
        }

        /* obtain from the APIC-MSR the APIC base value */
        let apic_msr = MsRegister::new(0x1b);
        let apic_base = unsafe { apic_msr.read() };

        /* check for disabled APIC (bit 11 stores enable-bit) */
        if !apic_base.bit_at(11) {
            panic!("The APIC is disabled at hardware level");
        }

        /* obtain the APIC physical address and convert it to virtual */
        let apic_base_phys_addr: PhysAddr = ((apic_base & 0xffff_f000) as usize).into();
        let apic_base_virt_addr = {
            /* TODO MAP A REGION */
            let apic_base_virt_addr: VirtAddr = (*apic_base_phys_addr).into();

            MemManager::instance().kernel_page_dir()
                .ensure_page_table_entry::<Page4KiB>(apic_base_virt_addr)
                .expect("Failed to map APIC")
                .set_phys_frame(apic_base_phys_addr)
                .set_present(true)
                .set_writeable(true)
                .set_readable(true)
                .set_user(false);

            apic_base_virt_addr
        };
        unsafe {
            SM_APIC_MANAGER = Some(Self { m_base_virt_addr: apic_base_virt_addr });

            /* store into the APIC-MSR the enable flag */
            apic_msr.write(apic_base | (1 << 11));
        }
    }
}

impl ApicManager /* Static Functions */ {
    /**
     * Returns whether the APIC is supported by the CPU
     */
    pub fn is_supported() -> bool {
        (unsafe { __cpuid(0x01) }.edx & (1 << 9)) != 0
    }
}

impl ApicManager /* Getters */ {
    /**
     * Returns the global instance of the `AcpiManager`
     */
    pub fn instance() -> &'static Self {
        unsafe {
            SM_APIC_MANAGER.as_ref().expect("Called ApicManager::instance() before \
                                             ApicManager::init_instance()")
        }
    }

    /**
     * Returns the `LocalApic` for this core
     */
    pub fn local_apic(&self) -> LocalApic {
        LocalApic { m_virt_addr: self.m_base_virt_addr }
    }
}

/**
 * Local Advanced Programmable Interrupt Controller
 */
pub struct LocalApic {
    m_virt_addr: VirtAddr
}

impl LocalApic /* Methods */ {
    /**
     * Enables the `LocalApic` for this core
     */
    pub unsafe fn enable(&mut self) {
        /* enable spurious interrupts */
        if self.read(Register::SpuriousInterrupt) & SPURIOUS_INTERRUPT_ENABLE == 0 {
            self.write(Register::SpuriousInterrupt, SPURIOUS_INTERRUPT_ENABLE);
        }

        /* set task priority and 16 as timer counter divider */
        self.write(Register::TaskPrio, 0x10);
        self.write(Register::TimerDivideConfig, 0x3)
    }
}

impl LocalApic /* Getters */ {
    /**
     * Returns the hardware `CpuId`
     */
    pub fn cpu_id(&self) -> CpuCoreId {
        unsafe { self.read(Register::CoreId) as CpuCoreId }
    }

    /**
     * Notifies the end of an interrupt
     */
    pub fn end_of_interrupt(&self) {
        unsafe { self.write(Register::EndOfInterrupt, 0) }
    }
}

impl LocalApic /* Privates */ {
    /**
     * Reads the value of the given `Register`
     */
    unsafe fn read(&self, register: Register) -> u32 {
        read_volatile((*self.m_virt_addr + register as usize) as *const u32)
    }

    /**
     * Overwrites the value of the given `Register`
     */
    unsafe fn write(&self, register: Register, value: u32) {
        write_volatile((*self.m_virt_addr + register as usize) as *mut u32, value);
    }
}

#[repr(usize)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
enum Register {
    CoreId                   = 0x20,
    TaskPrio                 = 0x80,
    EndOfInterrupt           = 0xB0,
    SpuriousInterrupt        = 0xF0,
    IntrCommandLow           = 0x300,
    IntrCommandHigh          = 0x310,
    LocalVecTableTimer       = 0x320,
    LocalVecTableThermal     = 0x330,
    LocalVecTablePerfCounter = 0x340,
    LocalVecTableLint0       = 0x350,
    LocalVecTableLint1       = 0x360,
    LocalVecTableError       = 0x370,
    TimerInitCounter         = 0x380,
    TimerCurrentCounter      = 0x390,
    TimerDivideConfig        = 0x3E0
}

const SPURIOUS_INTERRUPT_DISABLE: u32 = 0 << 8;
const SPURIOUS_INTERRUPT_ENABLE: u32 = 1 << 8;

const DELIVERY_MODE_NORMAL: u32 = 0 << 8;
const DELIVERY_MODE_LOW_PRIO: u32 = 1 << 8;
const DELIVERY_MODE_SYSTEM_MANAGEMENT: u32 = 2 << 8;
const DELIVERY_MODE_NON_MASKABLE: u32 = 4 << 8;
const DELIVERY_MODE_INIT: u32 = 5 << 8;
const DELIVERY_MODE_INTER_PROCESSOR: u32 = 6 << 8;
const DELIVERY_MODE_EXTERNAL: u32 = 7 << 8;

const DESTINATION_MODE_PHYSICAL: u32 = 0 << 11;
const DESTINATION_MODE_LOGICAL: u32 = 1 << 11;

const DELIVERY_STATUS_IDLE: u32 = 0 << 12;
const DELIVERY_STATUS_PENDING: u32 = 1 << 12;

const LEVEL_ASSERT: u32 = 0 << 14;
const LEVEL_DE_ASSERT: u32 = 1 << 14;

const TRIGGER_MODE_EDGE: u32 = 0 << 15;
const TRIGGER_MODE_LEVEL: u32 = 1 << 15;

const MODE_ONE_SHOT: u32 = 0 << 17;
const MODE_PERIODIC: u32 = 1 << 17;
const MODE_DEADLINE: u32 = 2 << 17;

const DESTINATION_NONE: u32 = 0 << 18;
const DESTINATION_THIS: u32 = 1 << 18;
const DESTINATION_ALL: u32 = 2 << 18;
const DESTINATION_ALL_BUT_THIS: u32 = 3 << 18;
