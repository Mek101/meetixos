/*! x86_64 CPU management implementation */

use crate::cpu::{
    CpuId,
    HwCpuBase
};

pub struct HwCpu;

impl HwCpuBase for HwCpu {
    fn new_bsp() -> Self {
        Self
    }

    fn new_ap() -> Self {
        Self
    }

    fn id(&self) -> CpuId {
        0 /* TODO APIC */
    }

    fn current_id() -> CpuId {
        0 /* TODO APIC */
    }
}
