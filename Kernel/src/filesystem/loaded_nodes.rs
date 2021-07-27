use alloc::collections::HashMap;
use alloc::vec::Vec;
use core::lazy::Lazy;

use crate::filesystem::r#virtual::INode;

use sync::rw_lock::{RwLock, spin_rw_lock::RawSpinRwLock};
use api_data::path::{PathComponent, PathExistsState};
use alloc::sync::Arc;
use core::ops::Range;
use core::cmp::min;

type INodeLockMap<'a> = RwLock<RawSpinRwLock, HashMap<&'a [PathComponent], &'a dyn INode>>;

pub enum INodeSearchResult<E> {
    Err(E),
    Found(dyn INode),
    FoundWithParent(dyn INode, dyn INode),
}

enum PartialINodeSearchResult<'a> {
    None,
    Found(&'a Arc<dyn INode>),
    NewBestScore(usize, &'a Arc<dyn INode>),
}

pub static LOADED_NODES: Lazy<LoadedNodes> = Lazy::new(|| {
    LoadedNodes {
        _opened_nodes: RwSpinLock::new(HashMap::new()).unwrap(),
        _cached_nodes: RwSpinLock::new(HashMap::new()).unwrap(),
        _filesystem_roots: RwSpinLock::new(FsRoots {}).unwrap(),
    }
});

pub struct LoadedNodes<'a> {
    _opened_nodes: INodeLockMap<'a>,
    _cached_nodes: INodeLockMap<'a>,
    _filesystem_roots: RwLock<RawSpinRwLock, FsRoots>,
}

struct FsRoots {}

impl LoadedNodes {
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

    fn search_inode_map(map: INodeLockMap, path: &[PathComponent], mut best_score: usize, mut nearest_node: &Arc<&dyn INode>) -> PartialINodeSearchResult {
        let mut found_new_best = false;

        {
            let nodes = map.lock();
            for (path, node) in nodes.iter() {
                if let Ok((ancestor, reverse_depth)) = LoadedNodes::get_minimum_common_ancestor(path, node, best_score) {
                    // Found in the map!
                    if reverse_depth == 0 {
                        PartialINodeSearchResult::Found(ancestor)
                    }
                    // Still a nearer node than before? Good enough.
                    else if reverse_depth < best_score {
                        found_new_best = true;
                        best_score = reverse_depth;
                        nearest_node = ancestor;
                    }
                }
            }
        }

        if found_new_best {
            PartialINodeSearchResult::NewBestScore(best_score, nearest_node)
        } else {
            PartialINodeSearchResult::None
        }
    }

    /**
     * Find an INode by path by checking the cache, opened nodes, roots and ultimately, walking the
     * VFS tree.
     * Only handles absolute paths.
     * This is thread safe.
     */
    pub fn find_node(&self, path: &Vec<PathComponent>) -> INodeSearchResult<()> {
        // Empty paths, non-absolute paths and maths with multiple root paths(?) are not supported.
        if path.len() == 0 || path[0] != PathComponent::Root || path[1..].contains(&PathComponent::Root) {
            return INodeSearchResult::Err(());
        }

        // The number of backwards iterations the current best match had.
        let mut best_score = usize::MAX;
        // The nearest node found to the target path. Starts as the system root.
        let mut nearest_node = {
            let fs_roots = self._filesystem_roots.read();
            fs_roots.get_vfs_tree_root()
        };

        // Are we lucky?
        if let Ok(root) = path.last() {
            if root == PathComponent::Root {
                return INodeSearchResult::Found(nearest_node);
            }
        }

        // First, search the opened nodes
        {
            let opened_nodes = self._opened_nodes.read();
            match LoadedNodes::search_inode_map(opened_nodes, path, best_score, nearest_node) {
                PartialINodeSearchResult::None => {}
                PartialINodeSearchResult::Found(node) => return INodeSearchResult::Found(node),
                PartialINodeSearchResult::NewBestScore(new_best, new_nearest) => {
                    best_score = new_best;
                    nearest_node = new_nearest;
                }
            }
        }

        // Second, search the cache
        {
            let cached_nodes = self._cached_nodes.read();
            match LoadedNodes::search_inode_map(cached_nodes, path, best_score, nearest_node) {
                PartialINodeSearchResult::None => {}
                PartialINodeSearchResult::Found(node) => return INodeSearchResult::Found(node),
                PartialINodeSearchResult::NewBestScore(new_best, new_nearest) => {
                    best_score = new_best;
                    nearest_node = new_nearest;
                }
            }
        }

        // Third, search the filesystem roots.

        // Fourth, walk the filesystem tree form the nearest node found.


        INodeSearchResult::Err(())
    }
}

impl FsRoots {
    pub fn get_vfs_tree_root() -> Arc<&dyn INode> {
        unimplemented!();
    }
}