mod error;
mod node;
pub mod octree;

pub use error::OctreeError;
pub use octree::Octree;

#[cfg(test)]
mod tests {
    extern crate core;

    use self::core::u8;
    use error::OctreeError;
    use octree::Octree;

    #[test]
    fn test_dimension() {
        assert!(
            Octree::<u8>::new(16).is_ok(),
            "Octree with square number dimension returned None"
        );
        assert!(
            Octree::<u8>::new(3).is_err(),
            "Octree with non-square number dimension returned Some()"
        );
    }

    #[test]
    fn test_insert() {
        let mut octree = Octree::<u8>::new(16).unwrap();
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
    }

    #[test]
    fn test_simplify() {
        let mut octree = Octree::<u8>::new(16).unwrap();
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
        octree.insert([0, 0, 0], 255).unwrap();

        if let Some(node) = octree.node_as_ref([0, 0, 0]) {
            assert_eq!(node.dimension(), 2, "Node not simplified");
        } else {
            assert!(false, "Point not found in Octree after inserting");
        }

        octree.insert([0, 0, 0], 128).unwrap();
        assert_eq!(octree.at([0, 0, 0]), Some(128), "Error desimplifying node");
        assert_eq!(octree.at([0, 0, 1]), Some(255), "Error desimplifying node");
    }

    #[test]
    fn test_iter() {
        let mut octree = Octree::<u8>::new(16).unwrap();
        octree.insert([0, 0, 0], 255).unwrap();
        octree.insert([12, 10, 6], 128).unwrap();

        let mut iter = octree.into_iter();
        assert_eq!(iter.nth(0), Some(255), "Value not found in iterator");
        assert_eq!(iter.nth(0), Some(128), "Value not found in iterator");
    }

    #[test]
    fn test_take() {
        let mut octree = Octree::<u8>::new(16).unwrap();
        octree.insert([0, 0, 0], 255).unwrap();
        let val = octree.take([0, 0, 0]);
        assert_eq!(octree.at([0, 0, 0]), None);
        assert_eq!(val, Some(255));
        let none_val = octree.take([0, 0, 0]);
        assert_eq!(none_val, None);
    }

    #[test]
    fn test_insert_none() {
        let mut octree = Octree::<u8>::new(16).unwrap();
        octree.insert([0, 0, 0], 255).unwrap();
        octree.insert_none([0, 0, 0]);
        let val = octree.at([0, 0, 0]);
        assert_eq!(val, None);
    }

    #[test]
    fn test_simplify_none() {
        if let Err(OctreeError::DimensionError) = Octree::<u8>::new(3) {
            println!("Passed!");
        };
        //octree.insert([0, 0, 0], 255).unwrap();
        //octree.insert_none([0, 0, 0]);
        //let val = octree.at([0, 0, 0]);
        //assert_eq!(val, None);
        //println!("{:?}", octree);
    }

    use node::OctreeNode;

    #[test]
    fn test_construct_root() {
        let root_node = OctreeNode::<u8>::construct_root(16);
        assert!(
            (root_node.dimension() == 16),
            "Root octree node dimension does not match tree dimension"
        );
        assert!(
            root_node.get().is_none(),
            "Root octree none contains Some(data), should contain None"
        );
        assert!(
            root_node.leaf(),
            "Root octree node not constructed as a leaf"
        );

        for root_children in root_node.children().iter() {
            assert!(
                root_children.is_none(),
                "Root octree node constructed with Some(child), should be all None"
            );
        }
    }
}
