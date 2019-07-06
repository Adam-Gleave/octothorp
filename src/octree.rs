extern crate core;

use self::core::u8;
use node::OctreeNode;
use std::fmt;
use types::NodeLoc;

/// Octree structure
pub struct Octree<T> {
    dimension: u16,
    max_depth: u8,
    root: Box<OctreeNode<T>>,
}

impl<T> Octree<T>
where
    T: Copy + PartialEq,
{
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
        let depth = f64::from(dimension).sqrt();
        let remainder = depth.fract();
        //TODO: Geometric sequence for verifying dimensions

        if remainder == 0.0 && ((depth as u8) < core::u8::MAX) {
            Some(Octree {
                dimension,
                max_depth: depth as u8,
                root: Box::new(OctreeNode::construct_root(dimension)),
            })
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
            match octree.set_root_data(data) {
                Err(_) => None,
                _ => Some(octree),
            }
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
        if self.root.leaf() {
            self.root.set(data)?;
            Ok(())
        } else {
            Err("Error setting root data: root node is not a leaf".to_string())
        }
    }

    /// Insert a new `OctreeNode<T>` into the `Octree<T>`
    /// If this is called on a location where a node already exists, just set the `data` field
    ///
    /// # Examples
    ///
    /// ```
    /// use octo::octree::Octree;
    ///
    /// if let Some(mut octree) = Octree::<u8>::new(16) {
    ///     octree.insert([0, 0, 0], 255).unwrap();
    /// }
    /// ```
    ///
    pub fn insert(&mut self, loc: [u16; 3], data: T) -> Result<(), String> {
        let mut node_loc = self.loc_from_array(loc);
        if self.contains_loc(&node_loc) {
            (*self.root).insert(&mut node_loc, data);
            Ok(())
        } else {
            Err("Error inserting node: location not bounded by octree".to_string())
        }
    }

    /// Get the value stored by the `Octree<T>` at a given node
    ///
    /// # Examples
    ///
    /// ```
    /// use octo::octree::Octree;
    /// use octo::types::NodeLoc;
    ///
    /// if let Some(mut octree) = Octree::<u8>::new(16) {
    ///     octree.insert([0, 0, 0], 255).unwrap();
    ///     assert_eq!(octree.at([0, 0, 0]), Some(255));
    /// }
    /// ```
    ///
    pub fn at(&self, loc: [u16; 3]) -> Option<T> {
        let mut node_loc = self.loc_from_array(loc);
        self.root.at(&mut node_loc)
    }

    /// Returns the x/y/z dimension of an `Octree<T>`
    pub fn dimension(&self) -> u16 {
        self.dimension
    }

    /// Returns the maximum depth of an `Octree<T>`
    pub fn max_depth(&self) -> u8 {
        self.max_depth
    }

    /// Get a shared reference to a given `OctreeNode<T>`
    pub fn node_as_ref(&self, loc: [u16; 3]) -> Option<&OctreeNode<T>> {
        let mut node_loc = self.loc_from_array(loc);
        self.root.node_as_ref(&mut node_loc)
    }

    /// Transform the `Octree<T>` into an iterator, consuming the `Octree<T>`
    ///
    /// # Examples
    ///
    /// ```
    /// use octo::octree::Octree;
    ///
    /// if let Some(mut octree) = Octree::<u8>::new(16) {
    ///     let mut loc1 = [0, 0, 0];
    ///     let mut loc2 = [12, 10, 6];
    ///     octree.insert([0, 0, 0], 255).unwrap();
    ///     octree.insert([12, 10, 6], 128).unwrap();
    ///
    ///     // This will print "255, 128"
    ///     for val in octree.iter() {
    ///         print!("{:?}", val);
    ///     }
    /// }
    /// ```
    ///
    pub fn iter(&mut self) -> OctreeIterator<T> {
        OctreeIterator::new_from_ref(&self)
    }

    /// Create a NodeLoc from a 3-index co-ordinate array
    fn loc_from_array(&self, array: [u16; 3]) -> NodeLoc {
        NodeLoc::new((array[0], array[1], array[2]))
    }

    /// Test if the `Octree<T>` bounds the given `NodeLoc`
    fn contains_loc(&self, loc: &NodeLoc) -> bool {
        loc.x() < self.dimension && loc.y() < self.dimension && loc.z() < self.dimension
    }
}

/// Struct providing iterator functionality for `Octree<T>`
pub struct OctreeIterator<T> {
    node_stack: Vec<OctreeNode<T>>,
    value_stack: Vec<T>,
}

