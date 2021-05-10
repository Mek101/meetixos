/*! Virtual memory layout randomizer */

use shared::{
    infos::vm_layout::VMLayout,
    logger::info
};

pub fn randomize_vm_layout_for_core() -> VMLayout {
    let vm_layout = VMLayout::new_zero();
    info!("\n{}", vm_layout);
    vm_layout
}
