/*! Virtual memory layout randomizer */

use shared::{
    addr::{
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

    let kern_space_size =
        loader_core_preload_cache().load_address() - VirtAddr::new(KERN_SPACE_BEGIN);

    let vm_components = VMComponents::new(necessary_bitmap_pages,
                                          kern_space_size.as_usize(),
                                          phys_total_memory());

    let mut vm_areas = [(VMComponent::None, VMLayoutArea::new_zero()),
                        (VMComponent::None, VMLayoutArea::new_zero()),
                        (VMComponent::None, VMLayoutArea::new_zero()),
                        (VMComponent::None, VMLayoutArea::new_zero()),
                        (VMComponent::None, VMLayoutArea::new_zero()),
                        (VMComponent::None, VMLayoutArea::new_zero())];
    let mut vm_start = KERN_SPACE_BEGIN;
    for (i, vm_component) in vm_components.enumerate() {
        let vml_addr = align_up(vm_start, vm_component.alignment());

        let diff = vml_addr - vm_start;
        let size = if diff > 0 && vm_component.is_resizable() {
            vm_component.size() - diff
        } else {
            vm_component.size()
        };

        vm_areas[i] = (vm_component, VMLayoutArea::new(VirtAddr::new(vml_addr), size));
        vm_start += size;
    }

    vm_areas.sort_unstable_by(|vma_1, vma_2| vma_1.0.as_value().cmp(&vma_2.0.as_value()));

    let kern_text_area = {
        let core_preload_cache = loader_core_preload_cache();
        VMLayoutArea::new(core_preload_cache.load_address(),
                          core_preload_cache.load_size())
    };

    let vm_layout = VMLayout::new(kern_text_area,
                                  vm_areas[0].1,
                                  vm_areas[1].1,
                                  vm_areas[2].1,
                                  vm_areas[3].1,
                                  vm_areas[4].1,
                                  vm_areas[5].1);
    info!("\n{}", vm_layout);

    unsafe {
        KERNEL_VM_LAYOUT = Some(vm_layout);
    }
}

pub fn vml_core_layout() -> &'static VMLayout {
    assert!(unsafe { KERNEL_VM_LAYOUT.is_some() });

    unsafe { KERNEL_VM_LAYOUT.as_ref().unwrap() }
}

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
    const COUNT: usize = 6;

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

    fn alignment(&self) -> usize {
        match self {
            Self::KernHeap(_)
            | Self::PhysMemBitmap(_)
            | Self::PhysMemMapping(_)
            | Self::PageCache(_) => Page4KiB::SIZE,
            Self::KernStack(_) | Self::TmpMapping(_) => Page2MiB::SIZE,
            _ => panic!("Tried to obtain alignment of `None` VMComponent")
        }
    }

    fn is_resizable(&self) -> bool {
        match self {
            Self::PhysMemBitmap(_)
            | Self::PhysMemMapping(_)
            | Self::KernStack(_)
            | Self::TmpMapping(_) => false,
            Self::KernHeap(_) | Self::PageCache(_) => true,
            _ => panic!("Tried to obtain resizable flags of `None` VMComponent")
        }
    }

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
        match raw_value.0 {
            0 => Self::PhysMemBitmap(raw_value.1),
            1 => Self::PhysMemMapping(raw_value.1),
            2 => Self::KernStack(raw_value.1),
            3 => Self::KernHeap(raw_value.1),
            4 => Self::PageCache(raw_value.1),
            5 => Self::TmpMapping(raw_value.1),
            _ => panic!("Tried to construct an invalid VMComponent")
        }
    }
}

struct VMComponents {
    m_components: [VMComponent; VMComponent::COUNT],

    m_extracted: [bool; VMComponent::COUNT],
    m_extracted_count: usize,
    m_random: Random
}

impl VMComponents {
    fn new(bitmap_pages_count: usize,
           kern_space_size: usize,
           total_memory: usize)
           -> Self {
        let phys_mem_bitmap_size = bitmap_pages_count * Page4KiB::SIZE;
        let phys_mem_map_size = total_memory;
        let kern_stack_size = Page2MiB::SIZE;
        let tmp_map_size = Page2MiB::SIZE;

        let last_components_size = (kern_space_size
                                    - kern_stack_size
                                    - phys_mem_bitmap_size
                                    - phys_mem_map_size
                                    - tmp_map_size)
                                   / 2;

        Self { m_components: [VMComponent::PhysMemBitmap(phys_mem_bitmap_size),
                              VMComponent::PhysMemMapping(phys_mem_map_size),
                              VMComponent::KernStack(kern_stack_size),
                              VMComponent::KernHeap(last_components_size),
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
