use core::sync::RwLock;

use bits::fields::BitArray;

use crate::filesystem::implementation::mxfs::Compression;

struct DataExtent {
    wrapper: RwLock<ChunkWrapper>
}

impl DataExtent {

}

struct DataExtentWrapper {
    header: u64,
    data_start_block: u64,
    end_block: u64,
}

impl DataExtentWrapper {
    pub fn get_compression(&self) -> Compression {
        self.header[20..24] as Compression
    }

    pub fn set_compression(&mut self, compression: Compression) {
        if self.get_compression() != compression {
           self.header[20..24] = compression[0..4]
            // We need to update the data with the new compression.
            // Also, how to handle an existing stream?
        }
    }

    pub fn read_stream(&self) -> impl Iterator<Item = u8>  {

    }

    pub fn write_stream(&mut self) -> Result<(), &str> {

    }
}

