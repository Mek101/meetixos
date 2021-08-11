use alloc::{
    sync::Arc,
    vec::Vec
};
use core::{
    lazy::Lazy,
    mem
};

use api_data::path::PathComponent;
use sync::SpinRwLock;

use crate::filesystem::{
    path_structs::{
        PartialResult,
        PartialResult::{
            Ancestor,
            Found
        },
        PathCache,
        PathMap,
        PathTable
    },
    r#virtual::{
        DirectoryNode,
        INode,
        NodeType
    },
    Filesystem
};

pub type INodeResult = Result<Arc<dyn INode>, ()>;

const CACHE_CAPACITY: usize = 1024;

pub static LOADED_NODES: Lazy<LoadedNodes> = Lazy::new(|| {
    LoadedNodes { _opened_nodes: SpinRwLock::new(PathTable::new()).unwrap(),
                  _cached_nodes:
                      SpinRwLock::new(PathCache::with_capacity(CACHE_CAPACITY)).unwrap(),
                  _filesystem_roots: SpinRwLock::new(FsRoots::new()).unwrap() }
});

pub struct LoadedNodes {
    _opened_nodes: SpinRwLock<PathTable<Arc<dyn INode>>>,
    _cached_nodes: SpinRwLock<PathCache<Arc<dyn INode>>>,
    _filesystem_roots: SpinRwLock<FsRoots>
}

struct FsRoots {
    _filesystem_mountpoints: PathTable<Arc<dyn Filesystem>>
}

/**
 * Returns the minimum common ancestor between a Path and an INode, and how
 * many Path components are necessary to go from the common ancestor to the
 * path. // Find and fix this formatting:
 *      param max_inverse_depth: optimization, after the given depth,
 * consider the search failed                               and pass to the
 * next. If more than the path length, defaults to                          
 * it.
 */
// fn get_minimum_common_ancestor(path: &[PathComponent],
//                                mut node: Arc<&dyn INode>,
//                                max_inverse_depth: usize)
//                                -> Result<(Arc<dyn INode>, usize), ()> {
//     let mut minimum_ancestor = None;
//     let mut minimum_inverse_depth = cmp::min(max_inverse_depth, path.len());
// // Start from the max
//
//     /*
//      * Iterate the nodes backwards to the system root.
//      */
//     loop {
//         // Try to match the current node with the path component nearest to
// the target         // path.
//         for path_inverse_depth in (0..minimum_inverse_depth).rev() {
//             // They match, meaning the current node is part of out path's
// ancestry             if *node.get_name() == path[path_inverse_depth] {
//                 // If we already have an ancestor, then it IS nearer to the
// path's target,                 // since we iterate backwards.
//                 if minimum_ancestor == None {
//                     minimum_ancestor = Some(&node);
//                     minimum_inverse_depth = path_inverse_depth;
//                 }
//
//                 // The node won't change inside this for loop, while keeping
// to iterate                 // will reset our ancestor, so break and iterate
// to the                 // next node.
//                 break;
//             }
//             // Otherwise, reset the "false" ancestor, since it's own
// ancestors aren't in             // part of our path's ancestry.
//             // This should only be executed before we match a path with the
// current node,             // otherwise we might reset a "true" ancestor.
//             else {
//                 /*
//                  * This will remove:
//                  * - If our path is a super-path that includes the node, the
//                    path
//                  * components after the node.
//                  * - If the path and the node have nothing in common but a
//                    few middle
//                  * path components, (eg /bar/FOO/file and
//                    /faz/fuz/FOO/something),
//                  * this removes the false matches.
//                  */
//                 minimum_ancestor = None;
//             }
//         }
//
//         match node.get_parent() {
//             Some(n) => node = n, // Iterate to the next ancestor.
//             None => {
//                 // Node is the system root, return.
//                 if let Some(result) = minimum_ancestor {
//                     Ok((result, minimum_inverse_depth))
//                 }
//                 // Something went wrong, since the path and the inode should
// _at least_                 // share the system root.
//                 else {
//                     Err(())
//                 }
//             }
//         }
//     }
// }

fn find_child_from_ancestor(ancestor: &Arc<&dyn INode>,
                            distance: usize,
                            path_to_child: &[PathComponent])
                            -> Result<(Arc<dyn INode>, Option<Arc<dyn INode>>), ()> {
    let mut current_ancestor = ancestor;
    let mut previous_ancestor = None;

    // Walk to the node iteratively, to prevent stack overflows.
    for depth_index in path_to_child.len() - distance..path_to_child.len() - 1 {
        match current_ancestor.get_type() {
            NodeType::File => Err(()),
            NodeType::Directory => {
                // Search for the next node
                let as_directory: &dyn DirectoryNode =
                    current_ancestor.as_type().unwrap();
                match as_directory.get_nodes() {
                    Ok(nodes) => {
                        match nodes.iter()
                                   .find(|n| *n.get_name() == path_to_child[depth_index])
                        {
                            Some(n) => {
                                previous_ancestor = Some(current_ancestor);
                                current_ancestor = n; // Our new ancestor.
                            },
                            None => Err(())
                        }
                    },
                    Err(_) => Err(())
                }
            }
        }
    }
    // We finally found it!
    if *current_ancestor.get_name() == path_to_child.last()? {
        Ok((current_ancestor, previous_ancestor))
    }
    Err(())
}

