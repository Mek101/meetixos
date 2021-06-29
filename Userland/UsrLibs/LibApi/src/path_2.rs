/*! Path Management */

use core::{
    fmt,
    fmt::Display
};
use std::{
    string::{
        String,
        ToString
    },
    vec::Vec
};

#[derive(Debug)]
pub struct Path {
    m_components: Vec<PathComponent>,
    m_str_len: usize
}

impl Path {
    pub fn push_string(&mut self, string_path: String, normalize_components: bool) {
        self.push(string_path.as_str(), normalize_components)
    }

    pub fn push(&mut self, str_path: &str, normalize_components: bool) {
        /* check whether the string path given starts with the separator (root) */
        if str_path.starts_with(PathComponent::SEPARATOR) {
            self.push_component_fast(PathComponent::Root)
        }

        /* split the string path components given and add to the vector */
        for str_path_component in
            str_path.split(PathComponent::SEPARATOR)
                    .filter(|path_component| !path_component.is_empty())
        {
            let path_component = PathComponent::from(str_path_component);

            /* push the component into the components vector */
            if normalize_components {
                self.push_component_normalize(path_component)
            } else {
                self.push_component_fast(path_component)
            }
        }
    }

    pub fn push_component_fast(&mut self, path_component: PathComponent) {
        /* root component resets the Path instance */
        if path_component.is_root() {
            self.clear();
        }

        /* add the component and update the instance fields */
        self.m_components.push(path_component.clone());
        self.m_str_len += path_component.len();

        /* add the length of the separator when needed */
        if path_component.need_separator_before() {
            self.m_str_len += PathComponent::SEPARATOR.len();
        }
    }

    pub fn push_component_normalize(&mut self, path_component: PathComponent) {
        if self.is_empty() || path_component.is_root() {
            self.push_component_fast(path_component)
        } else {
            if path_component.is_object_name() {
                self.push_component_fast(path_component);
            } else if path_component.is_parent_link() {
                /* at least one component exists due to !self.is_empty() */
                match self.m_components
                          .last()
                          .expect("Failed to obtain last Path component")
                {
                    PathComponent::Root => { /* nothing to do */ },
                    PathComponent::SelfLink => {
                        /* pop out this self link from the path */
                        self.pop_component();

                        while self.components_len() >= 1 {
                            let last_component_path =
                                self.m_components
                                    .last()
                                    .expect("Failed to obtain last component");

                            if last_component_path.is_object_name()
                               || last_component_path.is_self_link()
                            {
                                self.pop_component();
                            } else {
                                break;
                            }
                        }
                    },
                    PathComponent::ParentLink => {
                        /* pop the last component */
                        self.m_components.pop();

                        /* if the path contains more than one component (which is not
                         * the root or a parent link) pop another time to remove the
                         * previous component which refers the same component of the
                         * component to add
                         */
                        if self.components_len() >= 1 {
                            /* obtain the new last component */
                            let last_path_component =
                                self.m_components
                                    .last()
                                    .expect("Failed to obtain last Path component");

                            /* pop another time */
                            if !last_path_component.is_root()
                               && !last_path_component.is_parent_link()
                            {
                                self.m_components.pop();
                            }
                        }
                    },
                    PathComponent::ObjectName(_) => {
                        self.m_components.pop();
                    }
                }
            }

            /* PathComponent::SelfLink is explicitly ignored because it
             * doesn't add any useful information to the Path */
        }
    }

    pub fn pop_component(&mut self) -> Option<PathComponent> {
        self.m_components.pop().map(|path_component| {
                                   let len_to_subtract =
                                       if path_component.need_separator_before() {
                                           path_component.len() + 1
                                       } else {
                                           path_component.len()
                                       };

                                   self.m_str_len -= len_to_subtract;

                                   path_component
                               })
    }

    pub fn clear(&mut self) {
        self.m_components.clear();
        self.m_str_len = 0;
    }

    pub fn iter(&self) -> impl Iterator<Item = &PathComponent> {
        self.m_components.iter()
    }

    pub fn as_string(&self) -> String {
        let mut path_string = String::with_capacity(self.m_str_len);

        /* compose the <Path> as <String> */
        for (i, path_component) in self.m_components.iter().enumerate() {
            path_string += path_component.as_string().as_str();
            if path_component.need_separator_before() && i < self.m_components.len() - 1 {
                path_string += PathComponent::SEPARATOR;
            }
        }
        path_string
    }

    pub fn components_len(&self) -> usize {
        self.m_components.len()
    }

    pub fn len(&self) -> usize {
        self.m_str_len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for Path {
    fn default() -> Self {
        Self { m_components: Vec::new(),
               m_str_len: 0 }
    }
}

impl From<&str> for Path {
    fn from(str_path: &str) -> Self {
        let mut path_inst = Path::default();

        path_inst.push(str_path, false);
        path_inst
    }
}

impl From<String> for Path {
    fn from(string_path: String) -> Self {
        Self::from(string_path.as_str())
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[derive(Debug)]
#[derive(Clone)]
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

    pub fn need_separator_before(&self) -> bool {
        !self.is_root()
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
