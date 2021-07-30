use alloc::string::String;
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::ops::{Add, Bound, Deref};
use core::ptr::NonNull;

use api_data::path::PathComponent;

use crate::filesystem::r#virtual::INode;
use alloc::boxed::Box;
use alloc::alloc::Global;

type Link<T> = Option<NonNull<T>>;

pub enum PartialNodeResult {
    None,
    Found(Arc<dyn INode>),
    Ancestor(Arc<dyn INode>, usize),
}

/**
 * Support struct to be used as key.
 * Inverts the string ordering, such that ancestors come after child paths.
 */
struct PathWrapper {
    _string: String,    // The path as a string.
    _separators: usize  // The number of Path::SEPARATOR in the string.
}

/**
 * A node cache entry that is also a node of a doubly-linked list of entries that don't own each
 * other.
 */
struct CacheEntry {
    next: Link<CacheEntry>,
    prev: Link<CacheEntry>,
    value: Arc<dyn INode>
}

pub trait NodeTreeMap {
    fn set(&mut self, path: &[PathComponent], node: &Arc<dyn INode>);

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
pub struct NodeTable {
    _map: BTreeMap<PathWrapper, Arc<dyn INode>>
}

/**
 * WARNING: doesn't support parent or self path links and does not check for them: it will produce
 * wacky results if fed any!
 */
pub struct NodeCache<'a> {
    _map: BTreeMap<PathWrapper, Box<CacheEntry>>,
    _list_head: Link<CacheEntry>,
    _list_back: Link<CacheEntry>,
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

impl CacheEntry {
    pub fn new(value: &Arc<dyn INode>) -> Self {
        Self {
            next: None,
            prev: None,
            value: value.clone()
        }
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
    unsafe fn unlink(mut entry: NonNull<CacheEntry>) {
        if let Some(mut prev) = entry.as_mut().prev {
            prev.as_mut().next = entry.as_ref().next;
            entry.as_mut().prev = None;
        }
        if let Some(mut next) = entry.as_mut().next {
            next.as_mut().prev = entry.as_ref().prev;
            entry.as_mut().next = None;
        }
    }

    /**
     * Pushes an entry to the top of the list.
     */
    unsafe fn move_as_head(&mut self, mut entry: NonNull<CacheEntry>) {
        Self::unlink(entry);
        entry.as_mut().prev = None;
        if let Some(mut old_head) = self._list_head {
            old_head.as_mut().prev = Some(entry);
            entry.as_mut().next = Some(old_head);
            self._list_head = Some(entry);
        } else {
            entry.as_mut().next = None;
            self._list_head = Some(entry);
            self._list_back = Some(entry);
        }
    }

    pub fn new(capacity: usize) -> Self {
        Self {
            _map: BTreeMap::new(),
            _list_head: None,
            _list_back: None,
            _capacity: capacity
        }
    }

    pub fn capacity(&self) -> usize {
        self._capacity
    }

    pub fn set_capacity(&mut self) {
        todo!()
    }

    pub fn count(&self) -> usize {
        self._map.len()
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
        let path = PathWrapper::new(path);
        let entry = Box::new(CacheEntry::new(node));
        // Put the new entry at the top of the list.
        self.push_list(entry.as_ref());

        match self._map.insert(path, entry) {
            None => { }
            Some(old_entry) => {
                // Unlink the old value from the rest of the list
            }
        }
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