/**
 * Assumes the node indicated by the path isn't in the map.
 * Never returns the node targeted by the path, only the a nearer ancestor
 * it can find from the map values.
 */
// fn search_ancestor_node(nodes: &impl Iterator<Item = Arc<&dyn INode>>,
//                         path: &[PathComponent],
//                         mut best_score: usize)
//                         -> PartialResult {
//     let mut nearest_node: Optional<&Arc<&dyn INode>> = None;
//
//     for node in nodes.iter() {
//         if let Ok((ancestor, reverse_depth)) =
//             get_minimum_common_ancestor(path, node, best_score)
//         {
//             if reverse_depth < best_score {
//                 best_score = reverse_depth;
//                 nearest_node = Some(ancestor);
//             }
//             // Can't get any better from searching, since we assume that we
// can't reach 0.             if best_score == 1 {
//                 break;
//             }
//         }
//     }
//
//     match nearest_node {
//         Some(n) => PartialINodeSearchResult::NewBestScore(best_score, n),
//         None => PartialINodeSearchResult::None
//     }
// }

impl LoadedNodes {
    /**
     * Find an INode by path by checking the cache, opened nodes, roots and
     * as last resort, walking the VFS tree.
     * Only handles absolute paths.
     * This is thread safe.
     */
    pub fn find_node(&self, path: &Vec<PathComponent>) -> INodeResult {
        // Empty paths, non-absolute paths and maths with multiple root paths(?) are not
        // supported.
        if path.len() == 0
           || path[0] != PathComponent::Root
           || path[1..].contains(&PathComponent::Root)
        {
            Err(())
        }

        // Lock the filesystem roots table and then get the filesystem root inode since
        // it must not change or be unmounted while we're searching.
        let node_filesystem: Arc<&dyn INode> = {
            let roots = self._filesystem_roots.read();
            let filesystem = roots.get_filesystem_of(path)?;
            // Ensure the node's filesystem is locked before dropping the Filesystem table
            // guard.
            let node = filesystem.get_root_node();
            mem::drop(roots);
            node
        };

        // The number of backwards iterations the current best match had.
        let mut nearest_node_distance = usize::MAX;
        // The nearest node found to the target path. Starts at the node's filesystem
        // implementation root.
        let mut nearest_node = node_filesystem;

        // First, check if the path is the root of it's filesystem.
        if *path.last().unwrap() == PathComponent::Root {
            Ok(nearest_node)
        }

        // Second and third, try with the opened and cached nodes.
        for locked_nodes in [&self._opened_nodes, &self._cached_nodes] {
            let nodes = locked_nodes.read();
            match nodes.get_best_match(path, nearest_node_distance) {
                PartialResult::None => {},
                PartialResult::Found(node) => Ok(node.clone()),
                PartialResult::Ancestor(new_nearest, new_best) => {
                    nearest_node_distance = new_best;
                    nearest_node = new_nearest.clone();
                }
            }
        }

        // Fourth, walk from the node's nearest ancestor found.
        match find_child_from_ancestor(&nearest_node, nearest_node_distance, path) {
            Ok(result) => {
                if nearest_node_distance > 1 {
                    // Add the parent of the inode we just excavated to the cache.
                    let parent = result.1.unwrap();
                    // TODO: when supported, set a maximum timer to keep things speedy
                    let cached_nodes = self._cached_nodes.write();
                    cached_nodes.insert(path, parent);
                }
                INodeResult::Ok(result.0)
            },
            Err(_) => INodeResult::Err(())
        }
    }
}

impl FsRoots {
    pub fn new() -> Self {
        Self { _filesystem_mountpoints: PathTable::new() }
    }

    pub fn get_vfs_tree_root(&self) -> Result<Arc<dyn INode>, ()> {
        Ok(self.get_filesystem_at(&[PathComponent::Root])?.get_root_node())
    }

    pub fn as_path_inode_map(&self) -> impl Iterator<Item = Arc<dyn INode>> {
        //self._filesystem_mountpoints.iter().map(|(k, v)| (k, v.get_root_node()))
        unimplemented!()
    }

    pub fn get_filesystem_at(&self,
                             path: &[PathComponent])
                             -> Result<Arc<dyn Filesystem>, ()> {
        match self._filesystem_mountpoints.get_best_match(path, usize::MAX) {
            None => Err(()),
            Found(root) | Ancestor(root, _) => Ok(root.clone())
        }
    }
}
