/*! Kernel CPU management */

use crate::arch::hw_cpu::HwCpu;

/* Currently we support at max 8 CPUs */
const C_MAX_CPUS: usize = 8;

/* All the CPUs descriptors currently active */
static mut SM_ALL_CPUS: [Option<Cpu>; C_MAX_CPUS] =
    [None, None, None, None, None, None, None, None];

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
     * Returns the reference to the current `Cpu`
     */
    pub fn current() -> &'static Self {
        Self::by_id(HwCpu::current_id())
    }

    /**
     * Returns the reference to the given `cpu_id` `Cpu`
     */
    pub fn by_id(cpu_id: CpuId) -> &'static Self {
        assert!((cpu_id as usize) < C_MAX_CPUS);
        unsafe {
            SM_ALL_CPUS[cpu_id as usize].as_ref().unwrap_or_else(|| {
                                                     panic!("Requested an unregistered \
                                                             Cpu with id: {}",
                                                            cpu_id)
                                                 })
        }
    }
}

impl Cpu /* Getters */ {
    /**
     * Returns the `CpuId` of this `Cpu`
     */
    pub fn id(&self) -> CpuId {
        self.m_hw_cpu.id()
    }
}

impl Cpu /* Privates */ {
    /**
     * Stores the given `Cpu` into the `SM_ALL_CPUS` array
     */
    fn add_cpu(cpu: Self) -> &'static mut Self {
        let cpu_id = cpu.id();

        unsafe {
            assert!(SM_ALL_CPUS[cpu_id as usize].is_none());

            SM_ALL_CPUS[cpu_id as usize] = Some(cpu);
            SM_ALL_CPUS[cpu_id as usize].as_mut().unwrap()
        }
    }
}

/**
 * Interface on which the `Cpu` relies to obtain information or throw
 * initialization for hardware CPU
 */
pub trait HwCpuBase {
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
     * Halts this `HwCpu`
     */
    fn do_halt(&self);

    /**
     * Returns the `CpuId` of the executing `Cpu`
     */
    fn current_id() -> CpuId;

    /**
     * Returns the hardware `CpuId` of this `HwCpu`
     */
    fn id(&self) -> CpuId;
}
