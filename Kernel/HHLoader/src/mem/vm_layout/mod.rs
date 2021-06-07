/*! Virtual memory layout randomizer */

use shared::{
    addr::{
        align_down,
        virt::VirtAddr,
        Address
    },
    info::vm_layout::{
        VMLayout,
        VMLayoutArea
    },
    logger::{
        debug,
        trace
    }
};

use crate::{
    loader::loader_core_preload_cache,
    mem::{
        phys::phys_total_memory,
        vm_layout::components::{
            VMComponent,
            VMComponents
        }
    }
};

pub mod components;

/* Kernel space begin (192TiB) */
const KERN_SPACE_BEGIN: usize = 0x0000_c000_0000_0000;

/* global layout returned by <vml_core_layout()> */
static mut KERNEL_VM_LAYOUT: Option<VMLayout> = None;

/**
 * Randomizes the virtual memory layout for the Kernel's core
 */
pub fn vml_randomize_core_layout(necessary_bitmap_pages: usize) {
    assert!(unsafe { KERNEL_VM_LAYOUT.is_none() });

    /* obtain the pairs of VMComponent:VMLayoutArea randomly disposed, then use
     * the VMComponent value to sort them into the right order, as expected by
     * the <VMLayout> constructor.
     *
     * NOTE that the <VMComponent::as_value()> must return the ordinal value for
     * each variant to match the VMLayout constructor arguments position
     */
    let mut vm_areas = vml_randomize_components(necessary_bitmap_pages);
    vm_areas.sort_unstable_by(|vma_1, vma_2| vma_1.0.as_value().cmp(&vma_2.0.as_value()));

    /* construct the Kernel text area, which is the only area not randomizable */
    let kern_text_area = {
        let core_preload_cache = loader_core_preload_cache();
        VMLayoutArea::new(core_preload_cache.load_address(),
                          core_preload_cache.load_size())
    };

    /* finally construct the <VMLayout> */
    let vm_layout = VMLayout::new(kern_text_area,
                                  vm_areas[0].1,
                                  vm_areas[1].1,
                                  vm_areas[2].1,
                                  vm_areas[3].1,
                                  vm_areas[4].1,
                                  vm_areas[5].1);
    debug!("\n{}", vm_layout);

    /* store the value into the global */
    unsafe {
        KERNEL_VM_LAYOUT = Some(vm_layout);
    }
}

/**
 * Returns the immutable reference to the randomized <VMLayout>
 */
pub fn vml_core_layout() -> &'static VMLayout {
    assert!(unsafe { KERNEL_VM_LAYOUT.is_some() });

    unsafe { KERNEL_VM_LAYOUT.as_ref().unwrap() }
}

/**
 * Randomizes and returns the VMComponents for the Kernel core
 */
fn vml_randomize_components(necessary_bitmap_pages: usize)
                            -> [(VMComponent, VMLayoutArea); VMComponent::COUNT] {
    /* calculate the remaining VM space removing the Kernel text */
    let kern_space_size =
        loader_core_preload_cache().load_address() - VirtAddr::new(KERN_SPACE_BEGIN);

    /* constructs a VM components iterator & randomizer, which calculates the
     * size for each one keeping in mind the occupation of the other components
     * which must have a fixed size according to the given parameters
     */
    let vm_components = VMComponents::new(necessary_bitmap_pages,
                                          kern_space_size.as_usize(),
                                          phys_total_memory());

    /* prepare the iteration values */
    let mut vm_area_address = VirtAddr::new(KERN_SPACE_BEGIN);
    let mut vm_areas = [(VMComponent::None, VMLayoutArea::new_zero()),
                        (VMComponent::None, VMLayoutArea::new_zero()),
                        (VMComponent::None, VMLayoutArea::new_zero()),
                        (VMComponent::None, VMLayoutArea::new_zero()),
                        (VMComponent::None, VMLayoutArea::new_zero()),
                        (VMComponent::None, VMLayoutArea::new_zero())];

    /* iterate now the randomized components to construct the <VMLayoutArea>s */
    let mut total_alignment_diff = 0;
    for (i, vm_component) in vm_components.enumerate() {
        trace!("Extracted VMComponent: {}", vm_component);

        /* construct the VMLayout area aligning his size and address */
        let vm_area = vml_place_vm_component(vm_component,
                                             &mut vm_area_address,
                                             &mut total_alignment_diff);

        /* put into the vector the component and the area returned */
        vm_areas[i] = (vm_component, vm_area);
    }

    vm_areas
}

/**
 * Calculates the address and the size for the given <VMComponent>
 */
fn vml_place_vm_component(vm_component: VMComponent,
                          vm_area_address: &mut VirtAddr,
                          total_alignment_diff: &mut usize)
                          -> VMLayoutArea {
    let current_vm_area_address = vm_area_address.clone();

    /* align up the current address with the alignment of the current component,
     * then check the current difference between the aligned and the original
     * address
     */
    let vma_aligned_addr = current_vm_area_address.align_up(vm_component.alignment());
    let alignment_diff = (vma_aligned_addr - current_vm_area_address).as_usize();

    /* update the total alignment and keep a clone for us */
    let total_alignment_diff = {
        *total_alignment_diff += alignment_diff;
        *total_alignment_diff
    };

    /* shrink if possible the area with the accumulated size */
    let size = if total_alignment_diff > 0 && vm_component.is_shrinkable() {
        align_down(vm_component.size() - total_alignment_diff, vm_component.alignment())
    } else {
        vm_component.size()
    };

    debug!("{:x} -> {:x} : {:x}", current_vm_area_address, vma_aligned_addr, size);

    /* update out the address for the next VMComponent */
    *vm_area_address += size + alignment_diff;

    /* return the constructed VMLayoutArea */
    VMLayoutArea::new(vma_aligned_addr, size)
}
