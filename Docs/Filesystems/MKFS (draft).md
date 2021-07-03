# MKFS Draft

Inode-based, with 64-bit addressing

The data is memorized in chunks of various sizes, dependent on the device block size

### Chunk structure:

Metadata about the chunk is memorized in the first 64-bits of the chunk itself, an area called chunk header

```
------------------------------------------------------------------------
| REF-CNT 16-BIT | BLK-ESP 4-BIT | FLAGS 4-BIT | CRC 32-BIT | Data.....
|.......................................................................
|.......................................................................
|.......................................................................
|.......................................................................
|.......................................................................
|.......................................................................
|...........................................
```

BLK-ESP: the esponent to apply to get number of device blocks the chunks uses. Use the given formula to get the number of used device blocks: `2^BLK-ESP`

REF-CNT: the number of inodes referencing this block. Blocks have a copy-on-write behaviour when REF-CNT > 1.

FLAGS: the block's flags, indicating compression and more.

CRC: the CRC of the block's data.

The space used by the chunk itself can be calculated by 2^BLK-ESP x device block size.
Chunks are meant to reduce both internal and external fragmentation

### File-level information

NO CRC: disable crc check per block (only applied on blocks with REF-CNT = 1, since we don't know if the other files have also disabled crc)

NO COW: disable futher Copy-on-Write behaviour for a file. Doesn't deduplicate currently shared blocks, only with a defragment.

LOW FRAGMENT MODE: changes the allocation algorythm used for growing files.
Normally when a file is expanded, the filesystem will first check if by expanding the current chunk to the minimum to fit the new changes, how much space will be wasted (internal fragmentation). If the waste is above a preset trewshold, it will try to allocate a new chunk if it wastes less space.
This flag makes the filesystem always prefer expanding the current chunk if possible.
This will trade off diminishing external fragmentation with increased internal fragmentation, which might be worth it on spinning drives while providing no benefit on flash devices.
