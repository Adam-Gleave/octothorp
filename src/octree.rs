extern crate core;

use self::core::u8;
use types::NodeLoc;
use node::OctreeNode;

/// Octree structure
pub struct Octree<T> {
    dimension: u16,
    max_depth: u8,
    root: Box<OctreeNode<T>>,
}

impl<T> Octree<T>
    where T: Copy
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
            match octree.set_root_data(data) {
                Err(_) => None,
                _ => Some(octree)
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

    /// Insert a new `OctreeNode<T>` into the octree
    /// If this is called on a location where a node already exists, just set the `data` field
    ///
    /// # Examples
    ///
    /// ```
    /// use octo::octree::Octree;
    /// use octo::types::NodeLoc;
    ///
    /// if let Some(mut octree) = Octree::<u8>::new(16) {
    ///     let mut loc = NodeLoc::new((0, 0, 0,));
    ///     octree.insert(&mut loc, 255).unwrap();
    /// }
    /// ```
    ///
    pub fn insert(&mut self, loc: &mut NodeLoc, data: T) -> Result<(), String> {
        if self.contains_loc(loc) {
            (*self.root).insert(loc, data);
            Ok(())
        } else {
            Err("Error inserting node: location not bounded by octree".to_string())
        }
    }

    /// Test whether the `Octree<T>` contains a given `NodeLoc`
    fn contains_loc(&self, loc: &NodeLoc) -> bool {
        loc.x() < self.dimension && loc.y() < self.dimension && loc.z() < self.dimension
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
}
