use alloc::collections::HashMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cmp::min;
use core::lazy::Lazy;
use core::ops::Range;

use api_data::path::{PathComponent, PathExistsState};
use sync::SpinRwLock;

use crate::filesystem::{Filesystem, FsError};
use crate::filesystem::r#virtual::{INode, DirectoryNode};

type INodeLockMap<'a> = SpinRwLock<HashMap<&'a [PathComponent], &'a dyn INode>>;

pub type PartialINodeSearchResult<'a> = PartialSearchResult<&'a dyn INode>;

pub type INodeResult<'a> = Result<Arc<&'a dyn INode>, ()>;

enum PartialSearchResult<T> {
    None,
    Found(T),
    NewBestScore(usize, T),
}

pub static LOADED_NODES: Lazy<LoadedNodes> = Lazy::new(|| {
    LoadedNodes {
        _opened_nodes: INodeLockMap::new(HashMap::new()).unwrap(),
        _cached_nodes: INodeLockMap::new(HashMap::new()).unwrap(),
        _filesystem_roots: SpinRwLock::new(FsRoots::new()).unwrap(),
    }
});

pub struct LoadedNodes<'a> {
    _opened_nodes: INodeLockMap<'a>,
    _cached_nodes: INodeLockMap<'a>,
    _filesystem_roots: SpinRwLock<FsRoots<'a>>,
}

struct FsRoots<'a> {
    _filesystem_mountpoints: HashMap<&'a [PathComponent], &'a dyn Filesystem>,
}

/**
 * Returns the minimum common ancestor between a Path and an INode, and how many Path components
 * are necessary to go from the common ancestor to the path.
 * // Find and fix this formatting:
 *      param max_inverse_depth: optimization, after the given depth, consider the search failed
 *                               and pass to the next. If more than the path length, defaults to
 *                               it.
 */
fn get_minimum_common_ancestor(path: &[PathComponent], mut node: Arc<&dyn INode>, max_inverse_depth: usize) -> Result<(&Arc<&dyn Inode>, usize), ()> {
    let mut minimum_ancestor = None;
    let mut minimum_inverse_depth = min(max_inverse_depth, path.len()); // Start from the max

    /*
     * Iterate the nodes backwards to the system root.
     */
    loop {
        // Try to match the current node with the path component nearest to the target path.
        for path_inverse_depth in (0..minimum_inverse_depth).rev() {
            // They match, meaning the current node is part of out path's ancestry
            if *node.get_name() == path[path_inverse_depth] {
                // If we already have an ancestor, then it IS nearer to the path's target, since
                // we iterate backwards.
                if minimum_ancestor == None {
                    minimum_ancestor = Some(&node);
                    minimum_inverse_depth = path_inverse_depth;
                }

                // The node won't change inside this for loop, while keeping to iterate will
                // reset our ancestor, so break and iterate to the next node.
                break;
            }
            // Otherwise, reset the "false" ancestor, since it's own ancestors aren't in part of
            // our path's ancestry.
            // This should only be executed before we match a path with the current node,
            // otherwise we might reset a "true" ancestor.
            else {
                /*
                 * This will remove:
                 * - If our path is a super-path that includes the node, the path components
                 *   after the node
                 * - If the path and the node have nothing in common but a few middle path
                 *   components, (eg /bar/FOO/file and /faz/fuz/FOO/something), this removes the
                 *   false matches
                 */
                minimum_ancestor = None;
            }
        }

        match node.get_parent() {
            Some(n) => node = n, // Iterate to the next ancestor.
            None => {            // Node is the system root, return.
                if let Some(result) = minimum_ancestor {
                    Ok((result, minimum_inverse_depth))
                }
                // Something went wrong, since the path and the inode should _at least_ share
                // the system root.
                else {
                    Err(())
                }
            }
        }
    }
}

fn find_child_from_ancestor(mut ancestor: &Arc<&dyn INode>, path_to_child: &[PathComponent]) -> INodeResult {
    // Walk to the node iteratively, to prevent stack overflows.
    for depth_index in 0..path_to_child.len() - 1 {
        match ancestor.get_type() {
            NodeType::File => return Err(()),
            NodeType::Directory => {
                // Search for the next node
                let as_directory: &dyn DirectoryNode = ancestor.as_type().unwrap();
                match as_directory.get_nodes() {
                    Ok(nodes) => {
                        match nodes.iter().find(|n| *n.get_name() == ancestor[depth_index]) {
                            Some(n) => ancestor = n, // Our new ancestor.
                            None => return Err(())
                        }
                    }
                    Err(_) => return Err(())
                }
            }
        }
    }
    // We finally found it!
    if *ancestor.get_name() == path_to_child.last()? {
        Ok(ancestor)
    }
    Err(())
}

/**
 * Assumes the node indicated by the path isn't in the map.
 * Never returns the node targeted by the path, only the a nearer ancestor it can find from the map
 * values.
 */
