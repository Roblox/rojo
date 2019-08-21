use std::{
    path::{self, Path, PathBuf},
    collections::{HashMap, HashSet},
};

use serde::Serialize;
use log::warn;

#[derive(Debug, Serialize)]
struct PathMapNode<T> {
    value: T,
    children: HashSet<PathBuf>,
}

/// A map from paths to another type, like instance IDs, with a bit of
/// additional data that enables removing a path and all of its child paths from
/// the tree more quickly.
#[derive(Debug, Serialize)]
pub struct PathMap<T> {
    nodes: HashMap<PathBuf, PathMapNode<T>>,

    /// Contains the set of all paths whose parent either does not exist, or is
    /// not present in the PathMap.
    ///
    /// Note that these paths may have other _ancestors_ in the tree, but if an
    /// orphan's parent path is ever inserted, it will stop being an orphan. It
    /// will be... adopted!
    orphan_paths: HashSet<PathBuf>,
}

impl<T> Default for PathMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PathMap<T> {
    pub fn new() -> PathMap<T> {
        PathMap {
            nodes: HashMap::new(),
            orphan_paths: HashSet::new(),
        }
    }

    pub fn get(&self, path: impl AsRef<Path>) -> Option<&T> {
        self.nodes.get(path.as_ref()).map(|v| &v.value)
    }

    pub fn get_mut(&mut self, path: impl AsRef<Path>) -> Option<&mut T> {
        self.nodes.get_mut(path.as_ref()).map(|v| &mut v.value)
    }

    pub fn children(&self, path: impl AsRef<Path>) -> Option<Vec<&Path>> {
        self.nodes.get(path.as_ref()).map(|v| v.children.iter().map(AsRef::as_ref).collect())
    }

    pub fn contains_key(&self, path: impl AsRef<Path>) -> bool {
        self.nodes.contains_key(path.as_ref())
    }

    pub fn insert(&mut self, path: impl Into<PathBuf>, value: T) {
        let path = path.into();

        self.add_to_parent(path.clone());

        // Collect any children that are currently marked as orphaned paths, but
        // are actually children of this new node.
        let mut children = HashSet::new();
        for orphan_path in &self.orphan_paths {
            if orphan_path.parent() == Some(&path) {
                children.insert(orphan_path.clone());
            }
        }

        for child in &children {
            self.orphan_paths.remove(child);
        }

        self.nodes.insert(path, PathMapNode {
            value,
            children,
        });
    }

    /// Remove the given path and all of its linked descendants, returning all
    /// values stored in the map.
    pub fn remove(&mut self, root_path: impl AsRef<Path>) -> Vec<(PathBuf, T)> {
        let root_path = root_path.as_ref();

        self.remove_from_parent(root_path);

        let (root_path, root_node) = match self.nodes.remove_entry(root_path) {
            Some(node) => node,
            None => return Vec::new(),
        };

        let mut removed_entries = vec![(root_path, root_node.value)];
        let mut to_visit: Vec<PathBuf> = root_node.children.into_iter().collect();

        while let Some(path) = to_visit.pop() {
            match self.nodes.remove_entry(&path) {
                Some((path, node)) => {
                    removed_entries.push((path, node.value));

                    for child in node.children.into_iter() {
                        to_visit.push(child);
                    }
                },
                None => {
                    warn!("Consistency issue; tried to remove {} but it was already removed", path.display());
                },
            }
        }

        removed_entries
    }

    /// Traverses the route between `start_path` and `target_path` and returns
    /// the path closest to `target_path` in the tree.
    ///
    /// This is useful when trying to determine what paths need to be marked as
    /// altered when a change to a path is registered. Depending on the order of
    /// FS events, a file remove event could be followed by that file's
    /// directory being removed, in which case we should process that
    /// directory's parent.
    pub fn descend(&self, start_path: impl Into<PathBuf>, target_path: impl AsRef<Path>) -> PathBuf {
        let start_path = start_path.into();
        let target_path = target_path.as_ref();

        let relative_path = target_path.strip_prefix(&start_path)
            .expect("target_path did not begin with start_path");
        let mut current_path = start_path;

        for component in relative_path.components() {
            match component {
                path::Component::Normal(name) => {
                    let next_path = current_path.join(name);

                    if self.nodes.contains_key(&next_path) {
                        current_path = next_path;
                    } else {
                        return current_path;
                    }
                },
                _ => unreachable!(),
            }
        }

        current_path
    }

