/*! Slab allocator implementation */

use core::ptr::NonNull;

use crate::{
    PreferredExtendSize,
    SubHeapPool
};

/**
 * Single size block allocator that serves the requests in `O(1)`
 */
pub struct Slab<const BLOCK_SIZE: usize> {
    m_free_blocks: FreeBlockList
}

impl<const BLOCK_SIZE: usize> Slab<BLOCK_SIZE> {
    /**
     * Constructs a `Slab` from the given parameters
     */
    pub unsafe fn new(start_area_addr: *mut u8, area_size: usize) -> Self {
        Self { m_free_blocks: FreeBlockList::new(start_area_addr,
                                                 area_size,
                                                 BLOCK_SIZE) }
    }

    /**
     * Constructs a `Slab` with the `PREFERRED_EXTEND_SIZE`
     */
    pub unsafe fn with_preferred_size(start_area_addr: *mut u8) -> Self {
        Self::new(start_area_addr, Self::PREFERRED_EXTEND_SIZE)
    }

    /**
     * Allocates a new block of memory
     */
    pub fn allocate(&mut self) -> Option<NonNull<u8>> {
        self.m_free_blocks
            .pop()
            .map(|block| unsafe { NonNull::new_unchecked(block.as_ptr()) })
    }

    /**
     * Frees a previously allocated block
     */
    pub unsafe fn deallocate(&mut self, nn_ptr: NonNull<u8>) {
        self.m_free_blocks.push(&mut *(nn_ptr.as_ptr() as *mut SlabBlock));
    }

    /**
     * Returns the `BLOCK_SIZE` parameter
     */
    pub fn block_size(&self) -> usize {
        BLOCK_SIZE
    }

    /**
     * Returns the amount of free block
     */
    pub fn free_count(&self) -> usize {
        self.m_free_blocks.count()
    }

    /**
     * Returns whether the `FreeBlockList` is emtpy
     */
    pub fn is_empty(&self) -> bool {
        self.m_free_blocks.is_emtpy()
    }
}

impl<const BLOCK_SIZE: usize> SubHeapPool for Slab<BLOCK_SIZE> {
    unsafe fn add_region(&mut self,
                         start_area_ptr: NonNull<u8>,
                         area_size: usize)
                         -> Option<(NonNull<u8>, usize)> {
        /* calculate the right area size and the eventual exceeding */
        let exceeding_area_size = area_size % BLOCK_SIZE;
        let area_size = if exceeding_area_size > 0 {
            area_size - exceeding_area_size
        } else {
            area_size
        };

        /* extend the free-list of the slab */
        self.m_free_blocks.extend(start_area_ptr.as_ptr(), area_size, BLOCK_SIZE);

        /* return the exceeded if any */
        if exceeding_area_size > 0 {
            Some((NonNull::new_unchecked(start_area_ptr.as_ptr().add(area_size)),
                  exceeding_area_size))
        } else {
            None
        }
    }

    fn preferred_extend_size(&self) -> usize {
        Self::PREFERRED_EXTEND_SIZE
    }
}

impl<const BLOCK_SIZE: usize> PreferredExtendSize for Slab<BLOCK_SIZE> {
    const PREFERRED_EXTEND_SIZE: usize = BLOCK_SIZE * 4; /* at least 4 block for each extension */
}

/**
 * Single linked list of `Block`
 */
#[derive(Default)]
struct FreeBlockList {
    m_first: Option<&'static mut SlabBlock>,
    m_count: usize
}

impl FreeBlockList {
    /**
     * Constructs a `FreeBlockList` from the given parameters
     */
    unsafe fn new(start_area_addr: *mut u8, area_size: usize, block_size: usize) -> Self {
        let mut free_list = Self::default();
        free_list.extend(start_area_addr, area_size, block_size);
        free_list
    }

    /**
     * Adds the given region to this `FreeBlockList`
     */
    unsafe fn extend(&mut self,
                     start_area_addr: *mut u8,
                     area_size: usize,
                     block_size: usize) {
        for i in (0..area_size / block_size).rev() {
            let next_free_block =
                &mut *(start_area_addr.add(i * block_size) as *mut SlabBlock);
            self.push(next_free_block);
        }
    }

    /**
     * Returns the first available memory `Block` reference
     */
    fn pop(&mut self) -> Option<&'static mut SlabBlock> {
        self.m_first.take().map(|slab_block| {
                               self.m_first = slab_block.m_next.take();
                               self.m_count -= 1;
                               slab_block
                           })
    }

    /**
     * Pushes the given `Block` into this `FreeBlockList`
     */
    fn push(&mut self, slab_block: &'static mut SlabBlock) {
        slab_block.m_next = self.m_first.take();
        self.m_first = Some(slab_block);
        self.m_count += 1;
    }

    /**
     * Returns the amount of remaining blocks
     */
    fn count(&self) -> usize {
        self.m_count
    }

    /**
     * Returns whether the `FreeBlockList` is emtpy
     */
    fn is_emtpy(&self) -> bool {
        self.m_count == 0
    }
}

impl Drop for FreeBlockList {
    fn drop(&mut self) {
        while let Some(_) = self.pop() { /* nothing to do here */ }
    }
}

/**
 * Single linked list node that represents a free memory slab
 */
struct SlabBlock {
    m_next: Option<&'static mut SlabBlock>
}

impl SlabBlock {
    /**
     * Converts `&self` to a `*mut u8`
     */
    fn as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }
}
