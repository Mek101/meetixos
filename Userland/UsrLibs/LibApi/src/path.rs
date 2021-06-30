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

use api_data::{
    path::PathExistsState,
    sys::{
        codes::KernPathFnId,
        fn_path::KernFnPath
    }
};

use crate::handle::{
    KernHandle,
    Result
};

/**
 * Safe filesystem path wrapper
 */
#[derive(Debug)]
#[derive(Clone)]
pub struct Path {
    m_components: Vec<PathComponent>,
    m_str_len: usize
}

impl Path {
    /**
     * Pushes `string_path` to this `Path`.
     *
     * If the `string_path` contain an absolute path the `Path`'s content is
     * cleared and re-evaluated
     */
    pub fn push_string(&mut self, string_path: String) {
        self.push(string_path.as_str())
    }

    /**
     * Pushes `str_path` to this `Path`.
     *
     * If the `str_path` contain an absolute path the `Path`'s content is
     * cleared and re-evaluated
     */
    pub fn push(&mut self, str_path: &str) {
        /* check whether the string path given starts with the separator (root) */
        if str_path.starts_with(PathComponent::SEPARATOR) {
            self.do_push_component(PathComponent::Root)
        }

        /* split the string path components given and add to the vector */
        for str_path_component in
            str_path.split(PathComponent::SEPARATOR)
                    .filter(|path_component| !path_component.is_empty())
        {
            /* push the component into the components vector */
            self.do_push_component(PathComponent::from(str_path_component))
        }
    }

    /**
     * Wipes out any `PathComponent::SelfLink`s and tries to lexically
     * resolve the `PathComponent::ParentLink`s without any request to the
     * kernel.
     *
     * A normalized path have no existence guarantees
     */
    pub fn normalize(&mut self) {
        /* start from an empty path */
        let mut normalized_path = Path::default();

        /* iterate each component currently stored into this instance */
        for path_component in self.iter() {
            match path_component {
                PathComponent::Root | PathComponent::ObjectName(_) => {
                    /* root and standard components are always added */
                    normalized_path.do_push_component(path_component.clone())
                },
                PathComponent::ParentLink => {
                    if normalized_path.is_empty() {
                        /* when the path is still empty (is relative) add the self link
                         * component as Root and ObjectName
                         */
                        normalized_path.do_push_component(path_component.clone());
                    } else if let Some(last_path_component) =
                        normalized_path.m_components.last()
                    {
                        /* pop the last component if is an ObjectName component */
                        if last_path_component.is_object_name() {
                            normalized_path.pop_component();
                        }
                    }
                },
                PathComponent::SelfLink => { /* self links are wiped out */ }
            }
        }

        /* when no values are present put the SelfLink */
        if normalized_path.is_empty() {
            normalized_path.do_push_component(PathComponent::SelfLink);
        }

        /* overwrite self with the normalized path */
        *self = normalized_path;
    }

    /**
     * Removes the last component and returns it
     */
    pub fn pop_component(&mut self) -> Option<PathComponent> {
        self.m_components.pop().map(|path_component| {
                                   /* calculate the length to subtract */
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

    /**
     * Returns the `PathExistsState` for this `Path`
     */
    pub fn exists(&self) -> Result<PathExistsState> {
        let string_repr = self.as_string();
        let mut path_exist_state = PathExistsState::EmptyPath;

        KernHandle::kern_call_3(KernFnPath::Path(KernPathFnId::Exists),
                                string_repr.as_ptr() as usize,
                                string_repr.len(),
                                path_exist_state.as_syscall_ptr()).map(|_| {
                                                                      path_exist_state
                                                                  })
    }

    /**
     * Wipes out the content of this `Path`
     */
    pub fn clear(&mut self) {
        self.m_components.clear();
        self.m_str_len = 0;
    }

    /**
     * Iterates the `PathComponent`s inside this `Path`
     */
    pub fn iter(&self) -> impl Iterator<Item = &PathComponent> {
        self.m_components.iter()
    }

    /**
     * Returns the lexical `String` representation of this `Path`
     */
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

    /**
     * Returns the amount of `PathComponents` stored into this `Path`
     */
    pub fn components_len(&self) -> usize {
        self.m_components.len()
    }

    /**
     * Returns the length in bytes of the lexical `String` representation
     */
    pub fn len(&self) -> usize {
        self.m_str_len
    }

    /**
     * Returns whether this `Path` doesn't contains any `PathComponent`
     */
    pub fn is_empty(&self) -> bool {
        self.m_components.is_empty()
    }

    /**
     * Effectively append the given path_component
     */
    fn do_push_component(&mut self, path_component: PathComponent) {
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
}

impl Default for Path {
    fn default() -> Self {
        Self { m_components: Vec::new(),
               m_str_len: 0 }
    }
}

impl From<&str> for Path {
    fn from(str_path: &str) -> Self {
        let mut path = Path::default();
        path.push(str_path);
        path
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

/**
 * Lists the possibly parts of a lexical path
 */
#[derive(Debug)]
#[derive(Clone)]
pub enum PathComponent {
    Root,
    SelfLink,
    ParentLink,
    ObjectName(String)
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
