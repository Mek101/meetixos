/*! Kernel CPU management */

use alloc::{
    collections::BTreeMap,
    sync::Arc
};

use crate::{
    arch::hw_cpu_core::HwCpuCore,
    task::thread::Thread
};

/* <None> until <Processor::init_instance()> is called */
static mut SM_PROCESSOR: Option<Processor> = None;

/**
 * Unique `CpuCore` identifier
 */
pub type CpuCoreId = usize;

/**
 * High level representation for package processor
 */
pub struct Processor {
    m_cores_map: BTreeMap<CpuCoreId, CpuCore>,
    m_cores_max_frequency: u64,
    m_cores_bus_frequency: u64
}

impl Processor /* Constructors */ {
    /**
     * Initializes the global `SM_PROCESSOR` instance
     */
    pub fn init_instance() {
        const C_CORES_MAP_INIT_VAL: Option<CpuCore> = None;
        const C_BSP_CPU_CORE_ID: CpuCoreId = 0;

        /* initialize the global instance */
        unsafe {
            SM_PROCESSOR = Some(Self { m_cores_map: BTreeMap::new(),
                                       m_cores_max_frequency: 0,
                                       m_cores_bus_frequency: 0 });
        }

        /* register the BSP CPU core and initialize it */
        Self::instance_mut().register_cpu_core(C_BSP_CPU_CORE_ID, false);
        Self::instance_mut().core_by_id_mut(C_BSP_CPU_CORE_ID)
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
        self.m_cores_map.insert(cpu_core_id,
                                CpuCore { m_hw_cpu: HwCpuCore::new(is_ap),
                                          m_current_thread: None,
                                          m_idle_thread: None });
    }

    /**
     * Initializes the `CpuCore` for the current AP CPU core
     */
    pub fn init_this_ap(&mut self) {
        self.this_core_mut().m_hw_cpu.init();
    }

    /**
     * Initializes the interrupts management for the BSP CPU core
     */
    pub fn init_interrupts_for_bsp(&mut self) {
        let (cores_max_frequency, cores_bus_frequency) = {
            let bsp_cpu_core = self.core_by_id_mut(0)
                                   .expect("Processor doesn't have BSP CPU registered");

            /* initialize the interrupts of the BSP then calculate the speed of the
             * CPU, this is done here and after the interrupts initialization since
             * it uses interrupts management to do this
             */
            bsp_cpu_core.m_hw_cpu.init_interrupts();
            bsp_cpu_core.m_hw_cpu.calculate_speed()
        };

        self.m_cores_max_frequency = cores_max_frequency;
        self.m_cores_bus_frequency = cores_bus_frequency;
    }

    /**
     * Initializes the interrupts management for this AP CPU core
     */
    pub fn init_interrupts_for_this_ap(&self) {
        self.this_core().m_hw_cpu.init_interrupts()
    }

    /**
     * Starts the Symmetric Multi Processor
     */
    pub fn start_smp(&self) {
        todo!()
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

    /**
     * Returns the global `Processor` mutable instance
     */
    pub fn instance_mut() -> &'static mut Self {
        unsafe {
            SM_PROCESSOR.as_mut().expect("Called Processor::instance_mut() before \
                                          Processor::init_instance()")
        }
    }

    /**
     * Returns the current executing `CpuCore` instance
     */
    pub fn this_core(&self) -> &CpuCore {
        self.core_by_id(HwCpuCore::this_id())
            .expect("Processor::this_core(): Requested an unregistered CpuCore")
    }

    /**
     * Returns the current executing `CpuCore` mutable instance
     */
    pub fn this_core_mut(&mut self) -> &mut CpuCore {
        self.core_by_id_mut(HwCpuCore::this_id())
            .expect("Processor::this_core_mut(): Requested an unregistered CpuCore")
    }

    /**
     * Returns the `CpuCore` by his `CpuCoreId`
     */
    pub fn core_by_id(&self, cpu_core_id: CpuCoreId) -> Option<&CpuCore> {
        self.m_cores_map.get(&cpu_core_id)
    }

    /**
     * Returns the mutable `CpuCore` by his `CpuCoreId`
     */
    pub fn core_by_id_mut(&mut self, cpu_core_id: CpuCoreId) -> Option<&mut CpuCore> {
        self.m_cores_map.get_mut(&cpu_core_id)
    }

    /**
     * Returns the amount of registered cores
     */
    pub fn cores_count(&self) -> usize {
        self.m_cores_map.len()
    }

    /**
     * Returns the maximum frequency in Hz
     */
    pub fn cores_max_frequency(&self) -> u64 {
        self.m_cores_max_frequency
    }

    /**
     * Returns the bus frequency in Hz
     */
    pub fn cores_bus_frequency(&self) -> u64 {
        self.m_cores_bus_frequency
    }
}

/**
 * Per-CPU Core structure
 */
pub struct CpuCore {
    m_hw_cpu: HwCpuCore,
    m_current_thread: Option<Arc<Thread>>,
    m_idle_thread: Option<Arc<Thread>>
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

    /**
     * Returns the current `Thread` for this CPU Core
     */
    pub fn current_thread(&self) -> Arc<Thread> {
        self.m_current_thread
            .as_ref()
            .expect("Requested current_thread to the CpuCore but is None")
            .clone()
    }

    /**
     * Returns the idle `Thread` for this CPU Core
     */
    pub fn idle_thread(&self) -> Arc<Thread> {
        self.m_idle_thread
            .as_ref()
            .expect("Requested idle_thread to the CpuCore but is None")
            .clone()
    }
}

impl CpuCore /* Setters */ {
    /**
     * Sets the current `Thread` for this CPU Core
     */
    pub fn set_current_thread(&mut self, current_thread: Arc<Thread>) {
        self.m_current_thread = Some(current_thread);
    }

    /**
     * Sets the idle `Thread` for this CPU Core
     */
    pub fn set_idle_thread(&mut self, idle_thread: Arc<Thread>) {
        self.m_idle_thread = Some(idle_thread);
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
    fn init(&mut self);

    /**
     * Here must be initialized hardware interrupts controller and any stuff
     * which is needed by the architecture to manage software and hardware
     * interruptions
     */
    fn init_interrupts(&self);

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
     * Returns the best approximation of the cores and the bus speed in Hz
     */
    fn calculate_speed(&self) -> (u64, u64);

    /**
     * Returns the `CpuId` of the executing `Cpu`
     */
    fn this_id() -> CpuCoreId;

    /**
     * Returns the hardware `CpuId` of this `HwCpu`
     */
    fn id(&self) -> CpuCoreId;

    /**
     * Returns whether this `Cpu` have hardware interrupts enabled
     */
    fn are_interrupts_enabled(&self) -> bool;
}
