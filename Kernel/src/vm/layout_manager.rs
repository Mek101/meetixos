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
    pub fn new_randomized(phys_mem_size: usize) -> Self {
        let _sized_layout_components = Self::size_components(phys_mem_size);
        Self::new(&[])
    }

    pub fn new_plain(phys_mem_size: usize) -> Self {
        /* obtain the ordered and sized <LayoutComponents>, then place them in VM */
        let sized_layout_components = Self::size_components(phys_mem_size);
        let vm_layout_ranges = Self::place_components(&sized_layout_components);

        /* construct the LayoutManager */
        Self::new(&vm_layout_ranges)
    }

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
                   let start_virt_addr: VirtAddr =
                       unsafe { &__kernel_text_begin as *const _ as usize }.into();
                   let end_virt_addr: VirtAddr =
                       unsafe { &__kernel_text_end as *const _ as usize }.into();

                   /* return the Range<VirtAddr> of the kernel-text */
                   Range { start: start_virt_addr,
                           end: end_virt_addr }
               } }
    }
}

impl LayoutManager /* Getters */ {
    pub fn phys_mem_mapping_range(&self) -> &Range<VirtAddr> {
        &self.m_phys_mem_mapping_range
    }

    pub fn tmp_mem_mapping_range(&self) -> &Range<VirtAddr> {
        &self.m_tmp_mem_mapping_range
    }

    pub fn kern_regions_range(&self) -> &Range<VirtAddr> {
        &self.m_kern_regions_range
    }

    pub fn fs_page_cache_range(&self) -> &Range<VirtAddr> {
        &self.m_fs_page_cache_range
    }

    pub fn kern_text_range(&self) -> &Range<VirtAddr> {
        &self.m_kern_text_range
    }
}

impl LayoutManager /* Privates */ {
    fn size_components(phys_mem_size: usize)
                       -> [LayoutComponent; LayoutComponent::COUNT] {
        /* since the kernel uses a memory mapping paging strategy we need all the
         * physical memory mapped somewhere. Doing so all the physical memory is
         * accessible with <phys_addr_to_access + chosen_virt_addr>.
         * So reserve a virtual range where put the mapping.
         * Use 2MiB alignment to use 2MiB pages for mapping
         */
        let phys_mem_mapping_size = align_up(phys_mem_size, Page2MiB::SIZE);

        /* reserve 2 MiB to be able to map up to 512 4KiB pages or one huge 2MiB page */
        let tmp_mapping_size = Page2MiB::SIZE;

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
}

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

impl LayoutComponent /* Constructors */ {
    fn from_index(index: usize) -> Self {
        match index {
            0 => Self::PhysMemMapping { m_size: 0 },
            1 => Self::TmpMapping { m_size: 0 },
            2 => Self::KernRegions { m_size: 0 },
            3 => Self::FsPageCache { m_size: 0 },
            _ => Self::None
        }
    }
}

impl LayoutComponent /* Getters */ {
    fn virt_size(&self) -> usize {
        match self {
            Self::PhysMemMapping { m_size }
            | Self::TmpMapping { m_size }
            | Self::KernRegions { m_size }
            | Self::FsPageCache { m_size } => *m_size,
            _ => panic!("Tried to obtain size from a None LayoutComponent")
        }
    }

    fn alignment(&self) -> usize {
        match self {
            Self::TmpMapping { .. }
            | Self::KernRegions { .. }
            | Self::FsPageCache { .. } => Page4KiB::SIZE,
            Self::PhysMemMapping { .. } => Page2MiB::SIZE,
            _ => panic!("Tried to obtain alignment from a None LayoutComponent")
        }
    }

    fn is_shrinkable(&self) -> bool {
        match self {
            Self::PhysMemMapping { .. } | Self::TmpMapping { .. } => false,
            Self::KernRegions { .. } | Self::FsPageCache { .. } => true,
            _ => panic!("Tried to obtain shrinkable from a None LayoutComponent")
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