fn search_ancestor_node(nodes: &impl Iterator<Item=Arc<&dyn INode>>, path: &[PathComponent], mut best_score: usize) -> PartialINodeSearchResult {
    let mut nearest_node: Optional<&Arc<&dyn INode>> = None;

    for node in nodes.iter() {
        if let Ok((ancestor, reverse_depth)) = get_minimum_common_ancestor(path, node, best_score) {
            if reverse_depth < best_score {
                best_score = reverse_depth;
                nearest_node = Some(ancestor);
            }
            // Can't get any better from searching, since we assume that we can't reach 0.
            if best_score == 1 {
                break;
            }
        }
    }

    return match nearest_node {
        Some(n) => PartialINodeSearchResult::NewBestScore(best_score, n),
        None => PartialINodeSearchResult::None
    }
}

impl LoadedNodes {
    fn search_node_in_map(map: &HashMap<&[PathComponent], Arc<&dyn INode>>, path: &[PathComponent], best_score: usize) -> PartialINodeSearchResult {
        return match map.get(path) {
            Some(node) => PartialINodeSearchResult::Found(node),
            None => search_ancestor_node(map.values(), path, best_score)
        }
    }

    /**
     * Find an INode by path by checking the cache, opened nodes, roots and ultimately, walking the
     * VFS tree.
     * Only handles absolute paths.
     * This is thread safe.
     */
    pub fn find_node(&self, path: &Vec<PathComponent>) -> INodeResult {
        // Empty paths, non-absolute paths and maths with multiple root paths(?) are not supported.
        if path.len() == 0 || path[0] != PathComponent::Root || path[1..].contains(&PathComponent::Root) {
            Err(())
        }

        // Lock the filesystem roots table and then the inode's filesystem root since it must not
        // change or be unmounted while we're searching.
        let (node_filesystem, filesystem_state_guard): (Arc<&dyn INode>, RwLockDataReadGuard<()>) = {
            let roots = self._filesystem_roots.read();
            let filesystem: &dyn Filesystem = roots.get_filesystem_of(path)?;
            let filesystem_state = roots.get_filesystem_state();
            // Ensure the node's filesystem is locked before dropping the Filesystem table guard.
            let guard = filesystem_state.read();
            (filesystem.get_root_node(), guard)
        };


        // The number of backwards iterations the current best match had.
        let mut best_score = usize::MAX;
        // The nearest node found to the target path. Starts at the node's filesystem implementation
        // root.
        let mut nearest_node = node_filesystem;

        // First, check if the path is the root of it's filesystem.
        if *path.last().unwrap() == PathComponent::Root {
            Ok(nearest_node)
        }

        // Second, search the opened nodes
        {
            let opened_nodes = self._opened_nodes.read();
            match self::search_node_in_map(opened_nodes, path, best_score) {
                PartialINodeSearchResult::None => {}
                PartialINodeSearchResult::Found(node) => return Ok(node.clone()),
                PartialINodeSearchResult::NewBestScore(new_best, new_nearest) => {
                    best_score = new_best;
                    nearest_node = new_nearest.clone();
                }
            }
        }

        // Third, search the cache
        {
            let cached_nodes = self._cached_nodes.read();
            match self::search_inode_map(cached_nodes, path, best_score) {
                PartialINodeSearchResult::None => {}
                PartialINodeSearchResult::Found(node) => return Ok(node.clone()),
                PartialINodeSearchResult::NewBestScore(new_best, new_nearest) => {
                    best_score = new_best;
                    nearest_node = new_nearest.clone();
                }
            }
        }

        // Fourth, walk from the node's filesystem root.
        let result = find_child_from_ancestor(&nearest_node, path);

        filesystem_state_guard.drop();
        result
    }
}

impl FsRoots {
    pub fn new() -> Self {
        Self {
            _filesystem_mountpoints: HashMap::new()
        }
    }

    pub fn get_vfs_tree_root(&self) -> Result<Arc<&dyn INode>, ()> {
        Ok(self.get_filesystem_of(&[PathComponent::Root])?.get_root_node())
    }

    pub fn as_path_inode_map(&self) -> impl Iterator<Item=Arc<&dyn INode>> {
        self._filesystem_mountpoints.iter().map(|(k, v)| {
            (k, v.get_root_node())
        })
    }

    pub fn get_filesystem_of(self, path: &[PathComponent]) -> Result<Arc<&dyn Filesystem>, ()> {
        return match self._filesystem_mountpoints.get(path) {
            Some(fs) => return Ok(fs),
            None => {
                let nodes = self._filesystem_mountpoints.iter().values().map(|fs| fs.get_root_node());
                match search_ancestor_node(nodes, path, usize::MAX) {
                    PartialSearchResult::Found(n) => return Ok(n.get_filesystem()),
                    _ => Err(())
                }
            }
        }
    }
}