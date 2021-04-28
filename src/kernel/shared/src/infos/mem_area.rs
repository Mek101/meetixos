/*! # HAL Boot Physical Memory Area
 *
 * Implements the simple descriptor of physical memory region given to the
 * kernel through the [`BootInfos`]
 *
 * [`BootInfos`]: /hal/infos/struct.BootInfos.html
 */

use core::cmp::Ordering;

use crate::{
    addr::{Address, PhysAddr},
    mem::paging::{PageSize, PhysFrame, PhysFrameRange}
};

/** Maximum amount of [`BootMemArea`]s storable into a [`BootMemAreas`]
 *
 * [`BootMemArea`]: /hal/infos/struct.BootMemArea.html
 * [`BootMemAreas`]: /hal/infos/struct.BootMemAreas.html
 */
pub const BOOT_MEM_AREAS_COUNT_MAX: usize = 64;

/** # Boot Memory Area Collection
 *
 * Represents a fixed collection of ordered [`BootMemArea`]s
 *
 * [`BootMemArea`]: /hal/infos/struct.BootMemArea.html
 */
#[derive(Debug, Clone)]
pub struct BootMemAreas {
    m_areas: [Option<BootMemArea>; BOOT_MEM_AREAS_COUNT_MAX],
    m_next_usable: usize
}

impl BootMemAreas {
    /** # Constructs a `BootMemAreas`
     *
     * The returned instance is empty
     */
    pub fn new() -> Self {
        Self { m_areas: [None; BOOT_MEM_AREAS_COUNT_MAX],
               m_next_usable: 0 }
    }

    /** # Inserts a new sorted `BootMemArea`
     *
     * The new area is pushed into this collection and placed at the right
     * sorting place
     */
    pub fn insert(&mut self, new_area: BootMemArea) {
        self.push(new_area);
        self.sort_areas();
    }

    /** # Pushes a new `BootMemArea`
     *
     * The new area is pushed at the end of this collection
     */
    pub fn push(&mut self, new_area: BootMemArea) {
        assert!(self.m_next_usable < BOOT_MEM_AREAS_COUNT_MAX);

        /* push the new area at the end */
        self.m_areas[self.m_next_usable] = Some(new_area);
        self.m_next_usable += 1;
    }

    /** # Sort the `BootMemAreas`
     *
     * Places the valid areas at the beginning and the null at the end
     */
    pub fn sort_areas(&mut self) {
        /* use sort_unstable because it uses <quicksort> algorithm */
        self.m_areas.sort_unstable_by(|a1, a2| {
                        /* comparison algorithm taken from Phillip Opperman's
                         * bootloader
                         */
                        if a1.is_none() {
                            Ordering::Greater
                        } else if a2.is_none() {
                            Ordering::Less
                        } else {
                            let a1 = a1.unwrap();
                            let a2 = a2.unwrap();
                            let ord = a1.m_start_phys_addr.cmp(&a2.m_start_phys_addr);

                            if ord == Ordering::Equal {
                                let a1_end_addr = a1.m_start_phys_addr + a1.m_size;
                                let a2_end_addr = a2.m_start_phys_addr + a2.m_size;
                                a1_end_addr.cmp(&a2_end_addr)
                            } else {
                                ord
                            }
                        }
                    })
    }

    /** Returns the iterator to the valid [`BootMemArea`]s
     *
     * [`BootMemArea`]: /hal/infos/struct.BootMemArea.html
     */
    pub fn iter(&self) -> impl Iterator<Item = &BootMemArea> {
        self.m_areas
            .iter()
            .filter(|opt_area| opt_area.is_some())
            .map(|area| area.as_ref().unwrap())
    }
}

/** # Boot Memory Area
 *
 * Represents a simple descriptor for a valid physical memory area
 */
#[derive(Debug, Copy, Clone)]
pub struct BootMemArea {
    m_start_phys_addr: PhysAddr,
    m_size: usize
}

impl BootMemArea {
    /** # Constructs a `BootMemArea`
     *
     * The returned instance is filled with the given data
     */
    pub fn new(phys_addr: PhysAddr, size: usize) -> Self {
        Self { m_start_phys_addr: phys_addr,
               m_size: size }
    }

    /** Returns whether the given [`PhysAddr`] belongs to this `BootMemArea`
     *
     * [`PhysAddr`]: /hal/addr/struct.PhysAddr.html
     */
    pub fn contains(&self, phys_addr: PhysAddr) -> bool {
        phys_addr >= self.m_start_phys_addr
        && phys_addr < self.m_start_phys_addr + self.m_size
    }

    pub fn sub_area(&self, phys_addr: PhysAddr) -> Option<BootMemArea> {
        if self.contains(phys_addr) {
            let new_size = self.m_size - (phys_addr - self.m_start_phys_addr).as_usize();

            Some(Self::new(phys_addr, new_size))
        } else {
            None
        }
    }

    /** Returns this `BootMemArea` as [`PhysFrameRange`]
     *
     * [`PhysFrameRange`]: /hal/paging/type.PhysFrameRange.html
     */
    pub fn as_frame_range<S>(&self) -> PhysFrameRange<S>
        where S: PageSize {
        assert_eq!(self.m_size & S::MASK, 0);

        let start_frame = PhysFrame::of_addr(self.m_start_phys_addr);
        PhysFrame::range_of(start_frame, start_frame + self.m_size / S::SIZE)
    }

    /** Returns the starting physical address of the area
     */
    pub fn start_phys_addr(&self) -> PhysAddr {
        self.m_start_phys_addr
    }

    /** Returns the size in bytes of the area
     */
    pub fn size(&self) -> usize {
        self.m_size
    }
}
