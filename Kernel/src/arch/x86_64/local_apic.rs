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
        virt_addr::VirtAddr,
        TAddress
    },
    arch::x86_64::ms_register::MsRegister,
    cpu::CpuId,
    vm::mem_manager::MemManager
};

static mut SM_APIC_BASE_VIRT_ADDR: Option<VirtAddr> = None;

pub struct LocalApic {
    m_virt_addr: VirtAddr,
    m_enabled: bool
}

impl LocalApic /* Constructors */ {
    pub fn init_apic() -> bool {
        /* fail if the APIC is not supported by the CPU */
        if !Self::is_supported() {
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
        unsafe {
            /* store the APIC base VirtAddr for use of the other cores */
            SM_APIC_BASE_VIRT_ADDR = Some(apic_base_virt_addr);

            /* store into the APIC-MSR the enable flag */
            apic_msr.write(apic_base as u64 | (1 << 11));
        }
        true
    }

    pub fn new() -> Self {
        Self { m_virt_addr: VirtAddr::null(),
               m_enabled: false }
    }
}

impl LocalApic /* Methods */ {
    pub fn enable(&mut self) {
        unsafe {
            assert!(SM_APIC_BASE_VIRT_ADDR.is_some(),
                    "Tried to initialize LocalApic before LocalApic::init_apic()");

            /* copy the LAPIC virtual address from the global one */
            self.m_virt_addr = SM_APIC_BASE_VIRT_ADDR.unwrap();

            /* enable spurious interrupts */
            if self.read(Register::SpuriousInterrupt) & SPURIOUS_INTERRUPT_DISABLE != 0 {
                self.write(Register::SpuriousInterrupt, SPURIOUS_INTERRUPT_ENABLE);
            }

            /* set task priority and 16 as timer counter divider */
            self.write(Register::TaskPrio, 0x10);
            self.write(Register::TimerDivideConfig, 0x3)
        }

        /* mask as enabled for this core */
        self.m_enabled = true;
    }
}

impl LocalApic /* Static Functions */ {
    pub fn is_supported() -> bool {
        (unsafe { __cpuid(0x01) }.edx & (1 << 9)) != 0
    }
}

impl LocalApic /* Getters */ {
    pub fn is_enabled(&self) -> bool {
        self.m_enabled
    }

    pub fn cpu_id(&self) -> CpuId {
        unsafe { self.read(Register::CoreId) as CpuId }
    }

    pub fn end_of_interrupt(&self) {
        unsafe { self.write(Register::EndOfInterrupt, 0) }
    }
}

impl LocalApic /* Privates */ {
    unsafe fn read(&self, register: Register) -> u32 {
        read_volatile((*self.m_virt_addr + register as usize) as *const u32)
    }

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
