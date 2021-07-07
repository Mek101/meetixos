# MKFS Draft

Extent based, with 64-bit addressing

The data is memorized in extents (ie chunks) of various sizes, dependent on the device block size

### Data extent structure:

Metadata about the chunk is memorized in the first 64-bits of the chunk itself, an area called chunk header

```
----------------------------------------------------------------------------
| CRC 32-BIT | REF-CNT 16-BIT | BLK-EXP 4-BIT | FLAGS 4-BIT |..............|
| Data.....................................................................|
|..........................................................................|
|..........................................................................|
|..........................................................................|
|..........................................................................|
|..........................................................................|
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

NO COW: disable further Copy-on-Write behavior for a file. Doesn't deduplicate currently shared blocks, only with a defragment.

LOW FRAGMENT MODE: changes the allocation algorithm used for growing files.
Normally when a file is expanded, the filesystem will first check if by expanding the current data extent to the minimum to fit the new changes, how much space will be wasted (internal fragmentation). If the waste is above a preset threshold, it will try to allocate a new chunk if it wastes less space.
This flag makes the filesystem always prefer expanding the current chunk if possible.
This will trade off diminishing external fragmentation with increased internal fragmentation, which might be worth it on spinning drives while providing no benefit on flash devices.
Metadata extents always have this behavior.

## Areas

Areas in the media where the filesystem stores information and metadata. Some may be broken up in different metadata extents and moved as needed.

## Metadata extents:

Where MXFS memorizes information like directory content and names

```
------------------------------------------------------------------------
| CRC 32-BIT| CONT 64-BIT | BLK-ESP 4-BIT | Data................
.................................................................
.................................................................
.......................................................
```

The data format changes from the kind of metadata extent.

### Directory extent:

### File extent:

```
------------------------------------------------------------------------
| CRC 32-BIT | BLK-ESP 4-BIT | NAME-LEN 10-BIT |
```

Since the data extents are CoW'ed when there are multiple references to it, we don't have to worry about data size changing from under us, so we can store the effectively used bytes in the File extent.

### Information extent: