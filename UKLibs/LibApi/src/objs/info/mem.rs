/*! Object memory information */

/**
 * Stores the referenced `Object` memory statistics
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct ObjMemInfo {
    m_block_size: usize,
    m_blocks_used: usize,
    m_used_size: usize
}

#[cfg(feature = "enable_kernel_methods")]
impl ObjMemInfo {
    /**
     * Constructs an `ObjMemInfo` with the given parameters
     */
    pub const fn new(block_size: usize, blocks_used: usize, used_size: usize) -> Self {
        Self { m_block_size: block_size,
               m_blocks_used: blocks_used,
               m_used_size: used_size }
    }
}

impl ObjMemInfo {
    /**
     * Returns the block size unit used to store the `Object` data
     */
    pub fn block_size(&self) -> usize {
        self.m_block_size
    }

    /**
     * Returns the amount of unit blocks used to store the `Object` data
     */
    pub fn blocks_used(&self) -> usize {
        self.m_blocks_used
    }

    /**
     * Returns the effectively used size of the `Object` data
     */
    pub fn used_size(&self) -> usize {
        self.m_used_size
    }

    /**
     * Returns the real amount of memory used to store the `Object` data
     */
    pub fn real_size(&self) -> usize {
        self.m_block_size * self.m_blocks_used
    }

    /**
     * Returns the difference between the real size and the used size
     */
    pub fn unused_space(&self) -> usize {
        self.real_size() - self.used_size()
    }
}
