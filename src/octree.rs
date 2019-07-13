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
    /// # use octo::octree::Octree;
    /// let octree = Octree::<u8>::new(16).unwrap();
    /// ```
    ///
    pub fn new(dimension: u16) -> Option<Octree<T>> {
        let depth = f64::from(dimension).sqrt();
        let remainder = depth.fract();

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

    /// Insert a new `OctreeNode<T>` into the `Octree<T>`
    /// If this is called on a location where a node already exists, just set the `data` field
    ///
    /// # Examples
    ///
    /// ```
    /// # use octo::octree::Octree;
    ///
    /// # let mut octree = Octree::<u8>::new(16).unwrap();
    /// octree.insert([0, 0, 0], 255).unwrap();
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
    /// # use octo::octree::Octree;
    ///
    /// # let mut octree = Octree::<u8>::new(16).unwrap();
    /// octree.insert([0, 0, 0], 255).unwrap();
    /// assert_eq!(octree.at([0, 0, 0]), Some(255));
    /// ```
    ///
    pub fn at(&self, loc: [u16; 3]) -> Option<T> {
        let mut node_loc = self.loc_from_array(loc);
        self.root.at(&mut node_loc)
    }

    /// Get the value stored by the `Octree<T>` at a given node, and replace with `None`
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use octo::octree::Octree;
    /// 
    /// # let mut octree = Octree::<u8>::new(16).unwrap();
    /// octree.insert([0, 0, 0], 255).unwrap();
    /// let val = octree.take([0, 0, 0]);
    /// 
    /// assert_eq!(octree.at([0, 0, 0]), None);
    /// assert_eq!(val, Some(255));
    /// ```
    pub fn take(&mut self, loc: [u16; 3]) -> Option<T> {
        let mut node_loc = self.loc_from_array(loc);
        self.root.take(&mut node_loc)
    }

    /// Insert `None` into the `Octree<T>` at a given node
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use octo::octree::Octree;
    /// 
    /// # let mut octree = Octree::<u8>::new(16).unwrap();
    /// octree.insert([0, 0, 0], 255).unwrap();
    /// octree.insert_none([0, 0, 0]);
    /// 
    /// assert_eq!(octree.at([0, 0, 0]), None);
    /// ```
    /// 
    pub fn insert_none(&mut self, loc: [u16; 3]) {
        let mut node_loc = self.loc_from_array(loc);
        self.root.insert_none(&mut node_loc);
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
    /// # use octo::octree::Octree;
    ///
    /// # let mut octree = Octree::<u8>::new(16).unwrap();
    /// octree.insert([0, 0, 0], 255).unwrap();
    /// octree.insert([12, 10, 6], 128).unwrap();
    ///
    /// // This will print "255, 128"
    /// for val in octree.iter() {
    ///     print!("{:?}", val);
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
    /// # use octo::octree::Octree;
    ///
    /// # let mut octree = Octree::<u8>::new(16).unwrap();
    /// octree.insert([0, 0, 0], 255).unwrap();
    /// let mut iter = octree.into_iter();
    /// 
    /// assert_eq!(iter.nth(0), Some(255));
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

/// Debug printing
impl<T> fmt::Debug for Octree<T>
where
    T: Copy + PartialEq + fmt::Debug,
{
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        println!("{:?}", self.root);
        Ok(())
    }
}
