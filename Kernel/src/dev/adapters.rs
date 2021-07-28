pub type BlockId = usize;

pub trait ByteDevice: Send + Sync {
    /**
     * Read bytes from the given offset up to the buffer's length.
     * Returns the number of bytes read if successful.
     */
    fn read_at(&self, offset: usize, buffer: &mut [u8]) -> FsResult<usize>;

    /**
     * Write bytes at the given offset.
     * Returns the number of bytes written successfully.
     */
    fn write_at(&self, offset: usize, buffer: &[u8]) -> Result<usize>;

    /**
     * Sync with the underlying device.
     */
    fn sync(&self) -> Result<()>;
}

pub trait BlockDevice: Send + Sync {
    /**
     * Get the exponent to extract the block's size in bytes.
     * `byte_size = 2 ^ BLOCK_SIZE_EXP`.
     */
    fn get_block_size_exp(&self) -> u8;

    /**
     * Read from the given block up to the buffer's length.
     * Returns the number of bytes read.
     */
    fn read_at(&self, offset: BlockId, buffer: &mut [u8]) -> Result<usize>;

    /**
     * Write bytes from the given block.
     * Return the number of bytes written successfully.
     */
    fn write_at(&self, offset: BlockId, buffer: &[u8]) -> Result<usize>;

    /**
     * Sync with the underlying device.
     */
    fn sync(&self) -> Result<()>;
}
