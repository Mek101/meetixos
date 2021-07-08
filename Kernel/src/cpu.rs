/*! Kernel CPU management */

use alloc::vec::Vec;

use sync::mutex::{
    guard::MutexDataGuard,
    spin::RawSpinMutex,
    Mutex
};

use crate::arch::cpu::HwCpu;

const C_MAX_CPUS: usize = 8;

static mut SM_ALL_CPUS: [Option<Cpu>; C_MAX_CPUS] =
    [None, None, None, None, None, None, None, None];

pub type CpuId = u16;

pub struct Cpu {
    m_inner: Mutex<RawSpinMutex, CpuInner>
}

impl Cpu {
    pub fn early_init() {
        let bsp_cpu =
            Self { m_inner: Mutex::const_new(CpuInner { m_hw_cpu: HwCpu::new_bsp() }) };

        Self::register_cpu(bsp_cpu);
    }

    pub fn init_ap() {
        let ap_cpu =
            Self { m_inner: Mutex::const_new(CpuInner { m_hw_cpu: HwCpu::new_ap() }) };

        Self::register_cpu(ap_cpu);
    }

    pub fn id(&self) -> CpuId {
        self.m_inner.lock().m_hw_cpu.smp_id()
    }

    pub fn current() -> &'static Self {
        let cpu_id = HwCpu::current_id();

        unsafe {
            SM_ALL_CPUS[cpu_id as usize].as_ref().unwrap_or_else(|| {
                                                     panic!("Requested an unregistered \
                                                             Cpu. cpu_id: {}",
                                                            cpu_id)
                                                 })
        }
    }

    fn register_cpu(cpu: Self) {
        unsafe {
            let cpu_id = cpu.id();

            assert!(SM_ALL_CPUS[cpu_id as usize].is_none());

            SM_ALL_CPUS[cpu_id as usize] = Some(cpu);
        }
    }
}

struct CpuInner {
    m_hw_cpu: HwCpu
}

pub trait HwCpuBase {
    fn new_bsp() -> Self;
    fn new_ap() -> Self;
    fn id(&self) -> CpuId;
    fn current_id() -> CpuId;
}
