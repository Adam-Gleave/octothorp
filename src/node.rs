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
    children: Vec<Option<OctreeNode<T>>>,
    data: Option<T>,
}

impl<T> OctreeNode<T>
    where T: Copy
{
    /// Constructs a new `OctreeNode<T>`.
    pub fn new(curr_dimension: u16, data: T) -> OctreeNode<T> {
        OctreeNode::<T> {
            dimension: curr_dimension / 2,
            leaf: true,
            children: no_children::<T>(),
            data: Some(data),
        }
    }

    /// Constructs a root `OctreeNode<T>` to be used in an `Octree<T>` structure
    pub fn construct_root(dimension: u16) -> OctreeNode<T> {
        OctreeNode {
            dimension,
            leaf: true,
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
        let mut node = if self.children[child_loc as usize].is_some() {
            self.children[child_loc as usize].take().unwrap()
        } else {
            OctreeNode::<T>::new(self.dimension, data)
        };

        if self.leaf {
            self.make_leaf(false);
            self.data = None;
        }

        if self.dimension == 2 {
            node.make_leaf(true);
        } else {
            node.insert(loc, data);
        }

        self.children[child_loc as usize] = Some(node);
    }

    /// Get data of an `OctreeNode<T>` at a given `NodeLoc`
    pub fn at(&self, loc: &mut NodeLoc) -> Option<T> {
        let child_loc = self.get_child_loc(loc);
        let child = &self.children[child_loc as usize];

        if child.is_none() {
            None
        } else if child.as_ref().unwrap().leaf {
            child.as_ref().unwrap().data.clone()
        } else {
            child.as_ref().unwrap().at(loc)
        }
    }

    pub fn leaf(&self) -> bool {
        self.leaf
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
    vec![None, None, None, None, None, None, None, None,]
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
        assert!(
            root_node.leaf,
            "Root octree node not constructed as a leaf"
        );

        for root_children in root_node.children.iter() {
            assert!(
                root_children.is_none(),
                "Rooy octree node constructed with Some(child), should be all None"
            );
        }
    }
}
