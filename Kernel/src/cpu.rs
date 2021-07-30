/*! Kernel CPU management */

use alloc::vec::Vec;

use crate::arch::hw_cpu::HwCpu;

/* All the CPUs descriptors currently active */
static mut SM_ALL_CPUS: Vec<Cpu> = Vec::new();

/**
 * The index of the `Cpu` inside the global vector
 */
pub type CpuId = u16;

/**
 * High-level CPU management
 */
pub struct Cpu {
    m_hw_cpu: HwCpu
}

impl Cpu /* Constructors */ {
    /**
     * Initializes the primary CPU.
     *
     * Called once by `kernel_rust_start()`
     */
    pub fn early_init() {
        Self::add_cpu(Self { m_hw_cpu: HwCpu::new_bsp() }).m_hw_cpu.init();
    }

    /**
     * Initializes the current secondary CPU.
     */
    pub fn init_ap() {
        Self::add_cpu(Self { m_hw_cpu: HwCpu::new_ap() }).m_hw_cpu.init();
    }
}

impl Cpu /* Methods */ {
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
        /* TODO halt other CPUs */
        loop {
            self.m_hw_cpu.do_halt();
        }
    }
}

impl Cpu /* Static Functions */ {
    /**
     * Initializes the interrupt management for this `Cpu`
     */
    pub fn init_interrupts_for_this() {
        /* obtain the current <Cpu> descriptor */
        let this_cpu_id = HwCpu::current_id();
        let this_cpu = unsafe {
            SM_ALL_CPUS.get_mut(this_cpu_id as usize)
                       .expect(format!("Requested an unregistered Cpu with id: {}",
                                       this_cpu_id).as_str())
        };

        /* initialize the interrupts management */
        this_cpu.m_hw_cpu.init_interrupts();
    }
}

impl Cpu /* Getters */ {
    /**
     * Returns the reference to the current `Cpu`
     */
    pub fn current() -> &'static Self {
        Self::by_id(HwCpu::current_id())
    }

    /**
     * Returns the reference to the given `cpu_id` `Cpu`
     */
    pub fn by_id(cpu_id: CpuId) -> &'static Self {
        unsafe {
            SM_ALL_CPUS.get(cpu_id as usize)
                       .expect(format!("Requested an unregistered Cpu with id: {}",
                                       cpu_id).as_str())
        }
    }

    /**
     * Returns the `CpuId` of this `Cpu`
     */
    pub fn id(&self) -> CpuId {
        self.m_hw_cpu.id()
    }

    /**
     * Returns the base frequency in Hz of this `Cpu`
     */
    pub fn base_frequency(&self) -> u64 {
        self.m_hw_cpu.base_frequency()
    }

    /**
     * Returns the maximum frequency in Hz of this `Cpu`
     */
    pub fn max_frequency(&self) -> u64 {
        self.m_hw_cpu.max_frequency()
    }

    /**
     * Returns the bus frequency in Hz of this `Cpu`
     */
    pub fn bus_frequency(&self) -> u64 {
        self.m_hw_cpu.bus_frequency()
    }

    /**
     * Returns whether this `Cpu` have hardware interrupts enabled
     */
    pub fn are_interrupts_enabled(&self) -> bool {
        self.m_hw_cpu.are_interrupts_enabled()
    }
}

impl Cpu /* Privates */ {
    /**
     * Stores the given `Cpu` into the `SM_ALL_CPUS` array
     */
    fn add_cpu(cpu: Self) -> &'static mut Self {
        let cpu_id = cpu.id();

        unsafe {
            assert!(SM_ALL_CPUS.get(cpu_id as usize).is_none());

            SM_ALL_CPUS.push(cpu);
            &mut SM_ALL_CPUS[cpu_id as usize]
        }
    }
}

/**
 * Interface on which the `Cpu` relies to obtain information or throw
 * initialization for hardware CPU
 */
pub trait THwCpu {
    /**
     * Constructs an `HwCpu` which identifies the BSP CPU
     */
    fn new_bsp() -> Self;

    /**
     * Constructs an `HwCpu` which identifies an AP CPU
     */
    fn new_ap() -> Self;

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
    fn current_id() -> CpuId;

    /**
     * Returns the hardware `CpuId` of this `HwCpu`
     */
    fn id(&self) -> CpuId;

    /**
     * Returns the base frequency in Hz of this `Cpu`
     */
    fn base_frequency(&self) -> u64;

    /**
     * Returns the maximum frequency in Hz of this `Cpu`
     */
    fn max_frequency(&self) -> u64;

    /**
     * Returns the bus frequency in Hz of this `Cpu`
     */
    fn bus_frequency(&self) -> u64;

    /**
     * Returns whether this `Cpu` have hardware interrupts enabled
     */
    fn are_interrupts_enabled(&self) -> bool;
}
