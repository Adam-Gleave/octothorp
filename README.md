# Octothorp

Octothorp is a **_work in progress_** sparse octree library for rust. It can be used to manage any data, provided it is `Copy` and `PartialEq` (these limitations may be removed in the future).

## Example:
```rust
let octree = Octree::<u8>::new(16).unwrap();
octree.insert([0, 0, 0], 255).unwrap();

assert_eq!(octree.at([0, 0, 0], 255));
```
