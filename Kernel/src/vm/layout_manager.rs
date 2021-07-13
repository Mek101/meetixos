/*! Kernel layout manager */

use core::ops::Range;

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
    pub fn new_randomized(_phys_mem_size: usize) -> Self {
        Self::new(&[])
    }

    pub fn new_plain(phys_mem_size: usize) -> Self {
        /* obtain the ordered and sized <LayoutComponents> */
        let layout_components = Self::prepare_components(phys_mem_size);
        let vm_layout_ranges = Self::place_components(&layout_components);

        Self::new(&vm_layout_ranges)
    }

    fn new(vm_layout_ranges: &[Range<VirtAddr>]) -> Self {
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
               m_kern_text_range: Default::default() }
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
    fn prepare_components(phys_mem_size: usize)
                          -> [LayoutComponent; LayoutComponent::COUNT] {
        /* since the kernel uses a memory mapping paging strategy we need all the
         * physical memory mapped somewhere. Doing so all the physical memory is
         * accessible with <phys_addr_to_access + chosen_virt_addr>
         */
        let phys_mem_mapping_size = align_up(phys_mem_size, Page2MiB::SIZE);

        /* reserve 2 MiB to be able to map up to 512 4KiB pages or one huge 2MiB page */
        let tmp_mapping_size = Page2MiB::SIZE;

        /* obtain the remaining virtual space removing the kernel text */
        let remaining_kern_space_size = {
            let kern_text_begin_addr: VirtAddr =
                unsafe { &__kernel_text_begin as *const _ as usize }.into();
            dbg_println!(DbgLevel::Trace,
                         "kern_text_begin_addr: {}",
                         kern_text_begin_addr);

            *kern_text_begin_addr - Self::KERN_SPACE_BEGIN
        };
        dbg_println!(DbgLevel::Trace,
                     "remaining_kern_space_size: {}",
                     remaining_kern_space_size.display_pretty());

        /* remaining components receives an equal & shrinkable portion of the layout */
        let big_components_size = align_down((remaining_kern_space_size
                                              - phys_mem_mapping_size
                                              - tmp_mapping_size)
                                             / 2,
                                             Page4KiB::SIZE);

        /* return the components with the size */
        [LayoutComponent::PhysMemMapping { m_size: phys_mem_mapping_size },
         LayoutComponent::TmpMapping { m_size: tmp_mapping_size },
         LayoutComponent::KernRegions { m_size: big_components_size },
         LayoutComponent::FsPageCache { m_size: big_components_size }]
    }

    fn place_components(layout_components: &[LayoutComponent])
                        -> [Range<VirtAddr>; LayoutComponent::COUNT] {
        /* alignment mismatching and reset when encountered shrinkable components */
        let mut total_alignment_diff = 0;
        let mut vm_range_addr: VirtAddr = Self::KERN_SPACE_BEGIN.into();
        let mut layout_ranges =
            [Range::default(), Range::default(), Range::default(), Range::default()]; /* TODO Range is not Copy */

        /* place <LayoutComponent>s into virtual memory */
        for (i, &layout_component) in layout_components.iter().enumerate() {
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
        let aligned_up_addr = vm_range_addr.align_up(layout_component.inner_align());

        *total_alignment_diff += *aligned_up_addr - **vm_range_addr;
        let component_size =
            if *total_alignment_diff > 0 && layout_component.is_shrinkable() {
                let prev_alignment_diff = *total_alignment_diff;
                *total_alignment_diff = 0;

                align_down(layout_component.inner_size() - prev_alignment_diff,
                           layout_component.inner_align())
            } else {
                layout_component.inner_size()
            };

        *vm_range_addr = aligned_up_addr.offset(layout_component.inner_size());

        dbg_println!(DbgLevel::Trace,
                     "layout_component: {:?}, aligned_up_addr: {}, component_size: {}",
                     layout_component,
                     aligned_up_addr,
                     component_size.display_pretty());

        Range { start: aligned_up_addr,
                end: aligned_up_addr.offset(component_size) }
    }
}

#[derive(Debug)]
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
    fn inner_size(&self) -> usize {
        match self {
            Self::PhysMemMapping { m_size }
            | Self::TmpMapping { m_size }
            | Self::KernRegions { m_size }
            | Self::FsPageCache { m_size } => *m_size,
            _ => panic!("Tried to obtain size from a None LayoutComponent")
        }
    }

    fn inner_align(&self) -> usize {
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
