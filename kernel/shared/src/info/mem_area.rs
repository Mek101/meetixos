/*! Boot physical memory area */

use core::cmp::Ordering;

use crate::addr::phys::PhysAddr;

/**
 * Maximum amount of `BootMemArea`s storable into a `BootMemAreas`
 */
pub const BOOT_MEM_AREAS_COUNT_MAX: usize = 64;

/**
 * Fixed collection of address ordered  `BootMemArea`s
 */
#[derive(Debug, Clone)]
pub struct BootMemAreas {
    m_areas: [Option<BootMemArea>; BOOT_MEM_AREAS_COUNT_MAX],
    m_next_usable: usize
}

impl BootMemAreas {
    /**
     * Constructs an empty `BootMemAreas`
     */
    pub fn new() -> Self {
        Self { m_areas: [None; BOOT_MEM_AREAS_COUNT_MAX],
               m_next_usable: 0 }
    }

    /**  
     * Inserts a new sorted `BootMemArea`
     */
    pub fn insert(&mut self, new_area: BootMemArea) {
        self.push(new_area);
        self.sort_areas();
    }

    /**  
     * Pushes a new `BootMemArea`
     */
    pub fn push(&mut self, new_area: BootMemArea) {
        assert!(self.m_next_usable < BOOT_MEM_AREAS_COUNT_MAX);

        /* push the new area at the end */
        self.m_areas[self.m_next_usable] = Some(new_area);
        self.m_next_usable += 1;
    }

    /**  
     * Sort the `BootMemAreas`
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

    /**
     * Returns the iterator to the valid `BootMemArea`s
     */
    pub fn iter(&self) -> impl Iterator<Item = &BootMemArea> {
        self.m_areas
            .iter()
            .filter(|opt_area| opt_area.is_some())
            .map(|area| area.as_ref().unwrap())
    }
}

/**
 * Simple descriptor for a valid physical memory area
 */
#[derive(Debug, Copy, Clone)]
pub struct BootMemArea {
    m_start_phys_addr: PhysAddr,
    m_size: usize
}

impl BootMemArea {
    /**
     * Constructs a `BootMemArea` filled with the given arguments
     */
    pub fn new(phys_addr: PhysAddr, size: usize) -> Self {
        Self { m_start_phys_addr: phys_addr,
               m_size: size }
    }

    /**
     * Returns whether the given `PhysAddr` belongs to this `BootMemArea`
     */
    pub fn contains(&self, phys_addr: PhysAddr) -> bool {
        phys_addr >= self.m_start_phys_addr
        && phys_addr < self.m_start_phys_addr + self.m_size
    }

    /**
     * Returns the starting physical address of the area
     */
    pub fn start_phys_addr(&self) -> PhysAddr {
        self.m_start_phys_addr
    }

    /**
     * Returns the size in bytes of the area
     */
    pub fn size(&self) -> usize {
        self.m_size
    }
}
