extern crate core;

use self::core::u8;
use types::NodeLoc;

/// Enumeration representing child location in `OctreeNode<T>::children` field
#[repr(u8)]
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

/// Octree structure
pub struct Octree<T> {
    dimension: u16,
    max_depth: u8,
    root: Box<OctreeNode<T>>,
}

impl<T> Octree<T> {
    /// Constructs a new `Octree<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use octo::octree::Octree;
    ///
    /// let octree = Octree::<u8>::new(16);
    /// ```
    ///
    pub fn new(dimension: u16) -> Option<Octree<T>> {
        let depth = (dimension as f64).sqrt();
        let remainder = depth.fract();
        //TODO: Geometric sequence for verifying dimensions

        if remainder == 0.0 && ((depth as u8) < core::u8::MAX) {
            Some(
                Octree {
                    dimension,
                    max_depth: depth as u8,
                    root: Box::new(OctreeNode::construct_root(dimension)),
                }
            )
        } else {
            None
        }
    }

    /// Constructs a new `Octree<T>`, setting data of `self.root` node
    ///
    /// # Example
    ///
    /// ```
    /// use octo::octree::Octree;
    ///
    /// let octree = Octree::<u8>::new_with_data(16, 255);
    /// ```
    ///
    pub fn new_with_data(dimension: u16, data: T) -> Option<Octree<T>> {
        if let Some(mut octree) = Octree::<T>::new(dimension) {
            octree.set_root_data(data);
            Some(octree)
        } else {
            None
        }
    }

    /// Set the `data` field of a root node, provided it is the only leaf
    ///
    /// # Examples
    ///
    /// ```
    /// use octo::octree::Octree;
    ///
    /// if let Some(mut octree) = Octree::<u8>::new(16) {
    ///     octree.set_root_data(255).unwrap();
    /// }
    /// ```
    ///
    pub fn set_root_data(&mut self, data: T) -> Result<(), String> {
        if self.root.leaf {
            self.root.set(data);
            Ok(())
        } else {
            Err("Error setting root data: root node is not a leaf".to_string())
        }
    }

    /// Insert a new `OctreeNode<T>` into the octree
    /// If this is called on a location where a node already exists, just set the `data` field
    ///
    /// # Examples
    ///
    /// ```
    /// use octo::octree::Octree;
    /// use octo::types::NodeLoc;
    ///
    /// if let Some(mut octree) = Octree::<String>::new(16) {
    ///     octree.insert((NodeLoc::new((0, 0, 0,))),
    ///         "New node created!".to_string())
    ///         .unwrap();
    /// }
    /// ```
    ///
    pub fn insert(&mut self, loc: NodeLoc, data: T) -> Result<(), String> {
        if self.contains_loc(&loc) {
            (*self.root).insert(&loc, data)
        } else {
            Err("Error inserting node: location not bounded by octree".to_string())
        }
    }

    fn contains_loc(&self, loc: &NodeLoc) -> bool {
        loc.x() < self.dimension && loc.y() < self.dimension && loc.z() < self.dimension
    }
}

/// OctreeNode structure (inaccessible outside module)
struct OctreeNode<T> {
    dimension: u16,
    leaf: bool,
    children: Vec<Option<OctreeNode<T>>>,
    data: Option<T>,
}

impl<T> OctreeNode<T> {
    /// Constructs a new `OctreeNode<T>`.
    pub fn new(curr_dimension: u16, data: T) -> Option<OctreeNode<T>> {
        if curr_dimension < 2 {
            None
        } else {
            Some (
                OctreeNode::<T> {
                    dimension: curr_dimension / 2,
                    leaf: true,
                    children: no_children::<T>(),
                    data: Some(data),
                }
            )
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

    /// Algorithm to insert a new `OctreeNode<T>` into the tree
    pub fn insert(&mut self, loc: &NodeLoc, data: T) -> Result<(), String> {
        let child_loc = self.get_child_loc(&loc);
        //TODO
        Ok(())
    }

    /// Get correct insertion location of child node on insertion
    fn get_child_loc(&self, loc: &NodeLoc) -> ChildLoc {
        let comparator = self.dimension / 2;

        if loc.z() < comparator {
            if loc.y() < comparator {
                if loc.x() < comparator {
                    ChildLoc::BaseRearLeft
                } else {
                    ChildLoc::BaseRearRight
                }
            } else {
                if loc.x() < comparator {
                    ChildLoc::BaseFrontLeft
                } else {
                    ChildLoc::BaseFrontRight
                }
            }
        } else {
            if loc.y() < comparator {
                if loc.x() < comparator {
                    ChildLoc::TopRearLeft
                } else {
                    ChildLoc::TopRearRight
                }
            } else {
                if loc.x() < comparator {
                    ChildLoc::TopFrontLeft
                } else {
                    ChildLoc::TopFrontRight
                }
            }
        }
    }
}

fn no_children<T>() -> Vec<Option<OctreeNode<T>>> {
    vec![None, None, None, None, None, None, None, None,]
}

#[cfg(test)]
mod tests {
    extern crate core;

    use self::core::u8;
    use octree::{Octree, OctreeNode};

    #[test]
    fn test_dimension() {
        assert!(
            Octree::<u8>::new(16).is_some(),
            "Octree with square number dimension returned None"
        );
        assert!(
            Octree::<u8>::new(3).is_none(),
            "Octree with non-square number dimension returned Some()"
        );
    }

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
