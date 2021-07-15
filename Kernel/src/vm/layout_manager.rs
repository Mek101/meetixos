/*! Kernel layout manager */

use core::{
    fmt,
    fmt::Display,
    ops::Range
};

use helps::align::{
    align_down,
    align_up
};

use crate::{
    addr::{
        virt_addr::VirtAddr,
        Address
    },
    dbg::{
        display_pretty::DisplaySizePretty,
        print::DbgLevel
    },
    dbg_println,
    dev::random::Random,
    vm::paging::{
        Page2MiB,
        Page4KiB,
        PageSize
    }
};

extern "C" {
    static __kernel_text_begin: usize;
    static __kernel_text_end: usize;
}

/* <false> until <MemManager::init_instance()> is called */
static mut SM_INITIALIZED: bool = false;

/**
 * Kernel VM ranges.
 *
 * It is used as singleton stored into the `MemManager`
 */
pub struct LayoutManager {
    m_phys_mem_mapping_range: Range<VirtAddr>,
    m_tmp_mem_mapping_range: Range<VirtAddr>,
    m_kern_regions_range: Range<VirtAddr>,
    m_fs_page_cache_range: Range<VirtAddr>,
    m_kern_text_range: Range<VirtAddr>
}

impl LayoutManager /* Constants */ {
    /**
     * Kernel space begins at virtual offset of 192TiB
     */
    const KERN_SPACE_BEGIN: usize = 0xffff_c000_0000_0000;
}

impl LayoutManager /* Constructor */ {
    /**
     * Constructs a `LayoutManager` randomizing the `LayoutComponent`s as
     * meltdown mitigation
     */
    pub fn new_randomized(phys_mem_size: usize) -> Self {
        /* obtain the ordered and sized <LayoutComponents> */
        let sized_layout_components = Self::size_components(phys_mem_size);

        /* randomize the order of the LayoutComponent */
        let randomized_layout_components =
            Self::randomize_components(&sized_layout_components);

        /* assign them a virtual-range */
        let unordered_vm_layout_ranges =
            Self::place_components(&randomized_layout_components);

        /* re-order back the ranges as expected by the constructor */
        let ordered_vm_layout_ranges =
            Self::sort_for_constructor(&randomized_layout_components,
                                       &unordered_vm_layout_ranges);

        /* construct the LayoutManager */
        Self::new(&ordered_vm_layout_ranges)
    }

    /**
     * Constructs a plain `LayoutManager` which distributes the
     * `LayoutComponent`s as represented into the `LayoutComponent` enum
     */
    pub fn new_plain(phys_mem_size: usize) -> Self {
        /* obtain the ordered and sized <LayoutComponents> */
        let sized_layout_components = Self::size_components(phys_mem_size);

        /* place the components into the VM */
        let vm_layout_ranges = Self::place_components(&sized_layout_components);

        /* construct the LayoutManager */
        Self::new(&vm_layout_ranges)
    }

    /**
     * Internal constructor called by the previous two functions
     */
    fn new(vm_layout_ranges: &[Range<VirtAddr>]) -> Self {
        /* the LayoutManager is a singleton stored into the <MemManager> */
        unsafe {
            if SM_INITIALIZED {
                panic!("Tried to re-construct the LayoutManager. Use the MemManager!");
            } else {
                SM_INITIALIZED = true;
            }
        }

        /* NOTE: here the ranges are expected in the same order of the fields */
        let mut vm_layout_ranges_it = vm_layout_ranges.iter();

        Self { m_phys_mem_mapping_range:
                   vm_layout_ranges_it.next()
                                      .expect("Missing physical memory mapping range")
                                      .clone(),
               m_tmp_mem_mapping_range:
                   vm_layout_ranges_it.next()
                                      .expect("Missing temporary mapping range")
                                      .clone(),
               m_kern_regions_range:
                   vm_layout_ranges_it.next()
                                      .expect("Missing kernel regions range")
                                      .clone(),
               m_fs_page_cache_range:
                   vm_layout_ranges_it.next()
                                      .expect("Missing filesystem page cache range")
                                      .clone(),
               m_kern_text_range: {
                   /* to avoid to use the special linker symbols every time are needed
                    * the kernel-text ranges, they are stores as Range of VirtAddr into
                    * the <LayoutManager>
                    */
                   Range { start:
                               unsafe { &__kernel_text_begin as *const _ as usize }.into(),
                           end:
                               unsafe { &__kernel_text_end as *const _ as usize }.into() }
               } }
    }
}

