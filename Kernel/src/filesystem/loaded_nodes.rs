use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cmp;
use core::mem;
use core::lazy::Lazy;
use core::ops::Range;

use api_data::path::{PathComponent, PathExistsState};
use sync::SpinRwLock;

use sync::rw_lock::data_guard::RwLockDataReadGuard;
use sync::rw_lock::spin_rw_lock::RawSpinRwLock;

use crate::filesystem::{Filesystem, FsError};
use crate::filesystem::r#virtual::{INode, DirectoryNode};
use crate::filesystem::node_structs::{NodeTable, NodeTreeMap, PartialNodeResult};

type INodeLockMap<'a> = SpinRwLock<HashMap<&'a [PathComponent], dyn INode>>;

pub type INodeResult = Result<Arc<dyn INode>, ()>;

enum PartialSearchResult<T> {
    None,
    Found(T),
    NewBestScore(usize, T),
}

pub static LOADED_NODES: Lazy<LoadedNodes> = Lazy::new(|| {
    LoadedNodes {
        _opened_nodes: SpinRwLock::new(NodeTable::new()).unwrap(),
        _cached_nodes: INodeLockMap::new(HashMap::new()).unwrap(),
        _filesystem_roots: SpinRwLock::new(FsRoots::new()).unwrap(),
    }
});

pub struct LoadedNodes<'a> {
    _opened_nodes: SpinRwLock<NodeTable<'a>>,
    _cached_nodes: INodeLockMap<'a>,
    _filesystem_roots: SpinRwLock<FsRoots<'a>>,
}

struct FsRoots<'a> {
    _filesystem_mountpoints: HashMap<&'a [PathComponent], dyn Filesystem>,
}

/**
 * Returns the minimum common ancestor between a Path and an INode, and how many Path components
 * are necessary to go from the common ancestor to the path.
 * // Find and fix this formatting:
 *      param max_inverse_depth: optimization, after the given depth, consider the search failed
 *                               and pass to the next. If more than the path length, defaults to
 *                               it.
 */
fn get_minimum_common_ancestor(path: &[PathComponent], mut node: Arc<&dyn INode>, max_inverse_depth: usize) -> Result<(&Arc<&dyn INode>, usize), ()> {
    let mut minimum_ancestor = None;
    let mut minimum_inverse_depth = cmp::min(max_inverse_depth, path.len()); // Start from the max

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
                 *   after the node.
                 * - If the path and the node have nothing in common but a few middle path
                 *   components, (eg /bar/FOO/file and /faz/fuz/FOO/something), this removes the
                 *   false matches.
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
            NodeType::File => Err(()),
            NodeType::Directory => {
                // Search for the next node
                let as_directory: &dyn DirectoryNode = ancestor.as_type().unwrap();
                match as_directory.get_nodes() {
                    Ok(nodes) => {
                        match nodes.iter().find(|n| *n.get_name() == ancestor[depth_index]) {
                            Some(n) => ancestor = n, // Our new ancestor.
                            None => Err(())
                        }
                    }
                    Err(_) => Err(())
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

    match nearest_node {
        Some(n) => PartialINodeSearchResult::NewBestScore(best_score, n),
        None => PartialINodeSearchResult::None
    }
}

impl LoadedNodes {
    fn search_node_in_map(map: &impl NodeTreeMap, path: &[PathComponent], best_score: usize) -> PartialNodeResult {
        map.get_best_match(path, best_score)
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

        // Lock the filesystem roots table and then get the filesystem root inode since it must not
        // change or be unmounted while we're searching.
        let node_filesystem: Arc<&dyn INode> = {
            let roots = self._filesystem_roots.read();
            let filesystem = roots.get_filesystem_of(path)?;
            // Ensure the node's filesystem is locked before dropping the Filesystem table guard.
            let node = filesystem.get_root_node();
            mem::drop(roots);
            node
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
            let opened_nodes  = self._opened_nodes.read();
            match Self::search_node_in_map(&opened_nodes, path, best_score) {
                PartialINodeSearchResult::None => {}
                PartialINodeSearchResult::Found(node) => Ok(node.clone()),
                PartialINodeSearchResult::NewBestScore(new_best, new_nearest) => {
                    best_score = new_best;
                    nearest_node = new_nearest.clone();
                }
            }
        }

        // Third, search the cache
        {
            let cached_nodes = self._cached_nodes.read();
            match Self::search_node_in_map(&cached_nodes, path, best_score) {
                PartialINodeSearchResult::None => {}
                PartialINodeSearchResult::Found(node) => Ok(node.clone()),
                PartialINodeSearchResult::NewBestScore(new_best, new_nearest) => {
                    best_score = new_best;
                    nearest_node = new_nearest.clone();
                }
            }
        }

        // Fourth, walk from the node's filesystem root.
        let result = find_child_from_ancestor(&nearest_node, path)?;

        // Add the parent of the inode to the cache.
        // TODO: when supported, set a maximum timer to keep things speedy
        {
            let cached_nodes = self._cached_nodes.write();
            cached_nodes.insert(path, result.get_parent()?);
        }
        INodeResult::Ok(result)
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

    pub fn get_filesystem_of(self, path: &[PathComponent]) -> Result<Arc<dyn Filesystem>, ()> {
        match self._filesystem_mountpoints.get(path) {
            Some(fs) => Ok(fs),
            None => {
                let nodes = self._filesystem_mountpoints.iter().values().map(|fs| fs.get_root_node());
                match search_ancestor_node(nodes, path, usize::MAX) {
                    PartialSearchResult::Found(n) => Ok(n.get_filesystem()),
                    _ => Err(())
                }
            }
        }
    }
}