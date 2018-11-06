extern crate core;

use self::core::u8;
use types::NodeLoc;

/// Octree structure
pub struct Octree<T> {
    dimension: u32,
    max_depth: u8,
    children: Vec<Option<OctreeNode<T>>>,
}

impl<T> Octree<T> {
    /// Constructs a new `Octree<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use octree::Octree;
    ///
    /// let octree = Octree::<u8>::new(16);
    /// ```
    ///
    pub fn new(dimension: u32) -> Option<Octree<T>> {
        let max_depth = (dimension as f64).sqrt();
        let remainder = max_depth.fract();
        println!("{0}", remainder);

        if remainder == 0.0 && ((max_depth as u8) < core::u8::MAX) {
            Some(
                Octree {
                    dimension,
                    children: no_children(),
                    max_depth: max_depth as u8
                }
            )
        } else {
            None
        }
    }

    pub fn insert(&mut self, loc: NodeLoc, data: T) -> Result<(), String> {
        if self.contains_loc(loc) {
            return Ok(());
        }
        Err("Error inserting node: location not bounded by octree!".to_string())
    }

    fn contains_loc(&self, loc: NodeLoc) -> bool {
        loc.x() < self.dimension && loc.y() < self.dimension && loc.z() < self.dimension
    }
}

/// OctreeNode structure
pub struct OctreeNode<T> {
    depth: u8,
    leaf: bool,
    children: Vec<Option<OctreeNode<T>>>,
    data: T,
}

impl<T> OctreeNode<T> {
    /// Constructs a new `OctreeNode<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use octree::OctreeNode;
    ///
    /// let node = OctreeNode::<u8>::new(0, 255);
    /// ```
    ///
    fn new(depth: u8, data: T) -> Option<OctreeNode<T>> {
        match depth {
            core::u8::MAX => None,
            _ => Some (
                OctreeNode::<T> {
                    depth: depth + 1,
                    leaf: true,
                    children: no_children::<T>(),
                    data,
                }
            ),
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
    fn test_max_depth() {
        assert!(
            OctreeNode::<u8>::new(u8::max_value() - 1, 0).is_some(),
            "Octree node with valid depth returned None"
        );
        assert!(
            OctreeNode::<u8>::new(u8::max_value(), 0).is_none(),
            "Octree node with above max depth returned Some()"
        )
    }
}
