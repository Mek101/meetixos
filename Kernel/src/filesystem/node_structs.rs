use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::cmp::Ordering::{Equal, Greater, Less};
use core::ops::{Add, Bound, Deref};

use api_data::path::PathComponent;

use crate::filesystem::r#virtual::INode;

/**
 * Support struct to be used as key.
 * Inverts the string ordering, such that ancestors come after child paths.
 */
struct PathWrapper {
    _string: String,    // The path as a string.
    _separators: usize  // The number of Path::SEPARATOR in the string.
}

/**
 * WARNING: doesn't support parent or self path links and does not check for them: it will produce
 * wacky results if fed any!
 */
pub struct NodeTable<'a> {
    _map: BTreeMap<PathWrapper, Arc<&'a dyn INode>>
}

/**
 * WARNING: doesn't support parent or self path links and does not check for them: it will produce
 * wacky results if fed any!
 */
pub struct NodeCache<'a> {
    _map: BTreeMap<PathWrapper, Weak<&'a dyn INode>>
}

impl PathWrapper {
    pub fn new(path: &[PathComponent]) -> Self {
        let mut accumulator = String::new();
        path.iter().map(|p_comp| accumulator.add(&p_comp.as_string())).collect();
        accumulator.shrink_to_fit();

        Self {
            _string: accumulator,
            _separators: path.len(),
        }
    }
}

impl Ord for PathWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self._string.cmp(&other._string).reverse()
    }
}

impl PartialOrd for PathWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self._string.partial_cmp(&other._string)?.reverse())
    }
}

impl Eq for PathWrapper {}

impl PartialEq for PathWrapper {
    fn eq(&self, other: &Self) -> bool {
        self._string.eq(&self._string)
    }
}

impl NodeTable {
    /**
     * Returns true if the child is a sub-path of the ancestor.
     * Assumes the ancestor is smaller than the child. May panic if it's not.
     */
    fn is_ancestor(ancestor: &PathWrapper, child: &PathWrapper) -> bool {
        // Check that the ancestor string ends where the child has a path separator.
        // This rules out `/foo/ba` and `/foo/baz/file` cases.
        child._string[ancestor._string.len()] == PathComponent::SEPARATOR
            // Checks that the child's path string starts with the full ancestor.
            && child._string.starts_with(&ancestor._string)
    }

    pub fn new() -> Self {
        Self {
            _map: BTreeMap::new()
        }
    }

    pub fn set(&mut self, path: &[PathComponent], node: &Arc<&dyn INode>) {
        self._map.insert(path_to_str(path), node.clone());
    }

    pub fn get_exact(&self, path: &[PathComponent]) -> Option<&Arc<&dyn INode>> {
        self._map.get(&path_to_str(path))
    }

    /**
     * Returns the nearest ancestor to the given path, if any.
     */
    pub fn get_nearest_ancestor(&self, path: &[PathComponent]) -> Option<&Arc<&dyn INode>> {
        let path = PathWrapper::new(path);
        // Iterate up to the child.
        self._map.range((Bound::Unbounded, Bound::Excluded(path)))
            // The first path with less separators could be an ancestor (less characters only could
            // also include a sibling file/directory with shorter name).
            // Check that the ancestor's string path
            .find(|(k, _)| k._separators < path._separators && Self::is_ancestor(k, &path))
            .map(|_, ancestor: &&Arc<&dyn INode>| *ancestor)
    }

    pub fn remove(&mut self, path: &[PathComponent]) -> bool {
        self._map.remove(&path_to_str(path)) != None
    }
}

