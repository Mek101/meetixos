/*! Linked list allocator implementation */

use core::{
    alloc::Layout,
    mem::{
        align_of,
        size_of
    },
    ptr::NonNull
};

use helps::{
    align::align_up,
    misc::force_move
};

use crate::SubHeapAllocator;

/**
 * Sorted single linked list of `Hole`s
 */
pub struct LinkedList {
    m_first_hole: Hole
}

impl LinkedList {
    /**
     * Constructs a `LinkedList` from the given parameters
     */
    pub unsafe fn new(hole_addr: NonNull<u8>, hole_size: usize) -> LinkedList {
        let first_real_hole_ptr = {
            let raw_hole_addr = hole_addr.as_ptr() as usize;
            let aligned_hole_addr = align_up(raw_hole_addr, align_of::<Hole>());
            let aligned_hole_ptr = aligned_hole_addr as *mut Hole;
            let hole_to_write =
                Hole::new(hole_size - (aligned_hole_addr - raw_hole_addr), None);

            aligned_hole_ptr.write(hole_to_write);
            aligned_hole_ptr
        };

        LinkedList { m_first_hole: Hole::new(0, Some(&mut *first_real_hole_ptr)) }
    }

    /**
     * Returns the minimal allocation size
     */
    pub const fn block_size() -> usize {
        size_of::<usize>() * 2
    }

    /**
     * Returns a layout with size increased to fit at least
     * `LinkedList::min_size` and proper alignment of a `Hole`
     */
    fn align_layout(layout: Layout) -> Layout {
        let mut size = layout.size();
        if size < Self::block_size() {
            size = Self::block_size();
        }
        let size = align_up(size, align_of::<Hole>());
        let layout = Layout::from_size_align(size, layout.align()).unwrap();

        layout
    }
}

impl SubHeapAllocator for LinkedList {
    const PREFERRED_EXTEND_SIZE: usize = 8192; /* 8KiB for each extension */

    fn allocate(&mut self, layout: Layout) -> Option<NonNull<u8>> {
        allocate_first_fit(&mut self.m_first_hole, Self::align_layout(layout))
            .map(|allocation| {
                /* deallocate eventual front padding of the Allocation */
                if let Some(padding) = allocation.m_front_padding {
                    deallocate(&mut self.m_first_hole, padding.m_addr, padding.m_size);
                }

                /* deallocate eventual back padding of the Allocation */
                if let Some(padding) = allocation.m_back_padding {
                    deallocate(&mut self.m_first_hole, padding.m_addr, padding.m_size);
                }

                unsafe { NonNull::new_unchecked(allocation.m_hole_info.m_addr as *mut u8) }
            })
    }

    unsafe fn deallocate(&mut self, nn_ptr: NonNull<u8>, layout: Layout) {
        deallocate(&mut self.m_first_hole,
                   nn_ptr.as_ptr() as usize,
                   Self::align_layout(layout).size());
    }

    unsafe fn add_region(&mut self,
                         start_area_ptr: NonNull<u8>,
                         area_size: usize)
                         -> Option<(NonNull<u8>, usize)> {
        self.deallocate(start_area_ptr, Layout::from_size_align_unchecked(area_size, 1));
        None
    }

    fn block_size(&self) -> Option<usize> {
        None
    }
}

/**
 * A block of usable (allocatable) memory
 */
struct Hole {
    m_size: usize,
    m_next_hole: Option<&'static mut Hole>
}

impl Hole {
    /**
     * Constructs a `Hole` with the given parameters
     */
    const fn new(size: usize, next_hole: Option<&'static mut Hole>) -> Self {
        Self { m_size: size,
               m_next_hole: next_hole }
    }

    /**
     * Returns the information about this `Hole`
     */
    fn info(&self) -> HoleInfo {
        HoleInfo::new(self as *const _ as usize, self.m_size)
    }
}

/**
 * Metadata information about a `Hole`
 */
#[derive(Debug, Clone, Copy)]
struct HoleInfo {
    m_addr: usize,
    m_size: usize
}

impl HoleInfo {
    /**
     * Constructs a `HoleInfo` from the given parameters
     */
    const fn new(addr: usize, size: usize) -> Self {
        Self { m_addr: addr,
               m_size: size }
    }
}

/**
 * Result returned by `split_hole()` and `allocate_first_fit()`
 */
struct Allocation {
    m_hole_info: HoleInfo,
    m_front_padding: Option<HoleInfo>,
    m_back_padding: Option<HoleInfo>
}

/**
 * Splits the given hole into `(front_padding, hole, back_padding)` if it's
 * big enough to allocate `required_layout.size()` bytes with the
 * `required_layout.align()`.
 *
 * Front padding occurs if the required alignment is higher than the hole's
 * alignment.
 *
 * Back padding occurs if the required size is smaller than the size of the
 * aligned hole.
 *
 * All padding must be at least `HoleList::min_size()` big or the hole is
 * unusable.
 */
