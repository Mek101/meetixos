/*! Local Advanced Programmable Interrupt Controller */

use alloc::vec::Vec;
use core::{
    arch::x86_64::__cpuid,
    ops::Range,
    ptr::{
        read_volatile,
        write_volatile
    }
};

use num_enum::IntoPrimitive;

use bits::bit_fields::TBitFields;

use crate::{
    addr::{
        phys_addr::PhysAddr,
        virt_addr::VirtAddr,
        TAddress
    },
    arch::x86_64::ms_register::MsRegister,
    processor::CpuCoreId,
    vm::{
        mem_manager::MemManager,
        Page4KiB
    }
};

const ISA_IRQ_COUNT: usize = 16;

/* <None> until <ApicManager::init_instance()> is called */
static mut SM_APIC_MANAGER: Option<ApicManager> = None;

/**
 * Advanced Programmable Interrupt Controller manager
 */
pub struct ApicManager {
    m_base_virt_addr: VirtAddr,
    m_io_apics: Vec<IoApic>,
    m_io_apic_configs: Vec<IoApicConfig>
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
            /* TODO MAP A KERNEL REGION */
            let apic_base_addr: VirtAddr = (*apic_base_phys_addr).into();

            /* map into virtual memory the APIC base address */
            MemManager::instance().kernel_page_dir()
                                  .ensure_page_table_entry::<Page4KiB>(apic_base_addr)
                                  .expect("Failed to map APIC")
                                  .set_phys_frame(apic_base_phys_addr)
                                  .set_present(true)
                                  .set_writeable(true)
                                  .set_readable(true)
                                  .set_user(false);

            apic_base_addr
        };

        /* initialize to a default value all the ISA IRQ configurations */
        let io_apic_configs = {
            let mut io_apic_configs = Vec::with_capacity(ISA_IRQ_COUNT);

            for i in 0..ISA_IRQ_COUNT {
                /* construct a reasonable value for the I/O APIC config */
                let io_apic_config = IoApicConfig { m_gsi: i as u32,
                                                    m_delivery_mode:
                                                        DELIVERY_MODE_NORMAL,
                                                    m_polarity: POLARITY_HIGH_ACTIVE,
                                                    m_trigger_mode: TRIGGER_MODE_EDGE };

                /* put into the collection */
                io_apic_configs.push(io_apic_config);
            }
            io_apic_configs
        };

        unsafe {
            SM_APIC_MANAGER = Some(Self { m_base_virt_addr: apic_base_virt_addr,
                                          m_io_apics: Vec::with_capacity(8),
                                          m_io_apic_configs: io_apic_configs });

            /* store into the APIC-MSR the enable flag */
            apic_msr.write(apic_base | (1 << 11));
        }
    }
}

impl ApicManager /* Methods */ {
    /**
     * Adds an I/O APIC instance into the APIC manager
     */
    pub fn add_io_apic(&mut self, id: u8, base_addr: usize, base_gsi: u32) {
        /* TODO MAP A KERNEL REGION */
        let io_apic_base_addr = {
            let io_apic_base_addr: VirtAddr = base_addr.into();

            /* map into virtual memory the I/O APIC base address */
            MemManager::instance().kernel_page_dir()
                                  .ensure_page_table_entry::<Page4KiB>(io_apic_base_addr)
                                  .expect("Failed to map I/O APIC")
                                  .set_phys_frame(base_addr.into())
                                  .set_present(true)
                                  .set_readable(true)
                                  .set_writeable(true)
                                  .set_user(false);

            io_apic_base_addr
        };

        /* use a fake I/O APIC to read from registers */
        let readable_io_apic = IoApic { m_id: 0,
                                        m_version: 0,
                                        m_reg_ptr: io_apic_base_addr.as_ptr_mut(),
                                        m_data_ptr: io_apic_base_addr.offset(10)
                                                                     .as_ptr_mut(),
                                        m_gsi_range: Range::default() };

        /* construct the real I/O APIC */
        let io_apic = IoApic { m_id: id,
                               m_version: readable_io_apic.version() as u8,
                               m_reg_ptr: readable_io_apic.m_reg_ptr,
                               m_data_ptr: readable_io_apic.m_data_ptr,
                               m_gsi_range: base_gsi..readable_io_apic.count_max() + 1 };

        /* initialize the I/O APIC redirections */
        let red_table_reg: u32 = IoApicRegister::RedTable.into();
        for gsi in io_apic.m_gsi_range.clone() {
            unsafe {
                io_apic.write(red_table_reg + gsi * 2, INTERRUPT_MASK_ENABLE);
                io_apic.write(red_table_reg + gsi * 2 + 1, 0);
            }
        }

        /* store the I/O APIC into the global vector */
        self.m_io_apics.push(io_apic);
    }

