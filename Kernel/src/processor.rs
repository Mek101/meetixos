/*! Kernel CPU management */

use alloc::collections::BTreeMap;

use crate::arch::hw_cpu::HwCpuCore;

static mut SM_PROCESSOR: Option<Processor> = None;

/**
 * Unique `CpuCore` identifier
 */
pub type CpuCoreId = u16;

pub struct Processor {
    m_cores: BTreeMap<CpuCoreId, CpuCore>,
    m_max_freq: u64,
    m_base_freq: u64,
    m_bus_freq: u64
}

impl Processor /* Constructors */ {
    /**
     * Initializes the global `SM_PROCESSOR` instance
     */
    pub fn init_instance() {
        /* initialize the global instance */
        unsafe {
            SM_PROCESSOR = Some(Self { m_cores: BTreeMap::new(),
                                       m_max_freq: HwCpuCore::max_frequency(),
                                       m_base_freq: HwCpuCore::base_frequency(),
                                       m_bus_freq: HwCpuCore::bus_frequency() });
        }

        let processor_mut = Self::instance_mut();

        /* register the BSP CPU core and initialize it */
        processor_mut.register_cpu_core(0, false);
        processor_mut.core_by_id_mut(0)
                     .expect("Failed to obtain BSP CPU core")
                     .m_hw_cpu
                     .init();
    }
}

impl Processor /* Methods */ {
    /**
     * Register the given core as `Processor` core
     */
    pub fn register_cpu_core(&mut self, cpu_core_id: CpuCoreId, is_ap: bool) {
        self.m_cores.insert(cpu_core_id, CpuCore { m_hw_cpu: HwCpuCore::new(is_ap) });
    }

    pub fn init_interrupts_for_bsp(&'static mut self) {
        self.core_by_id_mut(0)
            .expect("Processor doesn't have BSP CPU registered")
            .m_hw_cpu
            .init_interrupts();
    }

    pub fn init_interrupts_for_ap(&'static mut self) {
        self.this_core_mut().m_hw_cpu.init_interrupts();
    }
}

impl Processor /* Getters */ {
    /**
     * Returns the global `Processor` instance
     */
    pub fn instance() -> &'static Self {
        unsafe {
            SM_PROCESSOR.as_ref().expect("Called Processor::instance() before \
                                          Processor::init_instance()")
        }
    }

    pub fn instance_mut() -> &'static mut Self {
        unsafe {
            SM_PROCESSOR.as_mut().expect("Called Processor::instance_mut() before \
                                          Processor::init_instance()")
        }
    }

    pub fn this_core(&self) -> &CpuCore {
        self.core_by_id(HwCpuCore::this_id())
            .expect("Processor::this_core(): Requested an unregistered CpuCore")
    }

    pub fn this_core_mut(&mut self) -> &mut CpuCore {
        self.core_by_id_mut(HwCpuCore::this_id())
            .expect("Processor::this_core_mut(): Requested an unregistered CpuCore")
    }

    pub fn core_by_id(&self, cpu_core_id: CpuCoreId) -> Option<&CpuCore> {
        self.m_cores.get(&cpu_core_id)
    }

    pub fn core_by_id_mut(&mut self, cpu_core_id: CpuCoreId) -> Option<&mut CpuCore> {
        self.m_cores.get_mut(&cpu_core_id)
    }

    pub fn max_frequency(&self) -> u64 {
        self.m_max_freq
    }

    pub fn base_frequency(&self) -> u64 {
        self.m_base_freq
    }

    pub fn bus_frequency(&self) -> u64 {
        self.m_bus_freq
    }
}

/**
 * High-level CPU management
 */
pub struct CpuCore {
    m_hw_cpu: HwCpuCore
}

impl CpuCore /* Methods */ {
    /**
     * Enable hardware interrupts for this `Cpu`
     */
    pub fn enable_interrupts(&self) {
        self.m_hw_cpu.do_enable_interrupts();
    }

    /**
     * Disables hardware interrupts for this `Cpu`
     */
    pub fn disable_interrupts(&self) {
        self.m_hw_cpu.do_disable_interrupts();
    }

    /**
     * Executes `f` without interrupts for this `Cpu`
     */
    pub fn without_interrupts<F>(&self, f: F)
        where F: FnOnce() {
        let was_enabled = self.are_interrupts_enabled();
        if was_enabled {
            self.disable_interrupts();
        }

        f();

        if was_enabled {
            self.enable_interrupts()
        }
    }

    /**
     * Halts this CPU
     */
    pub fn halt(&self) -> ! {
        loop {
            self.m_hw_cpu.do_halt();
        }
    }
}

impl CpuCore /* Getters */ {
    /**
     * Returns the `CpuId` of this `Cpu`
     */
    pub fn id(&self) -> CpuCoreId {
        self.m_hw_cpu.id()
    }

    /**
     * Returns whether this `Cpu` have hardware interrupts enabled
     */
    pub fn are_interrupts_enabled(&self) -> bool {
        self.m_hw_cpu.are_interrupts_enabled()
    }
}

/**
 * Interface on which the `Cpu` relies to obtain information or throw
 * initialization for hardware CPU
 */
pub trait THwCpuCore {
    /**
     * Constructs an `HwCpu` which identifies an hardware CPU core
     */
    fn new(is_ap: bool) -> Self;

    /**
     * Once the `HwCpu` is stored into the static `SM_ALL_CPUS` array this
     * method is called to initialize internal hardware stuffs which may
     * need `'static` lifetimes
     */
    fn init(&'static mut self);

    /**
     * Here must be initialized hardware interrupts controller and any stuff
     * which is needed by the architecture to manage software and hardware
     * interruptions
     */
    fn init_interrupts(&'static mut self);

    /**
     * Halts this `HwCpu`
     */
    fn do_halt(&self);

    /**
     * Enable hardware interrupts for this `Cpu`
     */
    fn do_enable_interrupts(&self);

    /**
     * Disables hardware interrupts for this `Cpu`
     */
    fn do_disable_interrupts(&self);

    /**
     * Returns the `CpuId` of the executing `Cpu`
     */
    fn this_id() -> CpuCoreId;

    /**
     * Returns the base frequency in Hz of this `Cpu`
     */
    fn base_frequency() -> u64;

    /**
     * Returns the maximum frequency in Hz of this `Cpu`
     */
    fn max_frequency() -> u64;

    /**
     * Returns the bus frequency in Hz of this `Cpu`
     */
    fn bus_frequency() -> u64;

    /**
     * Returns the hardware `CpuId` of this `HwCpu`
     */
    fn id(&self) -> CpuCoreId;

    /**
     * Returns whether this `Cpu` have hardware interrupts enabled
     */
    fn are_interrupts_enabled(&self) -> bool;
}
