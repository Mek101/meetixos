/*! # Heap Manager Constants
 *
 * Lists the constant values used by this crate
 */

/** Represents the expected alignment for the sizes given to the [`Heap`]
 * manager
 *
 * [`Heap`]: crate::Heap
 */
pub const PAGE_SIZE: usize = 4096;

/** Represents the maximum amount of bytes that can be wasted using slab
 * allocation, exceeded the value the allocation request rollbacks to linked
 * list allocation
 */
pub const SLAB_THRESHOLD: usize = 500;