impl<T> IntoIterator for Octree<T>
where
    T: Copy + PartialEq,
{
    type Item = T;
    type IntoIter = OctreeIterator<T>;

    /// Transform the `Octree<T>` into an iterator, consuming the `Octree<T>`
    ///
    /// # Examples
    ///
    /// ```
    /// use octo::octree::Octree;
    ///
    /// if let Some(mut octree) = Octree::<u8>::new(16) {
    ///     octree.insert([0, 0, 0], 255).unwrap();
    ///
    ///     let mut iter = octree.into_iter();
    ///     assert_eq!(iter.nth(0), Some(255));
    /// }
    /// ```
    ///
    fn into_iter(self) -> OctreeIterator<T> {
        OctreeIterator::new(self)
    }
}

impl<T> OctreeIterator<T>
where
    T: Copy + PartialEq,
{
    /// Create a new `OctreeIterator<T>` from an `Octree<T>`, consuming it in the process
    fn new(octree: Octree<T>) -> OctreeIterator<T> {
        let mut iter = OctreeIterator {
            node_stack: vec![],
            value_stack: vec![],
        };
        iter.node_stack.push(*(octree.root.clone()));
        iter.dfs();
        iter
    }

    /// Create a new `OctreeIterator<T>` from an `Octree<T>`, without consuming it
    fn new_from_ref(octree: &Octree<T>) -> OctreeIterator<T> {
        let mut iter = OctreeIterator {
            node_stack: vec![],
            value_stack: vec![],
        };
        iter.node_stack.push(*(octree.root.clone()));
        iter.dfs();
        iter
    }

    /// Iterator implementation using depth-first search
    fn dfs(&mut self) {
        while !self.node_stack.is_empty() {
            if let Some(curr_node) = self.node_stack.pop() {
                if let Some(data) = curr_node.get() {
                    self.value_stack.push(data);
                };
                for child in curr_node.children() {
                    if let Some(child_node) = child {
                        self.node_stack.push(child_node);
                    };
                }
            };
        }
    }
}

impl<T> Iterator for OctreeIterator<T>
where
    T: Copy,
{
    type Item = T;

    /// Essential `Iterator` implementation
    fn next(&mut self) -> Option<T> {
        self.value_stack.pop()
    }
}

/// Debug printing, including node locations
/// This is currently wildly unoptimised!
impl<T> fmt::Debug for Octree<T>
where
    T: Copy + PartialEq + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Octree nodes:")?;
        for x in 0..self.dimension {
            for y in 0..self.dimension {
                for z in 0..self.dimension {
                    let mut loc = NodeLoc::new((x, y, z));
                    if let Some(val) = self.root.at(&mut loc) {
                        writeln!(f, "({}, {}, {}): {:?}", x, y, z, val)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate core;

    use self::core::u8;
    use octree::Octree;

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
    fn test_insert() {
        if let Some(mut octree) = Octree::<u8>::new(16) {
            let loc1 = [0, 0, 0];
            let loc2 = [0, 0, 1];
            let loc3 = [12, 10, 6];
            octree.insert(loc1, 255).unwrap();
            octree.insert(loc2, 255).unwrap();
            octree.insert(loc3, 128).unwrap();
            assert!(
                octree.at(loc1).is_some(),
                "Point not found in Octree after inserting"
            );
            assert!(
                octree.at(loc2).is_some(),
                "Point not found in Octree after inserting"
            );
            assert!(
                octree.at(loc3).is_some(),
                "Point not found in Octree after inserting"
            );
        } else {
            assert!(false, "Error initialising Octree");
        };
    }

    #[test]
    fn test_simplify() {
        if let Some(mut octree) = Octree::<u8>::new(16) {
            octree.insert([0, 0, 0], 255).unwrap();
            octree.insert([0, 0, 1], 255).unwrap();
            octree.insert([0, 1, 0], 255).unwrap();
            octree.insert([0, 1, 1], 255).unwrap();
            octree.insert([1, 0, 0], 255).unwrap();
            octree.insert([1, 0, 1], 255).unwrap();
            octree.insert([1, 1, 0], 255).unwrap();

            if let Some(node) = octree.node_as_ref([0, 0, 0]) {
                assert_eq!(node.dimension(), 1, "Node erroneously simplified");
            } else {
                assert!(false, "Point not found in Octree after inserting");
            }

            octree.insert([1, 1, 1], 255).unwrap();

            if let Some(node) = octree.node_as_ref([0, 0, 0]) {
                assert_eq!(node.dimension(), 2, "Node not simplified");
            } else {
                assert!(false, "Point not found in Octree after inserting");
            }
        } else {
            assert!(false, "Error initialising Octree");
        }
    }

    #[test]
    fn test_iter() {
        if let Some(mut octree) = Octree::<u8>::new(16) {
            octree.insert([0, 0, 0], 255).unwrap();
            octree.insert([12, 10, 6], 128).unwrap();

            let mut iter = octree.into_iter();
            assert_eq!(iter.nth(0), Some(255), "Value not found in iterator");
            assert_eq!(iter.nth(0), Some(128), "Value not found in iterator");
        } else {
            assert!(false, "Error initialising Octree");
        };
    }
}
