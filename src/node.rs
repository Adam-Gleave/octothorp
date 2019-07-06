use types::NodeLoc;

/// Enumeration representing child location in `OctreeNode<T>::children` field
#[repr(u8)]
#[derive(Copy, Clone)]
enum ChildLoc {
    BaseRearLeft = 0,
    BaseRearRight,
    BaseFrontRight,
    BaseFrontLeft,
    TopRearLeft,
    TopRearRight,
    TopFrontRight,
    TopFrontLeft,
}

impl ChildLoc {
    /// Create a default child location value (0: BaseFrontRight)
    fn default() -> ChildLoc {
        ChildLoc::BaseFrontRight
    }
}

/// OctreeNode structure (inaccessible outside module)
#[derive(Debug, Clone)]
pub struct OctreeNode<T> {
    dimension: u16,
    leaf: bool,
    simplified: bool,
    children: Vec<Option<OctreeNode<T>>>,
    data: Option<T>,
}

impl<T> OctreeNode<T>
where
    T: Copy + PartialEq,
{
    /// Constructs a new `OctreeNode<T>`.
    pub fn new(curr_dimension: u16, data: T) -> OctreeNode<T> {
        OctreeNode::<T> {
            dimension: curr_dimension / 2,
            leaf: true,
            simplified: false,
            children: no_children::<T>(),
            data: Some(data),
        }
    }

    /// Constructs a root `OctreeNode<T>` to be used in an `Octree<T>` structure
    pub fn construct_root(dimension: u16) -> OctreeNode<T> {
        OctreeNode {
            dimension,
            leaf: true,
            simplified: false,
            children: no_children::<T>(),
            data: None,
        }
    }

    /// Sets node `data` field
    pub fn set(&mut self, data: T) -> Result<(), String> {
        if self.leaf {
            self.data = Some(data);
            Ok(())
        } else {
            Err("Could not set octree node data: node is not a leaf".to_string())
        }
    }

    /// Get node `data` field
    pub fn get(&self) -> Option<T> {
        self.data
    }

    /// Get node children
    pub fn children(&self) -> Vec<Option<OctreeNode<T>>> {
        self.children.clone()
    }

    /// Algorithm to insert a new `OctreeNode<T>` into the tree
    pub fn insert(&mut self, loc: &mut NodeLoc, data: T) {
        let child_loc = self.get_child_loc(loc);
        let mut node = if self.children[child_loc as usize].is_some() && !self.simplified {
            self.children[child_loc as usize].take().unwrap()
        } else {
            OctreeNode::<T>::new(self.dimension, data)
        };

        if self.leaf && !self.simplified {
            self.make_leaf(false);
            self.data = None;
        }

        if self.dimension == 2 {
            node.make_leaf(true);
        } else {
            node.insert(loc, data);
        }

        if self.simplified && self.data != Some(data) {
            for i in 0..self.children.len() {
                if i as usize != child_loc as usize {
                    self.children[i as usize] =
                        Some(OctreeNode::<T>::new(self.dimension, self.data.unwrap()));
                }
            }
            self.children[child_loc as usize] = Some(node.clone());
            self.leaf = false;
            self.simplified = false;
            self.data = None;
        } else {
            self.children[child_loc as usize] = Some(node.clone());
        }

        self.try_simplify(data);
    }

    /// Simplify the current node if all children have the same value
    fn try_simplify(&mut self, data: T) {
        for child in &self.children {
            if let Some(child_node) = child {
                if let Some(node_data) = child_node.get() {
                    if node_data != data {
                        return;
                    }
                } else {
                    return;
                }
            } else {
                return;
            };
        }

        self.data = Some(data);
        self.children = no_children::<T>();
        self.leaf = true;
        self.simplified = true;
    }

    /// Get data of an `OctreeNode<T>` at a given `NodeLoc`
    pub fn at(&self, loc: &mut NodeLoc) -> Option<T> {
        let child_loc = self.get_child_loc(loc);
        let child = &self.children[child_loc as usize];

        if child.is_none() {
            None
        } else if child.as_ref().unwrap().leaf {
            child.as_ref().unwrap().data
        } else {
            child.as_ref().unwrap().at(loc)
        }
    }

    /// Get a shared reference to a given `OctreeNode<T>`
    pub fn node_as_ref(&self, loc: &mut NodeLoc) -> Option<&OctreeNode<T>> {
        let child_loc = self.get_child_loc(loc);
        let child = &self.children[child_loc as usize];

        if child.is_none() {
            None
        } else if child.as_ref().unwrap().leaf {
            child.as_ref()
        } else {
            child.as_ref().unwrap().node_as_ref(loc)
        }
    }

    pub fn leaf(&self) -> bool {
        self.leaf
    }

    pub fn dimension(&self) -> u16 {
        self.dimension
    }

    /// Get correct insertion location of child node on insertion
    fn get_child_loc(&self, loc: &mut NodeLoc) -> ChildLoc {
        let comparator = self.dimension / 2;

        if loc.z() < comparator {
            if loc.y() < comparator {
                if loc.x() < comparator {
                    ChildLoc::BaseRearLeft
                } else {
                    loc.sub_x(comparator);
                    ChildLoc::BaseRearRight
                }
            } else {
                loc.sub_y(comparator);
                if loc.x() < comparator {
                    ChildLoc::BaseFrontLeft
                } else {
                    loc.sub_x(comparator);
                    ChildLoc::BaseFrontRight
                }
            }
        } else {
            loc.sub_z(comparator);
            if loc.y() < comparator {
                if loc.x() < comparator {
                    ChildLoc::TopRearLeft
                } else {
                    loc.sub_x(comparator);
                    ChildLoc::TopRearRight
                }
            } else {
                loc.sub_y(comparator);
                if loc.x() < comparator {
                    ChildLoc::TopFrontLeft
                } else {
                    loc.sub_x(comparator);
                    ChildLoc::TopFrontRight
                }
            }
        }
    }

    /// Set `OctreeNode<T>` as a leaf node
    fn make_leaf(&mut self, state: bool) {
        self.leaf = state;

        if self.leaf {
            self.children = no_children();
        }
    }
}

/// Helper function that returns an empty `OctreeNode<T>` child vector
fn no_children<T>() -> Vec<Option<OctreeNode<T>>> {
    vec![None, None, None, None, None, None, None, None]
}

#[cfg(test)]
mod tests {
    extern crate core;

    use node::OctreeNode;

    #[test]
    fn test_construct_root() {
        let root_node = OctreeNode::<u8>::construct_root(16);
        assert!(
            (root_node.dimension == 16),
            "Root octree node dimension does not match tree dimension"
        );
        assert!(
            root_node.data.is_none(),
            "Root octree none contains Some(data), should contain None"
        );
        assert!(root_node.leaf, "Root octree node not constructed as a leaf");

        for root_children in root_node.children.iter() {
            assert!(
                root_children.is_none(),
                "Root octree node constructed with Some(child), should be all None"
            );
        }
    }
}
