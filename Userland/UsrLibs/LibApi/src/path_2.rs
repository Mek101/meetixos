/*! Path Management */

use alloc::{
    string::{
        String,
        ToString
    },
    vec::Vec
};
use core::{
    fmt,
    fmt::Display
};

#[derive(Debug)]
pub struct Path {
    m_components: Vec<PathComponent>
}

impl Path {
    pub fn as_string(&self) -> String {
        //let mut
    }

    pub fn len(&self) -> usize {
        let mut total_len = 0;
        for (i, path_component) in self.m_components.iter().enumerate() {
            total_len += path_component.len();

            /* add */
            if !path_component.is_root() && i < self.m_components.len() - 1 {
                total_len += 1;
            }
        }
        total_len
    }
}

impl Default for Path {
    fn default() -> Self {
        Self { m_components: Vec::new() }
    }
}

#[derive(Debug)]
pub enum PathComponent {
    Root,
    SelfLink,
    ParentLink,
    ObjectName(String)
}

impl PathComponent {
    pub const SELF_LINK: &'static str = ".";
    pub const PARENT_LINK: &'static str = "..";
    pub const SEPARATOR: &'static str = "/";

    pub fn is_root(&self) -> bool {
        matches!(*self, Self::Root)
    }

    pub fn is_self_link(&self) -> bool {
        matches!(*self, Self::SelfLink)
    }

    pub fn is_parent_link(&self) -> bool {
        matches!(*self, Self::ParentLink)
    }

    pub fn is_object_name(&self) -> bool {
        matches!(&self, Self::ObjectName(_))
    }

    pub fn as_string(&self) -> String {
        match self {
            Self::Root => Self::SEPARATOR.to_string(),
            Self::SelfLink => Self::SELF_LINK.to_string(),
            Self::ParentLink => Self::PARENT_LINK.to_string(),
            Self::ObjectName(obj_name) => obj_name.to_string()
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Root => Self::SEPARATOR.len(),
            Self::SelfLink => Self::SELF_LINK.len(),
            Self::ParentLink => Self::PARENT_LINK.len(),
            Self::ObjectName(obj_name) => obj_name.len()
        }
    }
}

impl PartialEq for PathComponent {
    fn eq(&self, other: &Self) -> bool {
    }
}

impl Eq for PathComponent {
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