    /// Adds the path to its parent if it's present in the tree, or the set of
    /// orphaned paths if it is not.
    fn add_to_parent(&mut self, path: PathBuf) {
        if let Some(parent_path) = path.parent() {
            if let Some(parent) = self.nodes.get_mut(parent_path) {
                parent.children.insert(path);
                return;
            }
        }

        // In this branch, the path is orphaned because it either doesn't have a
        // parent according to Path, or because its parent doesn't exist in the
        // PathMap.
        self.orphan_paths.insert(path);
    }

    /// Removes the path from its parent, or from the orphaned paths set if it
    /// has no parent.
    fn remove_from_parent(&mut self, path: &Path) {
        if let Some(parent_path) = path.parent() {
            if let Some(parent) = self.nodes.get_mut(parent_path) {
                parent.children.remove(path);
                return;
            }
        }

        // In this branch, the path is orphaned because it either doesn't have a
        // parent according to Path, or because its parent doesn't exist in the
        // PathMap.
        self.orphan_paths.remove(path);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use maplit::hashset;

    #[test]
    fn smoke_test() {
        let mut map = PathMap::new();

        assert_eq!(map.get("/foo"), None);
        map.insert("/foo", 5);
        assert_eq!(map.get("/foo"), Some(&5));

        map.insert("/foo/bar", 6);
        assert_eq!(map.get("/foo"), Some(&5));
        assert_eq!(map.get("/foo/bar"), Some(&6));
        assert_eq!(map.children("/foo"), Some(vec![Path::new("/foo/bar")]));
    }

    #[test]
    fn orphans() {
        let mut map = PathMap::new();

        map.insert("/foo/bar", 5);
        assert_eq!(map.orphan_paths, hashset!["/foo/bar".into()]);

        map.insert("/foo", 6);
        assert_eq!(map.orphan_paths, hashset!["/foo".into()]);
    }

    #[test]
    fn remove_one() {
        let mut map = PathMap::new();

        map.insert("/foo", 6);

        assert_eq!(map.remove("/foo"), vec![
            (PathBuf::from("/foo"), 6),
        ]);

        assert_eq!(map.get("/foo"), None);
    }

    #[test]
    fn remove_child() {
        let mut map = PathMap::new();

        map.insert("/foo", 6);
        map.insert("/foo/bar", 12);

        assert_eq!(map.remove("/foo"), vec![
            (PathBuf::from("/foo"), 6),
            (PathBuf::from("/foo/bar"), 12),
        ]);

        assert_eq!(map.get("/foo"), None);
        assert_eq!(map.get("/foo/bar"), None);
    }

    #[test]
    fn remove_descendant() {
        let mut map = PathMap::new();

        map.insert("/foo", 6);
        map.insert("/foo/bar", 12);
        map.insert("/foo/bar/baz", 18);

        assert_eq!(map.remove("/foo"), vec![
            (PathBuf::from("/foo"), 6),
            (PathBuf::from("/foo/bar"), 12),
            (PathBuf::from("/foo/bar/baz"), 18),
        ]);

        assert_eq!(map.get("/foo"), None);
        assert_eq!(map.get("/foo/bar"), None);
        assert_eq!(map.get("/foo/bar/baz"), None);
    }

    #[test]
    fn remove_not_orphan_descendants() {
        let mut map = PathMap::new();

        map.insert("/foo", 6);
        map.insert("/foo/bar/baz", 12);

        assert_eq!(map.remove("/foo"), vec![
            (PathBuf::from("/foo"), 6),
        ]);

        assert_eq!(map.get("/foo"), None);
        assert_eq!(map.get("/foo/bar/baz"), Some(&12));
    }
}