/*! `Path` Management */

use core::convert::TryFrom;

use crate::obj::types::ObjType;

/**
 * Lists the possibly return states of `Path::exists()`
 */
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
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

impl PathExistsState {
    /**
     * Returns `&self` as usize pointer value
     */
    pub fn as_syscall_ptr(&mut self) -> usize {
        self as *mut Self as usize
    }
}
