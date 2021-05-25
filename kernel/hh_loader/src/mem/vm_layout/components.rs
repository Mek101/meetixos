/*! VM components manager */

use core::fmt;

use shared::{
    addr::{
        align_down,
        align_up
    },
    dbg::{
        dbg_display_size,
        KIB
    },
    mem::paging::{
        Page2MiB,
        Page4KiB,
        PageSize
    },
    random::Random
};

/**
 * Lists the VM components that could be randomized
 */
#[derive(Copy, Clone)]
pub(super) enum VMComponent {
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
     * `VMComponent`s which supports shrinking
     */
    const SHRINKABLE_COMPONENTS: [VMComponent; 2] =
        [VMComponent::PageCache(0), VMComponent::KernHeap(0)];

    /**
     * Amount of valid variants (excluded `None`)
     */
    pub(crate) const COUNT: usize = 6;

    /**
     * Returns the value of the current variant
     */
    pub fn size(&self) -> usize {
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
    pub fn alignment(&self) -> usize {
        match self {
            Self::KernHeap(_)
            | Self::PhysMemBitmap(_)
            | Self::KernStack(_)
            | Self::PageCache(_) => Page4KiB::SIZE,
            Self::PhysMemMapping(_) | Self::TmpMapping(_) => Page2MiB::SIZE,
            _ => panic!("Tried to obtain alignment of `None` VMComponent")
        }
    }

    /**
     * Returns whether the current variant represents a VM component that
     * could be shrinked
     */
    pub fn is_shrinkable(&self) -> bool {
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
    pub fn as_value(&self) -> usize {
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

impl fmt::Display for VMComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (variant_name, size) = match *self {
            Self::KernHeap(value) => ("KernHeap", value),
            Self::KernStack(value) => ("KernStack", value),
            Self::PhysMemBitmap(value) => ("PhysMemBitmap", value),
            Self::PhysMemMapping(value) => ("PhysMemMapping", value),
            Self::PageCache(value) => ("PageCache", value),
            Self::TmpMapping(value) => ("TmpMapping", value),
            Self::None => ("None", 0)
        };

        write!(f, "{}({})", variant_name, dbg_display_size(size))
    }
}

/**
 * `VMComponent` randomizer and size calculator
 */
pub(super) struct VMComponents {
    m_precalculated_components: [VMComponent; VMComponent::COUNT],

    m_random: Random,
    m_last_shrinkable_component: VMComponent,

    m_extracted: [bool; VMComponent::COUNT],
    m_extracted_count: usize
}

impl VMComponents {
    /**
     * Constructs a `VMComponents` and calculates the size for each
     * `VMComponent`
     */
    pub(crate) fn new(bitmap_pages_count: usize,
                      kern_space_size: usize,
                      total_memory: usize)
                      -> Self {
        /* calculate the size of the VM components from the given parameters */
        let bitmap_size = bitmap_pages_count * Page4KiB::SIZE;
        let phys_mem_mapping_size = align_up(total_memory, Page2MiB::SIZE);

        /* TODO these can be extracted from cmdline? */
        let kern_stack_size = 64 * KIB;
        let tmp_map_size = Page2MiB::SIZE;

        /* shrinkable components fits the remaining space in the same size */
        let shrinkable_component_size =
            align_down((kern_space_size
                        - bitmap_size
                        - phys_mem_mapping_size
                        - kern_stack_size
                        - tmp_map_size)
                       / VMComponent::SHRINKABLE_COMPONENTS.len(),
                       Page4KiB::SIZE);

        /* extract one of the shrinkable components to put it at the end of the VM
         * layout, in order to be able to handle alignment discard without
         * overlapping following areas
         */
        let random_gen = Random::new();
        let last_shrinkable_component =
            VMComponent::SHRINKABLE_COMPONENTS[random_gen.randomize_usize() % 2];

        /* mark as already extracted the last component */
        let extracted = {
            let mut extracted_buf = [false; VMComponent::COUNT];

            extracted_buf[last_shrinkable_component.as_value()] = true;
            extracted_buf
        };

        /* keep <m_precalculated_components> in the same order of the enum */
        Self { m_precalculated_components:
                   [VMComponent::KernHeap(shrinkable_component_size),
                    VMComponent::KernStack(kern_stack_size),
                    VMComponent::PhysMemBitmap(bitmap_size),
                    VMComponent::PhysMemMapping(phys_mem_mapping_size),
                    VMComponent::PageCache(shrinkable_component_size),
                    VMComponent::TmpMapping(tmp_map_size)],
               m_random: random_gen,
               m_last_shrinkable_component: last_shrinkable_component,
               m_extracted: extracted,
               m_extracted_count: 0 }
    }
}

impl Iterator for VMComponents {
    type Item = VMComponent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.m_extracted_count < VMComponent::COUNT {
            /* when the last component must be extracted use the special shrinkable
             * component from the constructor, to be able to catch alignment discard of
             * the previous components
             */
            if self.m_extracted_count == VMComponent::COUNT - 1 {
                self.m_extracted_count += 1;

                let next_component = self.m_last_shrinkable_component.as_value();
                return Some(self.m_precalculated_components[next_component]);
            } else {
                loop {
                    /* extract a new component index randomly */
                    let next_component =
                        self.m_random.randomize_usize() % VMComponent::COUNT;

                    if !self.m_extracted[next_component] {
                        self.m_extracted[next_component] = true;
                        self.m_extracted_count += 1;

                        return Some(self.m_precalculated_components[next_component]);
                    }
                }
            }
        } else {
            return None;
        }
    }
}