impl LayoutManager /* Getters */ {
    /**
     * Returns the virtual `Range` where is mapped all the memory
     */
    pub fn phys_mem_mapping_range(&self) -> &Range<VirtAddr> {
        &self.m_phys_mem_mapping_range
    }

    /**
     * Returns temporary mapped pages `Range`
     */
    pub fn tmp_mem_mapping_range(&self) -> &Range<VirtAddr> {
        &self.m_tmp_mem_mapping_range
    }

    /**
     * Returns the kernel `VmObject`s `Range`
     */
    pub fn kern_regions_range(&self) -> &Range<VirtAddr> {
        &self.m_kern_regions_range
    }

    /**
     * Returns the special kernel regions `Range` where are stored the cache
     * pages from the filesystem I/O
     */
    pub fn fs_page_cache_range(&self) -> &Range<VirtAddr> {
        &self.m_fs_page_cache_range
    }

    /**
     * Returns the kernel-text virtual `Range`
     */
    pub fn kern_text_range(&self) -> &Range<VirtAddr> {
        &self.m_kern_text_range
    }
}

impl LayoutManager /* Privates */ {
    /**
     * Returns all the `LayoutComponent`s with a size
     */
    fn size_components(phys_mem_size: usize)
                       -> [LayoutComponent; LayoutComponent::COUNT] {
        /* reserve 2 MiB to be able to map up to 512 4KiB pages or one huge 2MiB page */
        let tmp_mapping_size = Page2MiB::SIZE;

        /* reserve a part of the kernel's address space to store the mapping of all
         * the physical memory.
         *
         * Since the kernel uses a memory-mapped-paging strategy for all the
         * architectures this reservation is fundamental. The is size is 2MiB aligned
         * because 2MiB huge-pages are used, to avoid too much waste in intermediate
         * page-tables
         */
        let phys_mem_mapping_size = align_up(phys_mem_size, Page2MiB::SIZE);

        /* obtain the remaining virtual space removing the kernel text */
        let rem_vm_kern_space_size = {
            let kern_text_begin_addr =
                unsafe { &__kernel_text_begin as *const _ as usize };

            dbg_println!(DbgLevel::Trace,
                         "KernelSpace: kernSpaceBegin..kernTextBegin ({}..{}) ",
                         VirtAddr::from(Self::KERN_SPACE_BEGIN),
                         VirtAddr::from(kern_text_begin_addr));

            kern_text_begin_addr - Self::KERN_SPACE_BEGIN
        };
        dbg_println!(DbgLevel::Trace,
                     "Available Kernel Space: {} ({:#018x})",
                     rem_vm_kern_space_size.display_pretty(),
                     rem_vm_kern_space_size);

        /* remove from the remaining kernel space size the two components */
        let rem_vm_kern_space_size =
            rem_vm_kern_space_size - phys_mem_mapping_size - tmp_mapping_size;

        /* remaining components receives an equal & shrinkable portion of the layout */
        let shrinkable_components_size = align_down(rem_vm_kern_space_size
                                                    / LayoutComponent::SHRINKABLES.len(),
                                                    Page4KiB::SIZE);

        /* return the components with the size */
        [LayoutComponent::PhysMemMapping { m_size: phys_mem_mapping_size },
         LayoutComponent::TmpMapping { m_size: tmp_mapping_size },
         LayoutComponent::KernRegions { m_size: shrinkable_components_size },
         LayoutComponent::FsPageCache { m_size: shrinkable_components_size }]
    }

