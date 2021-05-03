/*! # Kernel Virtual Memory Layout
 *
 * This module exposes the constant virtual addresses value that describes
 * the layout of the virtual memory for the kernel and the user processes
 */

pub const KRN_HEAP_START: usize = 0x00007E8000000000;
pub const KRN_HEAP_END: usize = 0x00007E8003800000;

pub const KRN_UNMNG_AREA_START: usize = KRN_HEAP_END;
pub const KRN_UNMNG_AREA_END: usize = 0x00007F8000000000;

/* Virtual address where physical memory is mapped
 *
 * NOTE this value MUST be kept aligned with the one stored into the
 * kernel's Cargo.toml
 */
//pub const KRN_PHYS_MEM_AREA_START: usize = KRN_UNMNG_AREA_END;
// pub const KRN_PHYS_MEM_AREA_END: usize = 0x0000800000000000;

// pub const KRN_CODE_DATA_START: usize = 0xFFFFFFFFF0000000;
