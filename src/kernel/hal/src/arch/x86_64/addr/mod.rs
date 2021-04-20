/*! # x86_64 Virtual & Physical Address
 *
 * Implements the abstraction of the memory addresses into his two different
 * types: physical and virtual
 */

pub use phys::X64PhysAddr as HwPhysAddr;
pub use virt::X64VirtAddr as HwVirtAddr;

mod phys;
mod virt;