    /**
     * Places the given `LayoutComponent`s into VM
     */
    fn place_components(layout_components: &[LayoutComponent])
                        -> [Range<VirtAddr>; LayoutComponent::COUNT] {
        /* alignment mismatching and reset when encountered shrinkable components */
        let mut total_alignment_diff = 0;
        let mut vm_range_addr: VirtAddr = Self::KERN_SPACE_BEGIN.into();

        /* store the ranges in a fixed size array. TODO Range is not Copy */
        let mut layout_ranges =
            [Range::default(), Range::default(), Range::default(), Range::default()];

        /* place <LayoutComponent>s into virtual memory */
        for (i, &layout_component) in layout_components.iter().enumerate() {
            /* obtain the aligned Range<VirtAddr> for the current component */
            layout_ranges[i] = Self::place_component(layout_component,
                                                     &mut vm_range_addr,
                                                     &mut total_alignment_diff);
        }

        layout_ranges
    }

    /**
     * Places the given `LayoutComponent` at the given `VirtAddr` absorbing
     * if possible the eventual alignment discard produced by the alignment
     */
    fn place_component(layout_component: LayoutComponent,
                       vm_range_addr: &mut VirtAddr,
                       total_alignment_diff: &mut usize)
                       -> Range<VirtAddr> {
        /* obtain the aligned up VirtAddr for the current component */
        let aligned_up_addr = vm_range_addr.align_up(layout_component.alignment());

        /* the alignment could have produced a difference. So update the counter */
        *total_alignment_diff += *aligned_up_addr - **vm_range_addr;

        /* here we obtain the LayoutComponent size, which could be down-aligned with
         * the accumulated differences from the previous (and the current)
         * up-alignment wastes */
        let component_size =
            if *total_alignment_diff > 0 && layout_component.is_shrinkable() {
                /* copy the alignment diff and reset it */
                let prev_alignment_diff = *total_alignment_diff;
                *total_alignment_diff = 0;

                align_down(layout_component.virt_size() - prev_alignment_diff,
                           layout_component.alignment())
            } else {
                /* return the size as is */
                layout_component.virt_size()
            };

        /* update the VirtAddr for the next component if any */
        *vm_range_addr = aligned_up_addr.offset(component_size);
        dbg_println!(DbgLevel::Trace,
                     "{} ({}..{})",
                     layout_component,
                     aligned_up_addr,
                     *vm_range_addr);

        Range { start: aligned_up_addr,
                end: *vm_range_addr }
    }

    /**
     * Randomizes the `LayoutComponent`s order, in order to place them into
     * VM in a pseudo random order
     */
    fn randomize_components(sized_layout_components: &[LayoutComponent])
                            -> [LayoutComponent; LayoutComponent::COUNT] {
        let random_gen = Random::new();

        /* keep a bitmap of the extracted components */
        let mut extracted_components = [false; LayoutComponent::COUNT];

        /* extract one of the shrinkable component because of alignment mismatching
         * we must be able to eat-up the difference without overlapping the following
         * regions (or worse: the kernel text)
         */
        let last_shrinkable_component = {
            /* extract the first available shrinkable component */
            let component_index =
                random_gen.generate_u64() as usize % LayoutComponent::SHRINKABLES.len();

            /* extract the LayoutComponent from the SHRINKABLES, then return the same
             * LayoutComponent but with the valid size
             */
            let unsized_component = LayoutComponent::SHRINKABLES[component_index];
            sized_layout_components[unsized_component.as_index()]
        };

        /* mark as extracted into the bitmap */
        extracted_components[last_shrinkable_component.as_index()] = true;

        /* extract now the others randomly. NOTE the COUNT - 1! we want ot extract
         * all the component less the last, because we already have it
         */
        let mut layout_components = [LayoutComponent::None; LayoutComponent::COUNT];
        for index in 0..LayoutComponent::COUNT - 1 {
            loop {
                /* generate the next random number */
                let next_index =
                    random_gen.generate_u64() as usize % sized_layout_components.len();

                /* mark as extracted */
                if !extracted_components[next_index] {
                    extracted_components[next_index] = true;
                    layout_components[index] = sized_layout_components[next_index];
                    break;
                }
            }
        }

        /* store the last layout-component */
        layout_components[layout_components.len() - 1] = last_shrinkable_component;
        layout_components
    }

