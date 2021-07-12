# MKFS Draft

Inode based, with 64-bit addressing

The data is memorized in extents (ie chunks) of various sizes, dependent on the device block size.

Compression, data integrity check, hardlink and sparse file supported.

### Data extent structure:

Metadata about the chunk is memorized in the first 64-bits of the chunk itself, an area called chunk header

```
------------------------------------------------------------------------
| REF-CNT 16-BIT | BLK-EXP 4-BIT | FLAGS 4-BIT | CRC 32-BIT | Data.....
|.......................................................................
|.......................................................................
|.......................................................................
|.......................................................................
|.......................................................................
|.......................................................................
|...........................................
```

BLK-EXP: the exponent to apply to get number of device blocks the chunks uses. Use the given formula to get the number of used device blocks: `2^BLK-EXP`

REF-CNT: the number of inodes referencing this block. Blocks have a copy-on-write behavior when REF-CNT > 1.

FLAGS: the block's flags, indicating compression and more.

CRC: the CRC of the block's data.

The space used by the chunk itself can be calculated by 2^BLK-EXP x device block size.
Chunks are meant to reduce both internal and external fragmentation

### File-level information

NO CRC: disable crc check per block (only applied on blocks with REF-CNT = 1, since we don't know if the other files have also disabled crc)

NO COW: disable further Copy-on-Write behavior for a file. Doesn't deduplicate currently shared blocks, only with an explicit defragment.

LOW FRAGMENT MODE: changes the allocation algorithm used for growing files.
Normally when a file is expanded, the filesystem will first check if by expanding the current chunk to the minimum to fit the new changes, how much space will be wasted (internal fragmentation). If the waste is above a preset threshold, it will try to allocate a new chunk if it wastes less space.
This flag makes the filesystem always prefer expanding the current chunk if possible.
This will trade off diminishing external fragmentation with increased internal fragmentation, which might be worth it on spinning drives while providing no benefit on flash devices.

### File content data table

A map with on the right side, the offset the data extent reaches, on the left the pointer to the data extent.

Sparse file support is achieved by substituting a pointer in the map with a zero-ed address in order to mark an empty data extent.

```
------------------------------------------------------------------------
| FILE OFFSET 64-BIT | DATA EXTENT ADDRESS 64-BIT | (repeated until the end of the table)
```

### Directory extent

## Superblock

GUID: 69bfe672-67fd-4228-a54f-fccd3fb85998 128 bit
Version: 8 bit. Divide the decimal value by 10 to get the major-minor versions. Eg 164 -> 16.4
Link to the first extent pool 64 bit.
Link to the root directory 64 bit.

## Extent Pools

In order to try and keep data locality, the device is broken up into multiple pools, of <TO BE DECIDED> megabytes each. Each pool has a pool table with:
The link to the next pool
The size of the local free blocks bitmap
The size of the local bad blocks list
The free blocks bitmap
The bad blocks list

When a new file is allocated, the filesystem picks a pool for it, and allocates it's metadata or new data extents with it.
If a related extent cannot be placed in the same pool, it's simply moved to one nearby, and so on.