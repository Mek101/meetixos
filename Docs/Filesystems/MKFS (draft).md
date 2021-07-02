# MKFS Draft

Inode-based, with 64-bit addressing

The data is memorized in chunks of various sizes, dependent on the device block size

### Chunk structure:

Metadata about the chunk is memorized in the first 64-bits of the chunk itself, an area called chunk header

```
------------------------------------------------------------------------
| BLK-CNT 16-BIT | REF-CNT 16-BIT | FLAGS 8-BIT | CRC 24-BIT | Data.....
|.......................................................................
|.......................................................................
|.......................................................................
|.......................................................................
|.......................................................................
|.......................................................................
|...........................................
```

BLK-CNT: the number of device blocks the chunks uses.

REF-CNT: the number of inodes referencing this block. Blocks have a copy-on-write behaviour when REF-CNT > 1.

FLAGS: the block's flags, indicating compression, CRC usage and more.

CRC: the CRC of the block's data.

The space used by the chunk itself can be calculated by BLK-CNT x device block size.
Chunks are meant to reduce both internal and external fragmentation

