/*! `Path` related bits */

use core::convert::TryFrom;

use crate::bits::obj::types::ObjType;

/**
 * Lists the possibly return states of `Path::exists()`
 */
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum PathExistsState {
    /**
     * The path exists from the current directory (or the root if the `Path`
     * is absolute) to the last component.
     *
     * It contains the `ObjType` of the last component referenced
     */
    Exists(ObjType),

    /**
     * The path exists only until a certain component, the variant contains
     * the index of the last existing component which can be retrieved via
     * `path[]` operator
     */
    ExistsUntil(usize),

    /**
     * The path doesn't exists completely
     */
    NotExists,

    /**
     * An empty path was given
     */
    EmptyPath
}

impl TryFrom<(usize, usize)> for PathExistsState {
    type Error = ();

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        match value.0 {
            0 => {
                if let Ok(obj_type) = ObjType::try_from(value.1) {
                    Ok(Self::Exists(obj_type))
                } else {
                    Err(())
                }
            },
            1 => Ok(Self::ExistsUntil(value.1)),
            2 => Ok(Self::NotExists),
            3 => Ok(Self::EmptyPath),
            _ => Err(())
        }
    }
}
