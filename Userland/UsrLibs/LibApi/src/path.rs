/*! Path Management */

use alloc::{
    string::String,
    vec::Vec
};
use core::{
    fmt,
    fmt::Display
};

use api_data::{
    path::{
        PathComponent,
        PathExistsState
    },
    sys::{
        codes::KernPathFnId,
        fn_path::KernFnPath,
        AsSysCallPtr
    }
};

use crate::kern_handle::{
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
                                path_exist_state.as_syscall_ptr_mut()).map(|_| {
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
     * Returns the raw `PathComponent` slice
     */
    pub(crate) fn as_raw_components(&self) -> &[PathComponent] {
        self.m_components.as_slice()
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
