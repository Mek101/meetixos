/*! Heap Manager Constants */

/**
 * Maximum amount of bytes that can be wasted using slab allocation,
 * exceeded the value the allocation request rollbacks to linked list
 * allocation
 */
pub const SLAB_THRESHOLD: usize = 512;
