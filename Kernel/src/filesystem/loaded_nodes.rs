use alloc::collections::HashMap;
use alloc::vec::Vec;
use core::lazy::Lazy;

use crate::filesystem::r#virtual::INode;

use sync::rw_lock::{RwLock, spin_rw_lock::RawSpinRwLock};
use api_data::path::{PathComponent, PathExistsState};

type INodeLockMap<'a> = RwLock<RawSpinRwLock, HashMap<&'a [PathComponent], &'a dyn INode>>;

pub enum INodeSearchResult<E> {
    Err(E),
    Found(dyn INode),
    FoundWithParent(dyn INode, dyn INode),
}

enum PartialINodeSearchResult {
    None,
    Found(dyn INode),
    NewBestScore(usize, dyn INode)
}

pub static LOADED_NODES: Lazy<LoadedNodes> = Lazy::new(|| {
    LoadedNodes {
        _opened_nodes: RwSpinLock::new(Vec::new()).unwrap(),
        _cached_nodes: RwSpinLock::new(Vec::new()).unwrap(),
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
    fn search_inode_map(map: INodeLockMap, path: &[PathComponent], mut best_score: usize, mut nearest_node: Option<&dyn INode>) -> PartialINodeSearchResult {
        let nodes = map.lock();
        let mut found_new_best = false;
        for (index, path_component) in path.iter().rev().enumerate() {
            if let Ok(node) = nodes.get(path_component) {
                // Golden goal! We found it!
                if index == 0 {
                    PartialINodeSearchResult::Found(node)
                }
                // Else? Good enough.
                else if index < best_score {
                    found_new_best = true;
                    nearest_node = Some(node);
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
        // TODO: can't distinguish /foo/file from /bar/file, do something about it!
        // Empty paths and non-absolute paths are not supported.
        if path.len() == 0 || path[0] == PathComponent::Root {
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
        if path.last() == PathComponent::Root {
            return INodeSearchResult::Found(nearest_node);
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
    pub fn get_vfs_tree_root() -> &dyn INode {
        unimplemented!();
    }
}