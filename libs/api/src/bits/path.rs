/*! # Path Related Bits
 *
 * Implements the enumeration related to certain calls of [`Path`]
 *
 * [`Path`]: crate::path::Path
 */

use crate::bits::obj::ObjType;
use core::convert::TryFrom;

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

impl TryFrom<(usize, usize)> for PathExistsState {
    /** The type returned in the event of a conversion error
     */
    type Error = ();

    /** Performs the conversion
     */
    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        match value.0 {
            0 => {
                if let Ok(obj_type) = ObjType::try_from(value.1) {
                    Ok(Self::Exists(obj_type))
                } else {
                    Err(())
                }
            },
            1 => Ok(Self::ExistsUntil(value.1 as u32)),
            2 => Ok(Self::NotExists),
            3 => Ok(Self::EmptyPath),
            _ => Err(())
        }
    }
}