    /**
     * Restores the order of the `Range`s as expected by the `LayoutManager`
     * constructor but keeping the assigned address
     */
    fn sort_for_constructor(layout_components: &[LayoutComponent],
                            vm_layout_ranges: &[Range<VirtAddr>])
                            -> [Range<VirtAddr>; LayoutComponent::COUNT] {
        assert_eq!(layout_components.len(), vm_layout_ranges.len());

        /* collect into an array of pairs the components associated with the range */
        let mut components_ranges_pair = [(LayoutComponent::None, Range::default()),
                                          (LayoutComponent::None, Range::default()),
                                          (LayoutComponent::None, Range::default()),
                                          (LayoutComponent::None, Range::default())];
        for i in 0..LayoutComponent::COUNT {
            components_ranges_pair[i] =
                (layout_components[i].clone(), vm_layout_ranges[i].clone());
        }

        /* sort the array as expected by the constructor */
        components_ranges_pair.sort_unstable_by(|prev, next| {
                                  prev.0.as_index().cmp(&next.0.as_index())
                              });

        /* fill the vm_ranges array with the ranges and return them */
        let mut vm_ranges =
            [Range::default(), Range::default(), Range::default(), Range::default()];
        for i in 0..LayoutComponent::COUNT {
            vm_ranges[i] = components_ranges_pair[i].1.clone();
        }
        vm_ranges
    }
}

/**
 * Identifier of the VM components
 */
#[derive(Copy, Clone)]
enum LayoutComponent {
    PhysMemMapping {
        m_size: usize
    },
    TmpMapping {
        m_size: usize
    },
    KernRegions {
        m_size: usize
    },
    FsPageCache {
        m_size: usize
    },
    None
}

impl LayoutComponent /* Constants */ {
    /**
     * Amount of valid `LayoutComponent`s without `None`
     */
    const COUNT: usize = [Self::PhysMemMapping { m_size: 0 },
                          Self::TmpMapping { m_size: 0 },
                          Self::KernRegions { m_size: 0 },
                          Self::FsPageCache { m_size: 0 }].len();

    /**
     * Shrinkable `LayoutComponent`s
     */
    const SHRINKABLES: &'static [Self] =
        &[Self::KernRegions { m_size: 0 }, Self::FsPageCache { m_size: 0 }];
}

impl LayoutComponent /* Getters */ {
    /**
     * Returns the size of the component
     */
    fn virt_size(&self) -> usize {
        match self {
            Self::PhysMemMapping { m_size }
            | Self::TmpMapping { m_size }
            | Self::KernRegions { m_size }
            | Self::FsPageCache { m_size } => *m_size,
            _ => panic!("Tried to obtain size from a None LayoutComponent")
        }
    }

    /**
     * Returns the required alignment for this `LayoutComponent`
     */
    fn alignment(&self) -> usize {
        match self {
            Self::TmpMapping { .. }
            | Self::KernRegions { .. }
            | Self::FsPageCache { .. } => Page4KiB::SIZE,
            Self::PhysMemMapping { .. } => Page2MiB::SIZE,
            _ => panic!("Tried to obtain alignment from a None LayoutComponent")
        }
    }

    /**
     * Returns whether this `LayoutComponent` support shrinking
     */
    fn is_shrinkable(&self) -> bool {
        match self {
            Self::PhysMemMapping { .. } | Self::TmpMapping { .. } => false,
            Self::KernRegions { .. } | Self::FsPageCache { .. } => true,
            _ => panic!("Tried to obtain shrinkable from a None LayoutComponent")
        }
    }

    /**
     * Returns the index of this `LayoutComponent`
     */
    fn as_index(&self) -> usize {
        /* NOTE: keep these indexes aligned with the order */
        match self {
            Self::PhysMemMapping { .. } => 0,
            Self::TmpMapping { .. } => 1,
            Self::KernRegions { .. } => 2,
            Self::FsPageCache { .. } => 3,
            _ => panic!("Tried to obtain the index of a None LayoutComponent")
        }
    }
}

impl Display for LayoutComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (variant, size) = match *self {
            Self::PhysMemMapping { m_size } => ("PhysMemMapping", m_size),
            Self::TmpMapping { m_size } => ("TmpMapping", m_size),
            Self::KernRegions { m_size } => ("KernRegions", m_size),
            Self::FsPageCache { m_size } => ("FsPageCache", m_size),
            Self::None => ("None", 0)
        };

        write!(f, "LayoutComponent::{} {{ m_size: {} }}", variant, size.display_pretty())
    }
}
