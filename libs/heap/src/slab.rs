/*! Slab allocator implementation */

use core::ptr::NonNull;

/**
 * Single size block allocator that serves the requests in `O(1)`.
 *
 * Internally keeps a list of free blocks, so the allocate/deallocate
 * operation essentially consists in a free-list pop/push
 */
pub struct Slab {
    m_free_blocks: FreeList,
    m_block_size: usize
}

impl Slab {
    /**
     * Constructs a new `Slab` instance that allocates the given range
     */
    pub unsafe fn new(addr: usize, size: usize, block_size: usize) -> Self {
        assert!(size.is_power_of_two());
        assert_eq!(size % block_size, 0);

        Self { m_free_blocks: FreeList::new(addr, size, block_size),
               m_block_size: block_size }
    }

    /**
     * Extends the available memory for allocations.
     */
    pub unsafe fn extend(&mut self, addr: usize, size: usize) {
        self.m_free_blocks.extend(addr, size, self.m_block_size);
    }

    /**
     * Allocates a new memory block of the size given in initialization.
     *
     * Returns a `Result` variant with a `NonNull<u8>` pointer when `Ok` or
     * `Err` when the used allocator runs out of memory.
     *
     * The operation always performs in `O(1)` because consists in a
     * `FreeList::pop()`
     */
    pub fn alloc_block(&mut self) -> Result<NonNull<u8>, ()> {
        match self.m_free_blocks.pop() {
            Some(block) => Ok(NonNull::new(block.as_ptr()).unwrap()),
            None => Err(())
        }
    }

    /**
     * Makes the given block available again for further allocations.
     *
     * The request, as for allocation, happen in `O(1)` due to a
     * `FreeList::push()`
     */
    pub unsafe fn dealloc_block(&mut self, ptr: NonNull<u8>) {
        self.m_free_blocks.push(&mut *(ptr.as_ptr() as *mut Block));
    }

    /**
     * Returns the amount of free block
     */
    pub fn free_count(&self) -> usize {
        self.m_free_blocks.count()
    }

    /**
     * Returns whether the `FreeList` is emtpy
     */
    pub fn is_empty(&self) -> bool {
        self.m_free_blocks.is_emtpy()
    }

    /**
     * Returns the allocation block size
     */
    pub fn block_size(&self) -> usize {
        self.m_block_size
    }
}

/**
 * Single linked list of `Block`
 */
#[derive(Default)]
struct FreeList {
    m_first: Option<&'static mut Block>,
    m_count: usize
}

impl FreeList {
    /**
     * Constructs a new `FreeList` instance that allocates the given range
     */
    unsafe fn new(addr: usize, size: usize, element_size: usize) -> Self {
        let mut free_list = Self::default();
        free_list.extend(addr, size, element_size);
        free_list
    }

    /**
     * Extends the `FreeList` pushing inside of it `size / element_size`
     * elements that are then available for further `FreeList::pop()`
     */
    unsafe fn extend(&mut self, start_addr: usize, size: usize, element_size: usize) {
        for i in (0..size / element_size).rev() {
            self.push(&mut *((start_addr + i * element_size) as *mut Block));
        }
    }

    /**
     * Returns the first available memory `Block` reference
     */
    fn pop(&mut self) -> Option<&'static mut Block> {
        self.m_first.take().map(|element| {
                               self.m_first = element.m_next.take();
                               self.m_count -= 1;
                               element
                           })
    }

    /**
     * Makes available again the given block for further `FreeList::pop()`
     */
    fn push(&mut self, element: &'static mut Block) {
        element.m_next = self.m_first.take();
        self.m_first = Some(element);
        self.m_count += 1;
    }

    /**
     * Returns the amount of remaining blocks
     */
    fn count(&self) -> usize {
        self.m_count
    }

    /**
     * Returns whether the `FreeList` is emtpy
     */
    fn is_emtpy(&self) -> bool {
        self.m_count == 0
    }
}

impl Drop for FreeList {
    /**
     * Flushes the `FreeList` elements that are still into the list
     */
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

/**
 * Single linked list node that represents a free memory slab
 */
struct Block {
    m_next: Option<&'static mut Block>
}

impl Block {
    /**
     * Converts `&self` to a `*mut u8`
     */
    fn as_ptr(&self) -> *mut u8 {
        self as *const _ as *mut u8
    }
}