    /**
     * Configures the interrupt `irq` with the given configuration
     * parameters
     */
    pub fn configure_irq(&mut self,
                         irq: u8,
                         gsi: u32,
                         delivery_mode_fixed: bool,
                         polarity_high_active: bool,
                         trigger_edge: bool) {
        assert!(irq < (ISA_IRQ_COUNT as u8));

        let delivery_mode = if delivery_mode_fixed {
            DELIVERY_MODE_NORMAL
        } else {
            DELIVERY_MODE_NON_MASKABLE
        };
        let polarity = if polarity_high_active {
            POLARITY_HIGH_ACTIVE
        } else {
            POLARITY_LOW_ACTIVE
        };
        let trigger_mode = if trigger_edge {
            TRIGGER_MODE_EDGE
        } else {
            TRIGGER_MODE_LEVEL
        };

        if let Some(_) = self.io_apic_by_gsi(gsi) {
            let io_apic_config = &mut self.m_io_apic_configs[irq as usize];

            io_apic_config.m_gsi = gsi;
            io_apic_config.m_delivery_mode = delivery_mode;
            io_apic_config.m_polarity = polarity;
            io_apic_config.m_trigger_mode = trigger_mode;
        }
    }

    /**
     * Enables the given interrupt
     */
    pub fn enable_irq(&self, irq: u8) {
        /* TODO vector check & existence */

        let gsi = self.irq_to_gsi(irq);
        let cpu_core_id = self.local_apic().cpu_id();
        if let Some(io_apic) = self.io_apic_by_gsi(gsi) {
            let gsi = gsi - io_apic.m_gsi_range.start;
            let io_apic_config = &self.m_io_apic_configs[irq as usize];

            let red_table_reg: u32 = IoApicRegister::RedTable.into();
            unsafe {
                io_apic.write(red_table_reg + gsi * 2 + 1, (cpu_core_id << 24) as u32);
                io_apic.write(red_table_reg + gsi * 2,
                              INTERRUPT_MASK_DISABLE
                              | DESTINATION_MODE_PHYSICAL
                              | io_apic_config.m_polarity
                              | io_apic_config.m_trigger_mode
                              | io_apic_config.m_delivery_mode
                              | (irq + 0x20) as u32 /* TODO interrupt vector */);
            }
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
     * Returns the global mutable instance of the `AcpiManager`
     */
    pub unsafe fn instance_mut() -> &'static mut Self {
        SM_APIC_MANAGER.as_mut().expect("Called ApicManager::instance_mut() before \
                                         ApicManager::init_instance()")
    }

    /**
     * Returns the `LocalApic` for this core
     */
    pub fn local_apic(&self) -> LocalApic {
        LocalApic { m_virt_addr: self.m_base_virt_addr }
    }

    /**
     * Returns the I/O APIC for the given global-system-interrupt code
     */
    pub fn io_apic_by_gsi(&self, gsi: u32) -> Option<&IoApic> {
        self.m_io_apics.iter().find(|io_apic| io_apic.m_gsi_range.contains(&gsi))
    }

    /**
     * Returns the GSI for the given IRQ
     */
    pub fn irq_to_gsi(&self, irq: u8) -> u32 {
        self.m_io_apic_configs[irq as usize].m_gsi
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
        if self.read(LapicRegister::SpuriousInterrupt) & SPURIOUS_INTERRUPT_ENABLE == 0 {
            self.write(LapicRegister::SpuriousInterrupt, SPURIOUS_INTERRUPT_ENABLE);
        }

        /* set task priority and 16 as timer counter divider */
        self.write(LapicRegister::TaskPrio, 0x10);
        self.write(LapicRegister::TimerDivideConfig, 0x3)
    }
}

impl LocalApic /* Getters */ {
    /**
     * Returns the hardware `CpuId`
     */
    pub fn cpu_id(&self) -> CpuCoreId {
        unsafe { self.read(LapicRegister::CoreId) as CpuCoreId }
    }