fn split_hole(hole: HoleInfo, required_layout: Layout) -> Option<Allocation> {
    let required_size = required_layout.size();
    let required_align = required_layout.align();

    let (aligned_addr, front_padding) =
        if hole.m_addr == align_up(hole.m_addr, required_align) {
            /* hole has already the required alignment */
            (hole.m_addr, None)
        } else {
            /* the required alignment causes some padding before the allocation */
            let aligned_addr =
                align_up(hole.m_addr + LinkedList::block_size(), required_align);
            let hole_info = HoleInfo::new(hole.m_addr, aligned_addr - hole.m_addr);

            (aligned_addr, Some(hole_info))
        };

    let aligned_hole = {
        if aligned_addr + required_size > hole.m_addr + hole.m_size {
            /* hole is too small */
            return None;
        }

        HoleInfo::new(aligned_addr, hole.m_size - (aligned_addr - hole.m_addr))
    };

    let back_padding = if aligned_hole.m_size == required_size {
        /* the aligned hole has exactly the size that's needed, no padding accrues */
        None
    } else if aligned_hole.m_size - required_size < LinkedList::block_size() {
        /* we can't use this hole since its remains would form a new, too small hole */
        return None;
    } else {
        /* the hole is bigger than necessary, so there is some padding behind the
         * allocation
         */
        Some(HoleInfo::new(aligned_hole.m_addr + required_size,
                           aligned_hole.m_size - required_size))
    };

    Some(Allocation { m_hole_info: HoleInfo::new(aligned_hole.m_addr, required_size),
                      m_front_padding: front_padding,
                      m_back_padding: back_padding })
}

/**
 * Searches for the first available `Hole` which fits the requested `Layout`
 */
fn allocate_first_fit(mut prev_hole: &mut Hole, layout: Layout) -> Option<Allocation> {
    loop {
        let allocation: Option<Allocation> =
            prev_hole.m_next_hole
                     .as_mut()
                     .and_then(|curr_hole| split_hole(curr_hole.info(), layout.clone()));
        match allocation {
            Some(allocation) => {
                /* remove from the previous one the Hole, which is big enough */
                prev_hole.m_next_hole =
                    prev_hole.m_next_hole.as_mut().unwrap().m_next_hole.take();
                return Some(allocation);
            },
            None if prev_hole.m_next_hole.is_some() => {
                /* try next hole */
                prev_hole = force_move(prev_hole).m_next_hole.as_mut().unwrap();
            },
            _ => {
                /* no allocation possible, we may have exhausted the memory or the list
                 * fragmentation involved a non-big enough hole to serve the request
                 */
                return None;
            }
        }
    }
}

/**
 * Frees the allocation given by `(addr, size)`
 */
fn deallocate(mut hole: &mut Hole, addr: usize, mut size: usize) {
    loop {
        assert!(size >= LinkedList::block_size());

        let hole_addr = if hole.m_size == 0 {
            /* it's the dummy hole, which is the head of the HoleList.
             *
             * it's somewhere on the stack, so it's address is not the address of the
             * hole. We set the addr to 0 as it's always the first hole.
             */
            0
        } else {
            /* it's a real hole in memory and its address is the address of the hole */
            hole as *mut _ as usize
        };

        /* each freed block must be handled by the previous hole in memory.
         *
         * thus the freed address must be always behind the current hole.
         */
        assert!(hole_addr + hole.m_size <= addr,
                "invalid deallocation (probably a double free)");

        /* get information about the next block */
        let next_hole_info = hole.m_next_hole.as_ref().map(|next| next.info());

        match next_hole_info {
            Some(next)
                if hole_addr + hole.m_size == addr && addr + size == next.m_addr =>
            {
                /* block fills the gap between this hole and the next hole
                 * before: ___XXX____YYYYY____
                 * after:  ___XXXFFFFYYYYY____
                 *
                 * where X is this hole, Y the next hole and F the freed block
                 */

                /* merge the F and Y blocks to this X block, then remove te Y block */
                hole.m_size += size + next.m_size;
                hole.m_next_hole = hole.m_next_hole.as_mut().unwrap().m_next_hole.take();
            }
            _ if hole_addr + hole.m_size == addr => {
                /* block is right behind this hole but there is used memory after it
                 * before:  ___XXX______YYYYY____
                 * after:   ___XXXFFFF__YYYYY____
                 *
                 * OR
                 *
                 * block is right behind this hole and this is the last hole
                 * before:  ___XXX_______________
                 * after:   ___XXXFFFF___________
                 *
                 * where X is this hole, Y the next hole and F the freed block
                 */

                /* merge the F block to this X block */
                hole.m_size += size;
            },
            Some(next) if addr + size == next.m_addr => {
                /* block is right before the next hole but there is used memory before
                 * it
                 *
                 * before:  ___XXX______YYYYY____
                 * after:   ___XXX__FFFFYYYYY____
                 *
                 * where X is this hole, Y the next hole and F the freed block
                 */

                /* remove the Y block, then free the merged F/Y block in next iteration */
                hole.m_next_hole = hole.m_next_hole.as_mut().unwrap().m_next_hole.take();
                size += next.m_size;
                continue;
            },
            Some(next) if next.m_addr <= addr => {
                /* block is behind the next hole, so we delegate it to the next hole
                 * before:  ___XXX__YYYYY________
                 * after:   ___XXX__YYYYY__FFFF__
                 *
                 * where X is this hole, Y the next hole and F the freed block
                 */

                /* start next iteration at next hole */
                hole = force_move(hole).m_next_hole.as_mut().unwrap();
                continue;
            },
            _ => {
                /* block is between this and the next hole
                 * before:  ___XXX________YYYYY_
                 * after:   ___XXX__FFFF__YYYYY_
                 *
                 * OR
                 *
                 * this is the last hole
                 * before:  ___XXX_________
                 * after:   ___XXX__FFFF___
                 *
                 * where X is this hole, Y the next hole and F the freed block
                 */

                let new_hole = Hole::new(size, hole.m_next_hole.take());

                debug_assert_eq!(addr % align_of::<Hole>(), 0);

                /* write the new hole to the freed memory */
                let hole_ptr = addr as *mut Hole;
                unsafe { hole_ptr.write(new_hole) };

                /* add the F block as the next block of the X block */
                hole.m_next_hole = Some(unsafe { &mut *hole_ptr });
            }
        }
        break;
    }
}
