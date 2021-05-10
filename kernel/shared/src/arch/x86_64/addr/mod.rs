/*! x86_64 virtual & physical address implementation */

pub use phys::X64PhysAddr as HwPhysAddr;
pub use virt::X64VirtAddr as HwVirtAddr;

mod phys;
mod virt;
