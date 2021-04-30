/*! # Path Related Bits
 *
 * Implements the enumeration related to certain calls of [`Path`]
 *
 * [`Path`]: crate::path::Path
 */

use crate::bits::obj::ObjType;

/** # `Path::exists()` States
 *
 * Lists the available states for [`Path::exists()`]
 *
 * [`Path::exists()`]: crate::path::Path
 */
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum PathExistsState {
    /** The path exists from the current directory (or the root if the
     * [`Path`] is absolute) to the last component.
     *
     * It contains the [`ObjType`] of the last component referenced
     *
     * [`Path`]: crate::path::Path
     * [`ObjType`]: crate::bits::obj::types::ObjType
     */
    Exists(ObjType),

    /** The path exists only until a certain component, the variant
     * contains the index of the last existing component
     */
    ExistsUntil(u32),

    /** The path doesn't exists completely
     */
    NotExists,

    /** An empty path was given
     */
    EmptyPath
}
