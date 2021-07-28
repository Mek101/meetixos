use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::cmp::Ordering::{Equal, Greater, Less};
use core::ops::{Add, Bound, Deref};

use api_data::path::PathComponent;

use crate::filesystem::r#virtual::INode;

pub enum PartialNodeResult<'a> {
    None,
    Found(Arc<&'a dyn INode>),
    Ancestor(Arc<&'a dyn INode>, usize),
}

/**
 * Support struct to be used as key.
 * Inverts the string ordering, such that ancestors come after child paths.
 */
struct PathWrapper {
    _string: String,    // The path as a string.
    _separators: usize  // The number of Path::SEPARATOR in the string.
}

pub trait NodeTreeMap {
    fn set(&mut self, path: &[PathComponent], node: &Arc<&dyn INode>);

    fn get_exact(&self, path: &[PathComponent]) -> PartialNodeResult;

    /**
     * Returns the nearest ancestor to the given path and it's distance from it, if any.
     * Does not return the node from with the given path.
     */
    fn get_nearest_ancestor(&self, path: &[PathComponent], max_distance: usize) -> PartialNodeResult;

    /**
     * Returns the best match it can find from the given path and it's distance from it. IE: either
     * the node associated with the path, or the node with the nearest ancestor path.
     */
    fn get_best_match(&self, path: &[PathComponent], max_distance: usize) -> PartialNodeResult;

    fn remove(&mut self, path: &[PathComponent]) -> bool;
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
    _map: BTreeMap<PathWrapper, Weak<&'a dyn INode>>,
    _capacity: usize
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

    fn get_match_of<P>(&self, path: &PathWrapper, max_distance: usize, child_bound: Bound<PathWrapper>, predicate: P) -> PartialNodeResult
    where
        P: FnMut((&PathWrapper, &Arc<&dyn INode>)) -> bool
    {
        // The maximum number of separators in a path to be within the given maximum distance from
        // the child.
        let max_separators = path._separators - max_distance;

        match self._map.range((Bound::Unbounded, child_bound))
            .filter(|(path , _)| path._separators < max_separators)
            .find(predicate)
        {
            None => PartialNodeResult::None,
            Some((ancestor_path, ancestor)) => {
                let distance = path._separators - ancestor_path._separators;
                if distance == 0 {
                    PartialNodeResult::Found(ancestor.clone())
                } else {
                    PartialNodeResult::NewBestScore(ancestor.clone(), distance)
                }
            }
        }
    }

    pub fn new() -> Self {
        Self {
            _map: BTreeMap::new()
        }
    }
}

impl NodeTreeMap for NodeTable {
    fn set(&mut self, path: &[PathComponent], node: &Arc<&dyn INode>) {
        self._map.insert(path_to_str(path), node.clone());
    }

    fn get_exact(&self, path: &[PathComponent]) -> PartialNodeResult {
        match self._map.get(&path_to_str(path)) {
            None => PartialNodeResult::None,
            Some(node) => PartialNodeResult::Found(node.clone())
        }
    }

    /**
     * Returns the nearest ancestor to the given path, if any.
     */
    fn get_nearest_ancestor(&self, path: &[PathComponent], max_distance: usize) -> PartialNodeResult {
        let path = PathWrapper::new(path);
        // The first path with less separators could be an ancestor (less characters alone could
        // also include a sibling file/directory with shorter name).
        // Check that the ancestor's string path is a sub-path ot the child.
        self.get_match_of(&path, max_distance, Bound::Excluded(path), |(k, _)| {
            k._separators < path._separators && Self::is_ancestor(k, &path)
        })
    }

    fn get_best_match(&self, path: &[PathComponent], max_distance: usize) -> PartialNodeResult {
        let path = PathWrapper::new(path);
        // The first path with less or equal separators could be an ancestor or the path to the
        // child (less characters alone could also include a sibling file/directory with shorter
        // name).
        // Check that the current string path is a sub-path ot the child or the child itself.
        self.get_match_of(&path, max_distance, Bound::Included(path), |(k, _)| {
            if k._separators == path._separators {
                k._string == path._separators
            } else if k._separators < path._separators {
                Self::is_ancestor(k, &path)
            }
            false
        })
    }

    fn remove(&mut self, path: &[PathComponent]) -> bool {
        self._map.remove(&path_to_str(path)) != None
    }
}

impl NodeCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            _map: BTreeMap::new(),
            _capacity: capacity
        }
    }

    pub fn capacity(&self) -> usize {
        self._capacity
    }

    pub fn count(&self) -> usize {
        self._map.len()
    }

    /**
     * Remove any references still in the cache of dropped nodes.
     */
    pub fn clear_invalid(&mut self) {
        todo!()
    }

    pub fn clear(&mut self) {
        self._map.clear()
    }

    pub fn touch(&self, path: &[PathComponent]) {
        todo!()
    }
}

impl NodeTreeMap for NodeCache {
    fn set(&mut self, path: &[PathComponent], node: &Arc<&dyn INode>) {
        todo!()
    }

    fn get_exact(&self, path: &[PathComponent]) -> PartialNodeResult {
        todo!()
    }

    fn get_nearest_ancestor(&self, path: &[PathComponent], max_distance: usize) -> PartialNodeResult {
        todo!()
    }

    fn get_best_match(&self, path: &[PathComponent], max_distance: usize) -> PartialNodeResult {
        todo!()
    }

    fn remove(&mut self, path: &[PathComponent]) -> bool {
        todo!()
    }
}
