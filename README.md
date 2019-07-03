# Octothorp

Octothorp is a sparse octree library for rust. It can be used to manage any data, provided it is `Copy` (this limitation may be removed in the future).

## Example:
```rust
let octree = Octree::<u8>::new(16).unwrap();
let mut loc = NodeLoc::new((0, 0, 0,));

octree.insert(&mut loc, 255).unwrap();
assert_eq!(octree.at(&mut loc), 255);