    /**
     * Notifies the end of an interrupt
     */
    pub fn end_of_interrupt(&self) {
        unsafe { self.write(LapicRegister::EndOfInterrupt, 0) }
    }
}

impl LocalApic /* Privates */ {
    /**
     * Reads the value of the given `Register`
     */
    unsafe fn read(&self, register: LapicRegister) -> u32 {
        read_volatile(self.m_virt_addr.offset(register.into()).as_ptr())
    }

    /**
     * Overwrites the value of the given `Register`
     */
    unsafe fn write(&self, register: LapicRegister, value: u32) {
        write_volatile(self.m_virt_addr.offset(register.into()).as_ptr_mut(), value);
    }
}

/**
 * I/O APIC implementation
 */
pub struct IoApic {
    m_id: u8,
    m_version: u8,
    m_reg_ptr: *mut u32,
    m_data_ptr: *mut u32,
    m_gsi_range: Range<u32>
}

impl IoApic /* Methods */ {
    /**
     * Mask the given global signal interrupt
     */
    pub fn mask(&self, gsi: u32) {
        let red_table_reg = {
            let red_table_reg: u32 = IoApicRegister::RedTable.into();
            red_table_reg + gsi * 2
        };

        unsafe {
            self.write(red_table_reg, self.read(red_table_reg) | INTERRUPT_MASK_ENABLE);
        }
    }
}

impl IoApic /* Privates */ {
    /**
     * Returns the count of GSI stored
     */
    fn count_max(&self) -> u32 {
        (self.version() >> 16) & 0xff
    }

    /**
     * Returns the version of this I/O APIC
     */
    fn version(&self) -> u32 {
        unsafe { self.read(IoApicRegister::Version) }
    }

    /**
     * Reads from the given register
     */
    unsafe fn read<T>(&self, reg: T) -> u32
        where T: Into<u32> {
        write_volatile(self.m_reg_ptr, reg.into());
        read_volatile(self.m_data_ptr)
    }

    /**
     * Writes into the given register
     */
    unsafe fn write<T>(&self, reg: T, value: u32)
        where T: Into<u32> {
        write_volatile(self.m_reg_ptr, reg.into());
        write_volatile(self.m_data_ptr, value);
    }
}

/**
 * I/O APIC redirection configuration
 */
struct IoApicConfig {
    m_gsi: u32,
    m_delivery_mode: u32,
    m_polarity: u32,
    m_trigger_mode: u32
}

#[repr(usize)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(IntoPrimitive)]
enum LapicRegister {
    CoreId                   = 0x020,
    TaskPrio                 = 0x080,
    EndOfInterrupt           = 0x0B0,
    SpuriousInterrupt        = 0x0F0,
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

#[repr(u32)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(IntoPrimitive)]
enum IoApicRegister {
    Id       = 0x00,
    Version  = 0x01,
    Arb      = 0x02,
    RedTable = 0x10
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

const POLARITY_HIGH_ACTIVE: u32 = 0 << 13;
const POLARITY_LOW_ACTIVE: u32 = 1 << 13;

const LEVEL_ASSERT: u32 = 0 << 14;
const LEVEL_DE_ASSERT: u32 = 1 << 14;

const TRIGGER_MODE_EDGE: u32 = 0 << 15;
const TRIGGER_MODE_LEVEL: u32 = 1 << 15;

const INTERRUPT_MASK_DISABLE: u32 = 0 << 16;
const INTERRUPT_MASK_ENABLE: u32 = 1 << 16;

const MODE_ONE_SHOT: u32 = 0 << 17;
const MODE_PERIODIC: u32 = 1 << 17;
const MODE_DEADLINE: u32 = 2 << 17;

const DESTINATION_NONE: u32 = 0 << 18;
const DESTINATION_THIS: u32 = 1 << 18;
const DESTINATION_ALL: u32 = 2 << 18;
const DESTINATION_ALL_BUT_THIS: u32 = 3 << 18;
