use alloc::{
    boxed::Box,
    collections::BTreeMap,
    string::String,
    sync::Arc,
    vec::Vec
};
use core::{
    cmp::Ordering,
    mem,
    ops::{
        Add,
        Bound,
        DerefMut
    },
    ptr
};

use intrusive_collections::{
    intrusive_adapter,
    Adapter,
    KeyAdapter,
    LinkedList,
    LinkedListLink,
    RBTree,
    RBTreeLink
};

use api_data::path::PathComponent;
use heap::slab::Slab;
use sync::SpinMutex;

pub enum PartialResult<'a, T> {
    None,
    Found(&'a T),
    Ancestor(&'a T, usize)
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
 * A node cache entry that is also a node of a doubly-linked and of rb-tree.
 */
struct CacheEntry<T> {
    list_link: LinkedListLink,
    tree_link: RBTreeLink,
    path: PathWrapper,
    value: T
}

pub trait PathMap<T> {
    fn set(&mut self, path: &[PathComponent], item: T);

    fn get_exact(&self, path: &[PathComponent]) -> PartialResult<T>;

    /**
     * Returns the nearest ancestor to the given path and it's distance from
     * it, if any. Does not return the node from with the given path.
     */
    fn get_nearest_ancestor(&self,
                            path: &[PathComponent],
                            max_distance: usize)
                            -> PartialResult<T>;

    /**
     * Returns the best match it can find from the given path and it's
     * distance from it. IE: either the node associated with the path,
     * or the node with the nearest ancestor path.
     */
    fn get_best_match(&self,
                      path: &[PathComponent],
                      max_distance: usize)
                      -> PartialResult<T>;

    fn remove(&mut self, path: &[PathComponent]) -> bool;
}

/**
 * WARNING: doesn't support parent or self path links and does not check for
 * them: it will produce wacky results if fed any!
 */
pub struct PathTable<T> {
    _map: BTreeMap<PathWrapper, T>
}

/**
 * WARNING: doesn't support parent or self path links and does not check for
 * them: it will produce wacky results if fed any!
 */
pub struct PathCache<T> {
    _mem: Box<[u8]>,
    _heap: Slab<mem::size_of::<CacheEntry<T>>()>,
    _map: RBTree<EntryTreeAdapter<T>>,
    _lru_list: SpinMutex<LinkedList<EntryListAdapter<T>>>,
    _count: usize,
    _capacity: usize
}

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

impl PathWrapper {
    pub fn new(path: &[PathComponent]) -> Self {
        let mut accumulator = String::new();
        path.iter().map(|p_comp| accumulator.add(&p_comp.as_string())).collect();
        accumulator.shrink_to_fit();

        Self { _string: accumulator,
               _separators: path.len() }
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

impl Eq for PathWrapper {
}

impl PartialEq for PathWrapper {
    fn eq(&self, other: &Self) -> bool {
        self._string.eq(&self._string)
    }
}

intrusive_adapter!(EntryListAdapter<T> = Box<CacheEntry<T>>: CacheEntry<T> { list_link: LinkedListLink });
intrusive_adapter!(EntryTreeAdapter<T> = Box<CacheEntry<T>>: CacheEntry<T> { tree_link: RBTreeLink });
impl<'a, T> KeyAdapter<'a> for EntryTreeAdapter<T> {
    type Key = &'a PathWrapper;

    fn get_key(&self, value: &'a CacheEntry<T>) -> Self::Key {
        &value.path
    }
}

impl<T> CacheEntry<T> {
    pub const SIZE: usize = mem::size_of::<Self>();

    pub fn new(path: &[PathComponent], value: Arc<T>) -> Self {
        Self { list_link: LinkedListLink::default(),
               tree_link: RBTreeLink::default(),
               path: PathWrapper::new(path),
               value }
    }
}

impl<T> PathTable<T> {
    fn get_match_of<P>(&self,
                       path: &PathWrapper,
                       max_distance: usize,
                       child_bound: Bound<PathWrapper>,
                       predicate: P)
                       -> PartialResult<T>
        where P: FnMut((&PathWrapper, &Arc<T>)) -> bool {
        // The maximum number of separators in a path to be within the given maximum
        // distance from the child.
        let max_separators = path._separators - max_distance;

        match self._map
                  .range((Bound::Unbounded, child_bound))
                  .filter(|(path, _)| path._separators < max_separators)
                  .find(predicate)
        {
            None => PartialResult::None,
            Some((ancestor_path, ancestor)) => {
                let distance = path._separators - ancestor_path._separators;
                if distance == 0 {
                    PartialResult::Found(ancestor.clone())
                } else {
                    PartialResult::Ancestor(ancestor.clone(), distance)
                }
            }
        }
    }

    pub fn new() -> Self {
        Self { _map: BTreeMap::new() }
    }
}

impl<T> PathMap<T> for PathTable<T> {
    fn set(&mut self, path: &[PathComponent], node: T) {
        self._map.insert(PathWrapper::new(path), node.clone());
    }

    fn get_exact(&self, path: &[PathComponent]) -> PartialResult<T> {
        match self._map.get(&PathWrapper::new(path)) {
            None => PartialResult::None,
            Some(node) => PartialResult::Found(node.clone())
        }
    }

    /**
     * Returns the nearest ancestor to the given path, if any.
     */
    fn get_nearest_ancestor(&self,
                            path: &[PathComponent],
                            max_distance: usize)
                            -> PartialResult<T> {
        let path = PathWrapper::new(path);
        // The first path with less separators could be an ancestor (less characters
        // alone could also include a sibling file/directory with shorter name).
        // Check that the ancestor's string path is a sub-path ot the child.
        self.get_match_of(&path, max_distance, Bound::Excluded(path), |(k, _)| {
                k._separators < path._separators && is_ancestor(k, &path)
            })
    }

    fn get_best_match(&self,
                      path: &[PathComponent],
                      max_distance: usize)
                      -> PartialResult<T> {
        let path = PathWrapper::new(path);
        // The first path with less or equal separators could be an ancestor or the path
        // to the child (less characters alone could also include a sibling
        // file/directory with shorter name).
        // Check that the current string path is a sub-path ot the child or the child
        // itself.
        self.get_match_of(&path, max_distance, Bound::Included(path), |(k, _)| {
                if k._separators == path._separators {
                    k._string == path._string
                } else if k._separators < path._separators {
                    is_ancestor(k, &path)
                }
                false
            })
    }

    fn remove(&mut self, path: &[PathComponent]) -> bool {
        self._map.remove(&PathWrapper::new(path)) != None
    }
}

impl<T> PathCache<T> {
    /**
     * Moves a cache entry already in the lru list to the top of the list.
     */
    fn move_to_front(list: &mut LinkedList<EntryListAdapter<T>>, entry: &CacheEntry<T>) {
        unsafe {
            list.push_front(list.cursor_mut_from_ptr(entry).remove()?);
        }
    }

    /**
     * Moves a cache entry already in the lru list to the top of the list.
     */
    fn try_move_to_front(&self, entry: &CacheEntry<T>) {
        if let Some(mut list) = self._lru_list.try_lock() {
            Self::move_to_front(list.deref_mut(), entry);
        }
    }

    fn allocate(&mut self, entry: CacheEntry<T>) -> Option<&CacheEntry<T>> {
        self._heap.allocate().map(|mem_ptr| unsafe {
                                 let mut entry_ptr = mem_ptr.cast::<CacheEntry<T>>();
                                 ptr::write(entry_ptr.as_mut(), entry);
                                 mem_ptr.as_ref()
                             })
    }

    fn evict_lru(&mut self) {
        if let Some(lru) = {
            let mut list = self._lru_list.lock();
            list.pop_back()
        } {
            unsafe {
                self._map.cursor_mut_from_ptr(lru).remove();
                self._heap.deallocate(lru);
            }
            self._count -= 1;
        }
    }

    fn get_match_of<P>(&self,
                       path: &PathWrapper,
                       max_distance: usize,
                       child_bound: intrusive_collections::Bound<&PathWrapper>,
                       predicate: P)
                       -> PartialResult<T> {
        // The maximum number of separators in a path to be within the given maximum
        // distance from the child.
        let max_separators = path._separators - max_distance;
        let cursor = self._map.lower_bound(child_bound);

        while let Some(entry) = cursor.get() {
            if entry.path._separators < max_separators {
                if predicate(entry) {
                    self.try_move_to_front(entry);
                    let distance = path._separators - entry.path._separators;
                    if distance == 0 {
                        PartialResult::Found(entry.value.clone())
                    } else {
                        PartialResult::Ancestor(entry.value.clone(), distance)
                    }
                }
            }
        }
        PartialResult::None
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let byte_size = mem::size_of::<CacheEntry<T>>() * capacity;
        let mut mem = Vec::with_capacity(byte_size).into_boxed_slice();
        unsafe {
            Self { _mem: mem,
                   _heap: Slab::new(mem.ptr(), byte_size),
                   _map: RBTree::new(EntryTreeAdapter::new()),
                   _lru_list:
                       SpinMutex::new(LinkedList::new(EntryListAdapter::<CacheEntry<T>>::new())).unwrap(),
                   _count: 0,
                   _capacity: capacity }
        }
    }

    pub fn capacity(&self) -> usize {
        self._capacity
    }

    pub fn byte_size(&self) -> usize {
        self._mem.len()
    }

    pub fn count(&self) -> usize {
        self._count
    }

    pub fn flush(&mut self) {
        self._map.fast_clear();
        let mut list = self._lru_list.lock();
        let mut cursor = list.front_mut();

        while let Some(ptr) = cursor.remove() {
            unsafe {
                ptr::drop_in_place(ptr);
                self._heap.deallocate(ptr);
            }
        }
        self._count = 0;
    }

    pub fn touch(&self, path: &[PathComponent]) -> bool {
        if let Some(entry_ptr) = self._map.find(&PathWrapper::new(path)).get() {
            let mut list = self._lru_list.lock();
            Self::move_to_front(list.deref_mut(), entry_ptr.as_ref());
            true
        }
        false
    }
}

impl<T> PathMap<T> for PathCache<T> {
    fn set(&mut self, path: &[PathComponent], node: Arc<T>) {
        let new_entry = CacheEntry::new(path, node.clone());
        while self._count >= self._capacity {
            self.evict_lru();
        }
        let entry = self.allocate(new_entry)?;

        // Inserting in the map first since it panics otherwise.
        self._map.insert(entry);
        // Locking the list.
        let mut lru_list = self._lru_list.lock();
        lru_list.push_front(entry);

        self._count += 1;
    }

    fn get_exact(&self, path: &[PathComponent]) -> PartialResult<T> {
        if let Some(entry_ptr) = self._map.find(&PathWrapper::new(path)).get() {
            let entry: &CacheEntry<T> = entry_ptr.as_ref();
            // Try to update the lru list, only if it's convenient.
            self.try_move_to_front(entry);

            PartialResult::Found(entry.value.clone())
        }
        PartialResult::None
    }

    fn get_nearest_ancestor(&self,
                            path: &[PathComponent],
                            max_distance: usize)
                            -> PartialResult<T> {
        let path = PathWrapper::new(path);
        self.get_match_of(&path,
                          max_distance,
                          intrusive_collections::Bound::Excluded(&path),
                          |entry| {
                              entry.path._separators < path._separators
                              && is_ancestor(&entry.path, &path)
                          })
    }

    fn get_best_match(&self,
                      path: &[PathComponent],
                      max_distance: usize)
                      -> PartialResult<T> {
        let path = PathWrapper::new(path);
        self.get_match_of(&path,
                          max_distance,
                          intrusive_collections::Bound::Included(&path),
                          |entry| {
                              if entry.path._separators == path._separators {
                                  entry.path._string == path._string
                              } else if entry.path._separators < path._separators {
                                  is_ancestor(&entry.path, &path)
                              }
                          })
    }

    fn remove(&mut self, path: &[PathComponent]) -> bool {
        if self._count >= self._capacity {
            if let Some(entry_ptr) =
                self._map.find(&PathWrapper::new(path)).clone_pointer().unwrap()
            {
                unsafe {
                    {
                        let mut list = self._lru_list.lock();
                        list.cursor_from_ptr(entry_ptr).remove()
                    }
                    self._map.cursor_mut_from_ptr(entry_ptr).remove();
                    self._heap.deallocate(entry_ptr)
                }
                return true;
            }
        }
        false
    }
}
