/*! Virtual memory layout randomizer */

use shared::{
    addr::{
        align_down,
        align_up,
        virt::VirtAddr,
        Address
    },
    info::vm_layout::{
        VMLayout,
        VMLayoutArea
    },
    logger::info,
    mem::paging::{
        Page2MiB,
        Page4KiB,
        PageSize
    },
    random::Random
};

use crate::{
    loader::loader_core_preload_cache,
    mem::phys::phys_total_memory
};

/* kernel space begin (192TiB) */
const KERN_SPACE_BEGIN: usize = 0x0000_c000_0000_0000;

/* global layout returned by <vml_core_layout()> */
static mut KERNEL_VM_LAYOUT: Option<VMLayout> = None;

/**
 * Randomizes the virtual memory layout for the kernel's core
 */
pub fn vml_randomize_core_layout(necessary_bitmap_pages: usize) {
    assert!(unsafe { KERNEL_VM_LAYOUT.is_none() });

    /* calculate how many kernel VM is available removing the kernel text */
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
    let mut vm_area_address = KERN_SPACE_BEGIN;
    let mut vm_areas = [(VMComponent::None, VMLayoutArea::new_zero()),
                        (VMComponent::None, VMLayoutArea::new_zero()),
                        (VMComponent::None, VMLayoutArea::new_zero()),
                        (VMComponent::None, VMLayoutArea::new_zero()),
                        (VMComponent::None, VMLayoutArea::new_zero()),
                        (VMComponent::None, VMLayoutArea::new_zero())];

    /* iterate now the randomized components to construct the <VMLayoutArea>s */
    let mut accumulated_diff = 0;
    for (i, vm_component) in vm_components.enumerate() {
        /* align up the current address with the alignment of the current component */
        let vma_aligned_addr = align_up(vm_area_address, vm_component.alignment());

        /* check the current difference between the aligned and the original address */
        let alignment_diff = vma_aligned_addr - vm_area_address;
        accumulated_diff += alignment_diff;

        /* shrink if possible the area with the accumulated size */
        let size = if accumulated_diff > 0 && vm_component.is_shrinkable() {
            align_down(vm_component.size() - accumulated_diff, vm_component.alignment())
        } else {
            vm_component.size()
        };

        /* place the new area into the vector and go to the next area address */
        vm_areas[i] =
            (vm_component, VMLayoutArea::new(VirtAddr::new(vma_aligned_addr), size));
        vm_area_address += size;
    }

    /* sort the areas by his integer value */
    vm_areas.sort_unstable_by(|vma_1, vma_2| vma_1.0.as_value().cmp(&vma_2.0.as_value()));

    /* construct the kernel text area, which is the only area not randomizable */
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
    info!("\n{}", vm_layout);

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
 * Lists the VM components that could be randomized
 */
#[derive(Debug)]
enum VMComponent {
    KernHeap(usize),
    KernStack(usize),
    PhysMemBitmap(usize),
    PhysMemMapping(usize),
    PageCache(usize),
    TmpMapping(usize),
    None
}

impl VMComponent {
    /**
     * Amount of valid variants (excluded `None`)
     */
    const COUNT: usize = 6;

    /**
     * Returns the value of the current variant
     */
    fn size(&self) -> usize {
        match self {
            Self::KernHeap(value)
            | Self::KernStack(value)
            | Self::PhysMemBitmap(value)
            | Self::PhysMemMapping(value)
            | Self::PageCache(value)
            | Self::TmpMapping(value) => *value,
            _ => panic!("Tried to obtain size of `None` VMComponent")
        }
    }

    /**
     * Returns the alignment of the current variant
     */
    fn alignment(&self) -> usize {
        match self {
            Self::KernHeap(_) | Self::PhysMemBitmap(_) | Self::PageCache(_) => {
                Page4KiB::SIZE
            },
            Self::KernStack(_) | Self::PhysMemMapping(_) | Self::TmpMapping(_) => {
                Page2MiB::SIZE
            },
            _ => panic!("Tried to obtain alignment of `None` VMComponent")
        }
    }

    /**
     * Returns whether the current variant represents a VM component that
     * could be shrinked
     */
    fn is_shrinkable(&self) -> bool {
        match self {
            Self::PhysMemBitmap(_)
            | Self::PhysMemMapping(_)
            | Self::KernStack(_)
            | Self::TmpMapping(_) => false,
            Self::KernHeap(_) | Self::PageCache(_) => true,
            _ => panic!("Tried to obtain resizable flags of `None` VMComponent")
        }
    }

    /**
     * Returns the cardinal value of the current variant
     */
    fn as_value(&self) -> usize {
        /* NOTE: Here must be kept the exactly same order of the VMLayout's
         *       constructor otherwise the sorting would not work
         */
        match self {
            Self::KernHeap(_) => 0,
            Self::KernStack(_) => 1,
            Self::PhysMemBitmap(_) => 2,
            Self::PhysMemMapping(_) => 3,
            Self::PageCache(_) => 4,
            Self::TmpMapping(_) => 5,
            _ => panic!("Tried to obtain ordinal value of `None` VMComponent")
        }
    }
}

impl From<(usize, usize)> for VMComponent {
    fn from(raw_value: (usize, usize)) -> Self {
        /* keep the following values aligned with the as_value method */
        match raw_value.0 {
            0 => Self::KernHeap(raw_value.1),
            1 => Self::KernStack(raw_value.1),
            2 => Self::PhysMemBitmap(raw_value.1),
            3 => Self::PhysMemMapping(raw_value.1),
            4 => Self::PageCache(raw_value.1),
            5 => Self::TmpMapping(raw_value.1),
            _ => panic!("Tried to construct an invalid VMComponent")
        }
    }
}

/**
 * `VMComponent` randomizer and size calculator
 */
struct VMComponents {
    m_components: [VMComponent; VMComponent::COUNT],
    m_extracted: [bool; VMComponent::COUNT],
    m_extracted_count: usize,
    m_random: Random
}

impl VMComponents {
    /**
     * Constructs a `VMComponents` and calculates the size for each
     * `VMComponent`
     */
    fn new(bitmap_pages_count: usize,
           kern_space_size: usize,
           total_memory: usize)
           -> Self {
        let phys_mem_bitmap_size = bitmap_pages_count * Page4KiB::SIZE;
        let phys_mem_map_size = align_up(total_memory, Page2MiB::SIZE);
        let kern_stack_size = Page2MiB::SIZE;
        let tmp_map_size = Page2MiB::SIZE;

        let last_components_size = align_down((kern_space_size
                                               - kern_stack_size
                                               - phys_mem_bitmap_size
                                               - phys_mem_map_size
                                               - tmp_map_size)
                                              / 2,
                                              Page4KiB::SIZE);

        /* keep the order of the <VMComponent> variants */
        Self { m_components: [VMComponent::KernHeap(last_components_size),
                              VMComponent::KernStack(kern_stack_size),
                              VMComponent::PhysMemBitmap(phys_mem_bitmap_size),
                              VMComponent::PhysMemMapping(phys_mem_map_size),
                              VMComponent::PageCache(last_components_size),
                              VMComponent::TmpMapping(tmp_map_size)],
               m_extracted: [false; VMComponent::COUNT],
               m_extracted_count: 0,
               m_random: Random::new() }
    }
}

impl Iterator for VMComponents {
    type Item = VMComponent;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.m_extracted_count < VMComponent::COUNT {
                let next_component =
                    self.m_random.randomize_u16() as usize % VMComponent::COUNT;

                if !self.m_extracted[next_component] {
                    self.m_extracted[next_component] = true;
                    self.m_extracted_count += 1;
                    return Some(VMComponent::from((next_component,
                                                   self.m_components[next_component].size())));
                }
            } else {
                return None;
            }
        }
    }
}
