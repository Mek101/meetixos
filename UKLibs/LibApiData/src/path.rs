/*! `Path` Management */

use core::{
    fmt,
    fmt::Display
};

use alloc::string::{
    String,
    ToString
};

use crate::{
    object::types::ObjType,
    sys::AsSysCallPtr
};

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

impl AsSysCallPtr for PathExistsState {
    /* No methods to implement */
}

/**
 * Lists the possibly parts of a lexical path
 */
#[derive(Debug)]
#[derive(Clone)]
pub enum PathComponent {
    Root,
    SelfLink,
    ParentLink,
    ObjectName(String) /* it's used only for input to the kernel, so it's safe */
}

impl PathComponent {
    /**
     * Lexical value to indicate the current directory
     */
    pub const SELF_LINK: &'static str = ".";

    /**
     * Lexical value to indicate the parent directory
     */
    pub const PARENT_LINK: &'static str = "..";

    /**
     * Lexical value to separate the path components
     */
    pub const SEPARATOR: &'static str = "/";

    /**
     * Returns whether `self` is `PathComponent::Root`
     */
    pub fn is_root(&self) -> bool {
        matches!(*self, Self::Root)
    }

    /**
     * Returns whether `self` is `PathComponent::SelfLink`
     */
    pub fn is_self_link(&self) -> bool {
        matches!(*self, Self::SelfLink)
    }

    /**
     * Returns whether `self` is `PathComponent::ParentLink`
     */
    pub fn is_parent_link(&self) -> bool {
        matches!(*self, Self::ParentLink)
    }

    /**
     * Returns whether `self` is `PathComponent::ObjectName`
     */
    pub fn is_object_name(&self) -> bool {
        matches!(&self, Self::ObjectName(_))
    }

    /**
     * Returns whether the component need a separator
     */
    pub fn need_separator_before(&self) -> bool {
        !self.is_root()
    }

    /**
     * Returns the `String` representation for this `PathComponent`
     */
    pub fn as_string(&self) -> String {
        match self {
            Self::Root => Self::SEPARATOR.to_string(),
            Self::SelfLink => Self::SELF_LINK.to_string(),
            Self::ParentLink => Self::PARENT_LINK.to_string(),
            Self::ObjectName(obj_name) => obj_name.to_string()
        }
    }

    /**
     * Returns the length in bytes of `String` representation for this
     * `PathComponent`
     */
    pub fn len(&self) -> usize {
        match self {
            Self::Root => Self::SEPARATOR.len(),
            Self::SelfLink => Self::SELF_LINK.len(),
            Self::ParentLink => Self::PARENT_LINK.len(),
            Self::ObjectName(obj_name) => obj_name.len()
        }
    }
}

impl From<&str> for PathComponent {
    fn from(str_path_component: &str) -> Self {
        match str_path_component {
            Self::SEPARATOR => Self::Root,
            Self::SELF_LINK => Self::SelfLink,
            Self::PARENT_LINK => Self::ParentLink,
            _ => Self::ObjectName(String::from(str_path_component))
        }
    }
}

impl From<String> for PathComponent {
    fn from(string_path_component: String) -> Self {
        Self::from(string_path_component.as_str())
    }
}

impl Display for PathComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Root => write!(f, "{}", Self::SEPARATOR),
            Self::SelfLink => write!(f, "{}", Self::SELF_LINK),
            Self::ParentLink => write!(f, "{}", Self::PARENT_LINK),
            Self::ObjectName(obj_name) => write!(f, "{}", obj_name)
        }
    }
}
